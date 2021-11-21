use std::sync::{Arc, RwLock};

use app::*;
use sdl2::video::Window;
use sls_webgpu::{
  anyhow,
  game::{GameState, GameStateBuilder},
  imgui_wgpu,
  platform::{gui, sdl2_backend::ImguiSdlPlatform},
  Context,
};

mod app;
mod traits;

fn main() -> Result<(), String> {
  env_logger::init();

  let app = app::App::new().map_err(|e| format!("{:?}", e))?;
  app.run();
  Ok(())
}
