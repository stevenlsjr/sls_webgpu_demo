use log::error;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowBuildError};
use sdl2::EventPump;
use sls_webgpu::game::input::{InputResource, Sdl2Input};
use sls_webgpu::game::{CreateGameParams, GameState};
use sls_webgpu::platform::gui::DrawUi;
use sls_webgpu::{imgui, imgui_wgpu, platform::sdl2_backend::ImguiSdlPlatform, Context};
use std::ops::DerefMut;
use std::sync::{Arc, PoisonError, RwLock, RwLockWriteGuard};
use std::time::*;

pub struct App {
  pub(crate) context: Context<Window>,
  pub(crate) event_pump: EventPump,
  pub(crate) game_state: GameState,
  pub(crate) imgui_context: Arc<RwLock<imgui::Context>>,
  pub(crate) imgui_renderer: Arc<RwLock<imgui_wgpu::Renderer>>,
  pub(crate) imgui_platform: Arc<RwLock<ImguiSdlPlatform>>,
  pub(crate) sdl: sdl2::Sdl,
}

impl App {
  pub(crate) fn run(mut self) {
    self.game_state.set_is_running(true);
    let mut previous_time = Instant::now();
    let mut update_lag = Duration::from_nanos(0);
    let ms_per_update = Duration::from_millis(1000 / 60);
    let mut imgui_context = self.imgui_context.clone(); // take ownership from the App object
    {
      match (self.imgui_platform.write(), imgui_context.write()) {
        (Ok(mut platform), Ok(mut context)) => {
          platform.on_start(context.io_mut(), self.context.window())
        }
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
      self.context.update();

      if let Err(e) = self.on_render() {
        panic!("render error! {:?}", e);
      }
    }
  }
  fn on_render(&mut self) -> Result<(), sls_webgpu::Error> {
    let render_result = match (
      self.imgui_context.write(),
      self.imgui_renderer.write(),
      self.imgui_platform.clone().write(),
    ) {
      (Ok(mut context), Ok(mut renderer), Ok(mut platform)) => {
        {
          let window = self.context.window();
          platform.prepare_frame(context.io_mut(), window, &self.event_pump.mouse_state());
        }
        let mut ui = context.frame();
        self.game_state.draw_ui(&mut ui);
        platform.prepare_render(&ui, self.context.window());
        self.context.render(&self.game_state)
      }
      (a, b, c) => {
        log::warn!(
          "could not acquire write locks! {:?}, {:?}, {:?}",
          a.err(),
          b.err(),
          c.err()
        );
        self.context.render(&self.game_state)
      }
    };

    render_result.map_err(|e| sls_webgpu::Error::Other { reason: e })
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
        Event::Window { win_event, .. } => match win_event {
          WindowEvent::Resized(width, height) => {
            self.context.on_resize((width as u32, height as u32));
          }
          _ => {}
        },
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

  fn update_gui<'a>(&mut self, ui: imgui::Ui<'a>, dt: &Duration) -> imgui::Ui<'a> {
    use sls_webgpu::imgui::*;
    Window::new(im_str!("Hello"))
      .size([300.0, 100.0], Condition::FirstUseEver)
      .build(&ui, || {
        ui.text(im_str!("Hello world!!!"));
        ui.text(format!("DT: {:?}", dt));
      });

    ui
  }
}
