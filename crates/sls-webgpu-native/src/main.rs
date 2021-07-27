use std::any::Any;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::time::*;

use sdl2::keyboard::Keycode;
use sdl2::video::{Window, WindowBuildError};
use sdl2::{EventPump, Sdl};

use app::*;
use sls_webgpu::game::input::{InputResource, Sdl2Input};
use sls_webgpu::game::{CreateGameParams, GameState};
use sls_webgpu::platform::gui;
use sls_webgpu::platform::sdl2_backend::ImguiSdlPlatform;
use sls_webgpu::Context;
use sls_webgpu::{imgui, imgui_wgpu};

mod app;
mod traits;

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
  let event_pump = sdl.event_pump()?;
  let context = pollster::block_on(Context::new(window).build()).map_err(|e| format!("{}", e))?;

  let input_backend = Sdl2Input::new();
  let game_state = GameState::new(CreateGameParams {
    input_backend: Box::new(input_backend),
  });
  let mut imgui_context = gui::create_imgui(gui::Options {
    ..Default::default()
  });
  let imgui_platform = ImguiSdlPlatform::new(&mut imgui_context).map_err(|e| format!("{}", e))?;

  let texture_format = context
    .adapter
    .get_swap_chain_preferred_format(&context.surface)
    .ok_or("no swapchain texture format available")?;
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

  let app = App {
    imgui_context: Arc::new(RwLock::new(imgui_context)),
    context,
    imgui_renderer,
    game_state,
    event_pump,
    imgui_platform,
    sdl,
  };
  app.run();
  Ok(())
}
