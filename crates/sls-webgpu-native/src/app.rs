use log::error;
use sdl2::{
  event::{Event, WindowEvent},
  keyboard::Keycode,
  video::Window,
  EventPump,
};

use sls_webgpu::game::input::{InputResource, Sdl2Input};

use sls_webgpu::game::GameState;

use sls_webgpu::platform::gui::DrawUi;

use sls_webgpu::{imgui, imgui_wgpu, platform::sdl2_backend::ImguiSdlPlatform, Context};
use std::{
  ops::DerefMut,
  sync::{Arc, RwLock},
  time::*,
};

pub struct App {
  pub(crate) context: Arc<RwLock<Context>>,
  pub(crate) event_pump: EventPump,
  pub(crate) game_state: GameState,
  pub(crate) imgui_context: Arc<RwLock<imgui::Context>>,
  pub(crate) imgui_renderer: Arc<RwLock<imgui_wgpu::Renderer>>,
  pub(crate) imgui_platform: Arc<RwLock<ImguiSdlPlatform>>,
  pub(crate) sdl: sdl2::Sdl,
  pub window: Window,
}

impl App {
  pub(crate) fn run(mut self) {
    self.game_state.set_is_running(true);
    self.load_assets();
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
      previous_time = current_time;
      update_lag += elapsed_time;
      {
        let mut imgui_context = imgui_context.write().expect("imgui context lock poisoned");
        self.handle_input(imgui_context.deref_mut());
      }
      if !self.game_state.is_running() {
        break;
      }
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

    let mut gui_renderer_arc = self
      .imgui_renderer
      .write()
      .map_err(|e| Error::from_other(format!("lock is poisoned! {:?}", e)))?;

    self
      .context
      .write()
      .expect("Deadlock on render context")
      .render_with_ui(&self.game_state, ui, &mut gui_renderer_arc)
      .map_err(|e| sls_webgpu::Error::FromError(e.into()))
  }

  pub(crate) fn handle_input(&mut self, imgui_context: &mut imgui::Context) {
    let imgui_platform = self.imgui_platform.clone();
    let mut imgui_lock = imgui_platform
      .write()
      .unwrap_or_else(|e| panic!("imgui rwlock is poisoned!: {:?}", e));
    for event in self.event_pump.poll_iter() {
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
    let sdl2_input = input_res
      .backend
      .downcast_mut::<Sdl2Input>()
      .ok_or("input backend is not set as SDL2!")?;
    sdl2_input.sync_input(&self.sdl, &self.event_pump);
    Ok(())
  }

  fn load_assets(&mut self) {}
}
