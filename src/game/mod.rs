use std::{fmt, time::Duration};

use legion::*;

#[cfg(feature = "wgpu_imgui")]
pub use wgpu_imgui::*;

use crate::game::{
  components::{DebugShowScene, GameLoopTimer},
  input::InputResource,
  resources::{Scene, UIDataIn, UIDataOut},
  systems::*,
};

pub mod components;
pub mod input;
pub mod resources;
pub mod systems;

pub mod asset_loading;

#[cfg(feature = "html5_backend")]
pub mod html5_backend;
#[cfg(feature = "html5_backend")]
pub mod wasm_bindings;

pub struct GameState {
  world: World,
  /// legion schedule triggered every fixed time update
  fixed_schedule: Schedule,
  /// legion schedule triggered every frame
  per_frame_schedule: Schedule,

  /// legion schedule triggered when the window size changes
  on_resize_schedule: Schedule,

  /// the world's resource store
  resources: Resources,

  /// if set to false, end the game loop
  is_running: bool,
  registry: Registry<String>,
}

impl fmt::Debug for GameState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("GameState")
      .field("world", &self.world)
      .field("is_running", &self.is_running)
      .finish()
  }
}

#[derive(Default)]
pub struct CreateGameParams {
  pub asset_loader_queue: Option<Box<dyn AssetLoaderQueue>>,
}

impl GameState {
  pub fn new(options: CreateGameParams) -> Self {
    let world = World::default();
    let is_running = false;
    let fixed_schedule = Schedule::builder()
      .add_system(systems::fixed_update_logging_system())
      .add_system(systems::write_camera_ui_data_system())
      .add_system(systems::model_systems::rotate_models_system(0.0))
      .build();
    let per_frame_schedule = Schedule::builder()
      .add_system(systems::per_frame_logging_system())
      .add_thread_local(systems::camera_move_system())
      .add_system(systems::write_renderable_ui_data_system())
      .build();

    let on_resize_schedule = Schedule::builder()
      .add_system(camera_systems::camera_on_resize_system())
      .build();

    let mut resources = Self::initial_resources();
    resources.insert(InputResource {
      backend: InputState::default(),
    });
    if let Some(asset_loader_queue) = options.asset_loader_queue {
      resources.insert(asset_loader_queue)
    }
    let state = Self {
      world,
      fixed_schedule,
      per_frame_schedule,
      on_resize_schedule,
      resources,
      is_running,
      registry: Self::make_world_registry(),
    };
    state
  }

  pub fn new_frame(&mut self) {
    let context = self
      .resources
      .get::<Arc<RwLock<Context>>>()
      .map(|x| x.clone());
    match &context {
      Some(context) => {
        let frame = WgpuFrame::new(&*context);
        self.resources.insert(frame);
      }
      None => {
        log::warn!("context resource is not inserted");
      }
    }
  }

  fn make_world_registry() -> Registry<String> {
    let mut registry = Registry::<String>::default();
    registry.register::<LightSource>("light_source".to_owned());
    registry.register::<Transform3D>("transform_3D".to_owned());
    registry.register::<RenderModel>("render_model".to_owned());
    registry.register::<Camera>("camera".to_owned());
    registry
  }

  pub fn as_json(&self) -> Result<serde_json::Value, serde_json::Error> {
    let entity_serializer = Canon::default();
    let ser_world = self
      .world
      .as_serializable(legion::any(), &self.registry, &entity_serializer);

    serde_json::to_value(ser_world)
  }

  fn initial_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(GameLoopTimer::default());
    resources.insert(DebugShowScene(false));
    resources.insert(Scene { main_camera: None });
    resources.insert(UIDataIn::default());
    resources.insert(UIDataOut::default());

    resources.insert(MeshLookup::default());
    #[cfg(not(target_arch = "wasm32"))]
      {
        resources.insert(Box::new(MultithreadedAssetLoaderQueue::new()) as Box<dyn AssetLoaderQueue>)
      }
    resources
  }

  pub fn add_wgpu_resources(&mut self, context: &Arc<RwLock<Context>>) {
    self.resources.insert(context.clone());
    let context = context.read().unwrap();
    // insert resource manager smart pointers as legion resources
    self.resources.insert(context.resources.clone());
    self.resources.insert(context.resources.models.clone());
    self.resources.insert(context.resources.meshes.clone());
    self.resources.insert(context.resources.textures.clone());
    self.resources.insert(context.resources.render_pipelines.clone());
    self.resources.insert(context.resources.shaders.clone());
    // self.resources.insert(frame);
  }

  /// Lifecycle functions
  /// on_start, update, fixed_update, resize, etc

  pub fn on_start(&mut self) {
    let mut builder = Schedule::builder();

    builder.add_thread_local(setup_scene_system());
    #[cfg(feature = "wgpu_renderer")]
      {
        if self
          .resources
          .get::<Arc<RwLock<crate::wgpu_renderer::context::Context>>>()
          .is_some()
        {
          builder.add_thread_local(systems::model_systems::create_models_wgpu_system());
        }
      }
    let mut scheduler = builder.build();
    scheduler.execute(&mut self.world, &mut self.resources);
    self.world.sy
  }

  pub fn update(&mut self, dt: &Duration) {
    {
      let mut loop_timer = self
        .resources
        .get_mut::<GameLoopTimer>()
        .unwrap_or_else(|| panic!("game loop missing! "));
      loop_timer.per_frame_dt = *dt;
    }
    self
      .per_frame_schedule
      .execute(&mut self.world, &mut self.resources);
  }
  pub fn fixed_update(&mut self, dt: &Duration) {
    self.poll_task_completions();

    {
      let mut loop_timer = self
        .resources
        .get_mut::<GameLoopTimer>()
        .unwrap_or_else(|| panic!("game loop missing! "));
      loop_timer.fixed_dt = *dt;
    }
    self
      .fixed_schedule
      .execute(&mut self.world, &mut self.resources);
  }

  pub fn on_resize(&mut self, drawable_size: (usize, usize), window_size: (usize, usize)) {
    let resize = resources::ScreenResolution {
      drawable_size,
      window_size,
    };
    {
      let mut screen_resolution = self.resources.get_mut_or_insert(resize.clone());
      screen_resolution.drawable_size = resize.drawable_size;
      screen_resolution.window_size = resize.window_size;
    }
    self
      .on_resize_schedule
      .execute(&mut self.world, &mut self.resources);
  }

  /// accessors and setters

  pub fn is_running(&self) -> bool {
    self.is_running
  }

  pub fn set_is_running(&mut self, value: bool) {
    self.is_running = value;
  }

  #[inline]
  pub fn resources(&self) -> &Resources {
    &self.resources
  }
  #[inline]
  pub fn resources_mut(&mut self) -> &mut Resources {
    &mut self.resources
  }

  pub fn world(&self) -> &World {
    &self.world
  }
  pub fn world_mut(&mut self) -> &mut World {
    &mut self.world
  }
  pub fn fixed_schedule(&self) -> &Schedule {
    &self.fixed_schedule
  }
  pub fn per_frame_schedule(&self) -> &Schedule {
    &self.per_frame_schedule
  }

  pub fn map_input_backend<R, F: FnOnce(&InputState) -> R>(
    &self,
    callback: F,
  ) -> Result<R, String> {
    let resource = self
      .resources
      .get::<InputResource>()
      .ok_or("input resource is not loaded")?;
    let backend = &resource.backend;
    Ok(callback(backend))
  }
  pub fn map_input_backend_mut<R, F: FnOnce(&mut InputState) -> R>(
    &mut self,
    callback: F,
  ) -> Result<R, String> {
    let mut resource = self
      .resources
      .get_mut::<InputResource>()
      .ok_or("input resource is not loaded")?;
    let backend = &mut resource.backend;
    Ok(callback(backend))
  }
  fn poll_task_completions(&self) {
    #[cfg(not(target_arch = "wasm32"))]
      {
        let ctx = self.resources.get_mut::<crate::wgpu_renderer::Context>();
        let loader = self.resources.get_mut::<MultithreadedAssetLoaderQueue>();
        match (ctx, loader) {
          (Some(_ctx), Some(_loader)) => {}
          (_ctx, _loader) => {
            // log::warn!("missing resources needed to load assets: {:?}, {:?}", ctx.is_some(), loader.is_some())
          }
        }
      }
  }
}

#[cfg(feature = "wgpu_imgui")]
mod wgpu_imgui {
  use imgui::*;

  use crate::platform::gui::wgpu_imgui::DrawUi;

  use super::*;

  impl DrawUi for GameState {
    fn draw_ui(&self, ui: &mut Ui) {
      use imgui::*;

      let main_camera_data = self.resources.get::<UIDataIn>();

      Window::new(im_str!("Window!!"))
        .size([600.0, 300.0], Condition::Appearing)
        .position([0.0, 0.0], Condition::Appearing)
        .build(ui, || {
          ui.text(im_str!("Hellooo!"));
          if let Some(loop_timer) = self.resources.get::<GameLoopTimer>() {
            ui.text(format!("Fixed DT: {}s", loop_timer.fixed_dt.as_secs_f64()));
            ui.text(format!(
              "Frame DT: {}s",
              loop_timer.per_frame_dt.as_secs_f64()
            ));
          }
          if let Some(ui_data) = main_camera_data {
            ui.text(format!("camera position {:?}", ui_data.camera.position));
            ui.text(format!("camera front vector {:?}", ui_data.camera.forward));

            ui.group(|| {
              ui.text(format!("mouse pos {:?}", ui_data.mouse_pos));
              ui.text(format!("mouse delta: {:?}", ui_data.mouse_delta))
            });

            ui.group(|| {
              for (model, xform) in &ui_data.drawable_meshes {
                ui.text(format!("model: {:?}, {:?}", model, xform.position));
              }
            });
          }
        });
    }
  }
}

use crate::{
  game::{asset_loading::resources::AssetLoaderQueue, input::InputState, resources::MeshLookup},
  wgpu_renderer::frame::WgpuFrame,
  Context,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::game::asset_loading::MultithreadedAssetLoaderQueue;

use crate::{
  camera::Camera,
  game::components::{LightSource, RenderModel, Transform3D},
};
use legion::serialize::Canon;

use std::sync::{Arc, RwLock};
#[cfg(feature = "wgpu_renderer")]
pub use wgpu_renderer::*;

#[cfg(feature = "wgpu_renderer")]
mod wgpu_renderer {
  use super::*;
  use crate::wgpu_renderer::context::Context;
  use std::sync::{Arc, RwLock};

  // methods with wgpu render backend
  impl GameState {
    pub fn wgpu_setup(&mut self, context_ptr: Arc<RwLock<Context>>) {
      self.resources.insert(context_ptr);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
}
