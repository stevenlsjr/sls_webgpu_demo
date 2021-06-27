mod traits;

use std::error::Error;
use sdl2::video::WindowBuildError;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sls_webgpu::context::Context;
use log::{warn};

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
  let mut event_pump = sdl.event_pump()?;
  let mut context = pollster::block_on(Context::new(window)
    .build())
    .map_err(|e| format!("{}", e))?;


  'running: loop {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => {
          break 'running;
        }
        Event::Window { win_event, .. } => match win_event {
          WindowEvent::Resized(width, height) => {
            context.on_resize((width as u32, height as u32));
          }
          _ => {}
        }
        ,
        _ => {}
      }
    }
    context.update();
    if let Err(e) = context.render() {
      panic!("render error! {:?}", e);
    }
  }

  Ok(())
}
