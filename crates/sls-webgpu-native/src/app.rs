use std::{
  collections::HashMap,
  ops::DerefMut,
  sync::{Arc, RwLock, Weak},
  thread::spawn,
  time::*,
};

use crossbeam::channel::{bounded, unbounded, Receiver, Sender};
use log::error;
use rayon::{ThreadPool, ThreadPoolBuilder};
use sdl2::{
  event::{Event, WindowEvent},
  keyboard::Keycode,
  video::Window,
  EventPump,
};

use sls_webgpu::{
  anyhow::{self, anyhow},
  game::{
    asset_loading::{
      asset_load_message::{AssetLoadedMessagePayload, AssetLoadedMessagePayload::GltfModel},
      resources::MainSceneAssets,
      MultithreadedAssetLoaderQueue,
    },
    input::InputResource,
    resources::ScreenResolution,
    CreateGameParams, GameState,
  },
  gltf,
  gltf::{buffer::Data, json::Value, Document, Error},
  image, imgui, imgui_wgpu,
  platform::{gui, gui::DrawUi, sdl2_backend::ImguiSdlPlatform},
  renderer_common::{
    allocator::ResourceManager,
    gltf_loader::GltfImportOutput,
    handle::{Handle, HandleIndex},
  },
  wgpu_renderer::{
    material::Material,
    mesh::{Mesh, MeshGeometry},
    model::{Model, StreamingMesh},
    textures::{BindTexture, TextureResource},
  },
  Context,
};

pub struct App {
  pub(crate) context: Arc<RwLock<Context>>,
  pub(crate) event_pump: EventPump,
  pub(crate) game_state: GameState,
  pub(crate) imgui_context: Arc<RwLock<imgui::Context>>,
  pub(crate) imgui_renderer: Arc<RwLock<imgui_wgpu::Renderer>>,
  pub(crate) imgui_platform: Arc<RwLock<ImguiSdlPlatform>>,
  pub(crate) sdl: sdl2::Sdl,
  worker_pool: rayon::ThreadPool,
  assets_loaded_receiver: Receiver<anyhow::Result<AssetLoadedMessagePayload>>,
  assets_loaded_sender: Sender<anyhow::Result<AssetLoadedMessagePayload>>,
  pub window: Window,
  pub avocado_model_data: Option<GltfImportOutput>,
  models: Weak<RwLock<ResourceManager<StreamingMesh>>>,
}

impl App {
  pub fn new() -> anyhow::Result<Self> {
    let (tx, avo_model_rx) = bounded::<anyhow::Result<GltfImportOutput>>(1);
    // load model in a separate thread
    rayon::spawn(move || {
      let avocato_model = gltf::import("./assets/BoomBox.glb")
        .map_err(|e| anyhow::Error::from(e))
        .map(|model| GltfImportOutput::new(model.0, model.1, model.2));
      if let Err(e) = tx.send(avocato_model) {
        panic!("could not send model data to main thread! {:?}", e)
      }
    });

    let sdl = sdl2::init().map_err(|s| anyhow!(s))?;
    let video_sys = sdl.video().map_err(|s| anyhow!(s))?;
    let mut window = create_window(&video_sys, (1600, 1200))?;
    let event_pump = sdl.event_pump().map_err(|s| anyhow!(s))?;
    let context = pollster::block_on(Context::new(&mut window).build())?;

    let models = Arc::downgrade(&context.resources.models);

    let mut imgui_context = gui::create_imgui(gui::Options {
      hidpi_factor: 2.0,
      font_size: 20.0,
    });
    let imgui_platform = ImguiSdlPlatform::new(&mut imgui_context)?;

    let texture_format = context
      .surface
      .get_preferred_format(&context.adapter)
      .ok_or(anyhow!("no surface texture format available"))?;
    let renderer_options = imgui_wgpu::RendererConfig {
      texture_format,
      ..imgui_wgpu::RendererConfig::new_srgb()
    };

    let imgui_renderer = Arc::new(RwLock::new(imgui_wgpu::Renderer::new(
      &mut imgui_context,
      &context.device,
      &context.queue,
      renderer_options,
    )));

    let imgui_platform = Arc::new(RwLock::new(imgui_platform));
    let context = Arc::new(RwLock::new(context));

    let mut game_state = GameState::new(CreateGameParams {
      asset_loader_queue: Some(Box::new(
        sls_webgpu::game::asset_loading::MultithreadedAssetLoaderQueue::new(),
      )),
    });
    {
      game_state.wgpu_setup(context.clone());
    }
    let worker_pool = rayon::ThreadPoolBuilder::new().build()?;
    let (s, r) = unbounded();
    let avocado_model_data = match avo_model_rx
      .recv()
      .expect("could not receive data from channel")
    {
      Ok(m) => {
        let model = Some(m);
        model
      }
      Err(e) => {
        log::error!("could not load avocado model: {:?}", e);
        None
      }
    };
    let app = Self {
      imgui_context: Arc::new(RwLock::new(imgui_context)),
      context,
      imgui_renderer,
      game_state,
      event_pump,
      imgui_platform,
      sdl,
      worker_pool,
      window,
      assets_loaded_receiver: r,
      assets_loaded_sender: s,
      models,
      avocado_model_data,
    };
    Ok(app)
  }

  pub(crate) fn run(mut self) {
    self.game_state.set_is_running(true);
    if let Err(e) = self.load_assets() {
      panic!("fatal error loading assets! {:?}", e);
    }

    let mut previous_time = Instant::now();
    let mut update_lag = Duration::from_nanos(0);
    let ms_per_update = Duration::from_millis(1000 / 60);
    let imgui_context = self.imgui_context.clone(); // take ownership from the App object
    {
      match (self.imgui_platform.write(), imgui_context.write()) {
        (Ok(mut platform), Ok(mut context)) => platform.on_start(context.io_mut(), &self.window),
        (a, b) => log::error!("write lock poisoned! {:?}, {:?}", a.err(), b.err()),
      };
    }
    self.game_state.on_start();

    while self.game_state.is_running() {
      let current_time = Instant::now();
      let elapsed_time = current_time - previous_time;
      self.game_state.new_frame();
      previous_time = current_time;
      update_lag += elapsed_time;
      {
        let mut imgui_context = imgui_context.write().expect("imgui context lock poisoned");
        self.handle_input(imgui_context.deref_mut());
      }
      if !self.game_state.is_running() {
        break;
      }
      // check for asset loaded messages, and handle the results
      self.check_assets_loaded();

      // per frame update
      self.game_state.update(&elapsed_time);

      // fixed-dt update (for physics and stuff)
      while update_lag >= ms_per_update {
        self.game_state.fixed_update(&ms_per_update);
        update_lag -= ms_per_update;
      }
      {
        self
          .context
          .write()
          .expect("could not write to context")
          .update();
      }
      if let Err(e) = self.on_render() {
        panic!("render error! {:?}", e);
      }
    }
  }
  fn on_render(&mut self) -> Result<(), sls_webgpu::Error> {
    use sls_webgpu::Error;
    let platform_arc = self.imgui_platform.clone();
    let context_arc = self.imgui_context.clone();

    let mut im_ctx = context_arc
      .write()
      .map_err(|e| Error::from_other(format!("lock is poisoned! {:?}", e)))?;

    {
      let mut im_platform = platform_arc
        .write()
        .map_err(|e| Error::from_other(format!("lock is poisoned! {:?}", e)))?;

      im_platform.prepare_frame(
        im_ctx.io_mut(),
        &self.window,
        &self.event_pump.mouse_state(),
      );
    }
    let mut ui = im_ctx.frame();
    self.game_state.draw_ui(&mut ui);

    // let mut gui_renderer_arc = self
    //   .imgui_renderer
    //   .write()
    //   .map_err(|e| Error::from_other(format!("lock is poisoned! {:?}", e)))?;

    self
      .context
      .write()
      .expect("Deadlock on render context")
      .render(&mut self.game_state)
      .map_err(|e| sls_webgpu::Error::FromError(e.into()))
  }

  pub(crate) fn handle_input(&mut self, imgui_context: &mut imgui::Context) {
    let imgui_platform = self.imgui_platform.clone();
    let mut imgui_lock = imgui_platform
      .write()
      .unwrap_or_else(|e| panic!("imgui rwlock is poisoned!: {:?}", e));
    {
      let mut game_input = self
        .game_state
        .resources()
        .get_mut::<InputResource>()
        .expect("game input is not available to write");
      game_input.backend.on_start_frame();
    }
    for event in self.event_pump.poll_iter() {
      {
        self
          .game_state
          .resources()
          .get_mut::<InputResource>()
          .expect("game input is not available to write")
          .backend
          .handle_sdl_event(&event, &self.window)
      }
      if !imgui_lock.ignore_event(&event) {
        imgui_lock.handle_event(imgui_context, &event);
      } else {
        log::debug!("ignoring event {:?} for imgui", event)
      }
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => self.game_state.set_is_running(false),
        Event::Window {
          win_event: WindowEvent::Resized(width, height),
          ..
        } => {
          let window_size = (width as usize, height as usize);
          let mut context = self.context.write().expect("deadlock on render context");
          let drawable_size = self.window.drawable_size();
          context.on_resize((width as u32, height as u32));
          self.game_state.on_resize(
            (drawable_size.0 as usize, drawable_size.1 as usize),
            window_size,
          );
        }
        _ => {}
      }
    }
    if let Err(err) = self.sync_input_state() {
      error!("error synching input state: {:?}", &err);
    }
  }

  pub(crate) fn sync_input_state(&mut self) -> Result<(), String> {
    let mut input_res = self
      .game_state
      .resources()
      .get_mut::<InputResource>()
      .ok_or("Could not get input resource as writable")?;
    let sdl2_input = &mut input_res.backend;
    sdl2_input.sync_input(&self.sdl, &self.event_pump);
    Ok(())
  }

  fn load_assets(&mut self) -> Result<(), anyhow::Error> {
    // load screen resolution and render information into game state
    let window_size = self.window.size();
    let drawable_size = self.window.drawable_size();
    {
      let mut resources = self.game_state.resources_mut();
      resources.insert(ScreenResolution {
        window_size: (window_size.0 as _, window_size.1 as _),
        drawable_size: (drawable_size.0 as _, drawable_size.1 as _),
      });
    }

    match &self.avocado_model_data {
      Some(sample_model) => {
        let mut ctx = self.context.write().unwrap();
        let mut mesh = StreamingMesh::new("assets/BoomBox.glb".to_owned());
        let mesh_path = mesh.path().to_owned();
        mesh.load_from_gltf(
          ctx.deref_mut(),
          &sample_model.document,
          &sample_model.buffers,
          &sample_model.images,
        )?;

        let models_arc = self.models.upgrade().map(|ptr| ptr.clone()).unwrap();
        let mut meshes = models_arc.write().expect("cannot access meshes");
        let avocado_model = meshes.insert(mesh);
        ctx.models_to_draw.push(avocado_model);
        let assets = MainSceneAssets {
          avocado_model,
          avocado_model_path: mesh_path,
        };
        let mut resources = self.game_state.resources_mut();
        resources.insert(assets);
      }
      None => anyhow::bail!("sample model has not been loaded"),
    };

    Ok(())
  }

  fn check_assets_loaded(&mut self) {
    let results: Vec<_> = self.assets_loaded_receiver.try_iter().collect();
    for r in results {
      match r {
        Ok(message) => {
          println!("loaded asset: {:?}", message);
          self.on_model_loaded(message);
        }
        Err(e) => {
          log::error!("error loading asset: {:?}", e)
        }
      }
    }
  }

  fn on_model_loaded(&mut self, message: AssetLoadedMessagePayload) {}
}

fn create_window(
  video_sys: &sdl2::VideoSubsystem,
  window_size: (u32, u32),
) -> Result<Window, anyhow::Error> {
  let window = video_sys
    .window("Webgpu demo!", window_size.0, window_size.1)
    .resizable()
    .position_centered()
    .allow_highdpi()
    .build()?;
  Ok(window)
}
