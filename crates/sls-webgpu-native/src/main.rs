mod traits;

use log::warn;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowBuildError};
use sdl2::{EventPump, Sdl};
use sls_webgpu::context::Context;
use sls_webgpu::game::input::{InputResource, Sdl2Input};
use sls_webgpu::game::{CreateGameParams, GameState};
use std::any::Any;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::time::*;
use log::error;

fn main() -> Result<(), String> {
  env_logger::init();
  let sdl = sdl2::init()?;
  let video_sys = sdl.video()?;
  let window = video_sys
    .window("Webgpu demo!", 800, 800)
    .resizable()
    .position_centered()
    .build()
    .map_err(|e| e.to_string())?;
  let (width, height) = window.size();
  let event_pump = sdl.event_pump()?;
  let context =
    pollster::block_on(Context::new(window).build()).map_err(|e| format!("{}", e))?;

  let input_backend = Sdl2Input::new();
  let game_state = GameState::new(CreateGameParams {
    input_backend: Box::new(input_backend),
  });
  let mut app = App {
    context,
    game_state,
    event_pump,
    sdl,
  };
  app.run();
  Ok(())
}

struct App {
  context: Context<Window>,
  event_pump: EventPump,
  game_state: GameState,
  sdl: sdl2::Sdl,
}

impl App {
  fn run(&mut self) {
    self.game_state.set_is_running(true);
    let mut previous_time = Instant::now();
    let mut update_lag = Duration::from_nanos(0);
    let ms_per_update = Duration::from_millis(1000 / 60);

    self.game_state.on_start();
    while self.game_state.is_running() {
      let current_time = Instant::now();
      let elapsed_time = current_time - previous_time;
      previous_time = current_time;
      update_lag += elapsed_time;

      self.handle_input();
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
      if let Err(e) = self.context.render() {
        panic!("render error! {:?}", e);
      }
    }
  }


  fn handle_input(&mut self) {
    for event in self.event_pump.poll_iter() {
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
  fn sync_input_state(&mut self) -> Result<(), String> {
    let mut input_res = self.game_state
      .resources().get_mut::<InputResource>()
      .ok_or("Could not get input resource as writable")?;
    let sdl2_input = input_res.backend.downcast_mut::<Sdl2Input>()
      .ok_or("input backend is not set as SDL2!")?;
    sdl2_input.sync_input(&self.sdl, &self.event_pump);
    Ok(())
  }
}
