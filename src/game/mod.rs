use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use legion::*;
use log::info;
use serde::Serialize;

#[cfg(feature = "wgpu_imgui")]
use wgpu_imgui::*;

use crate::camera::Camera;
use crate::game::components::{DebugShowScene, GameLoopTimer};
use crate::game::input::{InputBackend, InputResource};
use crate::game::resources::Scene;
use crate::game::systems::*;

mod camera_systems;
pub mod components;
pub mod input;
pub mod resources;
pub mod systems;

use atomic_refcell::AtomicRef;

#[cfg(feature = "html5_backend")]
pub mod html5_backend;

pub struct GameState {
  world: World,
  fixed_schedule: Schedule,
  per_frame_schedule: Schedule,
  resources: Resources,
  is_running: bool,
}

impl fmt::Debug for GameState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("GameState")
      .field("world", &self.world)
      .field("is_running", &self.is_running)
      .finish()
  }
}

pub struct CreateGameParams {
  pub input_backend: Box<dyn InputBackend>,
}

impl GameState {
  pub fn new(options: CreateGameParams) -> Self {
    let CreateGameParams { input_backend } = options;

    let mut world = World::default();
    let is_running = false;
    let fixed_schedule = Schedule::builder()
      .add_system(systems::fixed_update_logging_system())
      .build();
    let per_frame_schedule = Schedule::builder()
      .add_system(systems::per_frame_logging_system())
      .add_thread_local(systems::camera_move_system())
      .build();
    let mut resources = Self::initial_resources();
    resources.insert(InputResource {
      backend: input_backend,
    });
    Self {
      world,
      is_running,
      fixed_schedule,
      per_frame_schedule,
      resources,
    }
  }

  fn initial_resources() -> Resources {
    let mut resources = Resources::default();
    resources.insert(GameLoopTimer::default());
    resources.insert(DebugShowScene(false));
    resources.insert(Scene { main_camera: None });
    resources
  }

  pub fn on_start(&mut self) {
    let mut scheduler = Schedule::builder()
      .add_thread_local(setup_scene_system())
      .build();
    scheduler.execute(&mut self.world, &mut self.resources);
  }

  pub fn update(&mut self, dt: &Duration) {
    {
      let mut loop_timer = self
        .resources
        .get_mut::<GameLoopTimer>()
        .unwrap_or_else(|| panic!("game loop missing! "));
      loop_timer.per_frame_dt = dt.clone();
    }
    self
      .per_frame_schedule
      .execute(&mut self.world, &mut self.resources);
  }
  pub fn fixed_update(&mut self, dt: &Duration) {
    {
      let mut loop_timer = self
        .resources
        .get_mut::<GameLoopTimer>()
        .unwrap_or_else(|| panic!("game loop missing! "));
      loop_timer.fixed_dt = dt.clone();
    }
    self
      .fixed_schedule
      .execute(&mut self.world, &mut self.resources);
  }

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
  pub fn mut_resources(&mut self) -> &mut Resources {
    &mut self.resources
  }

  pub fn world(&self) -> &World {
    &self.world
  }
  pub fn fixed_schedule(&self) -> &Schedule {
    &self.fixed_schedule
  }
  pub fn per_frame_schedule(&self) -> &Schedule {
    &self.per_frame_schedule
  }

  pub fn map_input_backend<B: InputBackend, R, F: FnOnce(&B) -> R>(
    &self,
    callback: F,
  ) -> Result<R, String> {
    let resource = self
      .resources
      .get::<InputResource>()
      .ok_or_else(|| "input resource is not loaded")?;
    let backend: &B = resource
      .backend
      .downcast_ref()
      .ok_or_else(|| "resource is not the correct type")?;
    Ok(callback(backend))
  }
  pub fn map_input_backend_mut<B: InputBackend, R, F: FnOnce(&mut B) -> R>(
    &mut self,
    callback: F,
  ) -> Result<R, String> {
    let mut resource = self
      .resources
      .get_mut::<InputResource>()
      .ok_or_else(|| "input resource is not loaded")?;
    let backend: &mut B = resource
      .backend
      .downcast_mut()
      .ok_or_else(|| "resource is not the correct type")?;
    Ok(callback(backend))
  }
}

#[cfg(feature = "wgpu_imgui")]
mod wgpu_imgui {
  use super::*;
  use imgui::*;

  use crate::platform::gui::wgpu_imgui::DrawUi;

  impl DrawUi for GameState {
    fn draw_ui(&self, ui: &mut Ui) {
      use imgui::*;
      Window::new(im_str!("Window!!"))
        .size([300.0, 300.0], Condition::Appearing)
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
        });
    }
  }
}

#[cfg(test)]
mod test {
  use nalgebra_glm::*;

  use crate::game::input::DummyInputBackend;

  use super::*;

  struct Suite {
    game: GameState,
  }

  fn setup() -> Suite {
    let game = GameState::new(CreateGameParams {
      input_backend: Box::new(DummyInputBackend::default()),
    });
    Suite { game }
  }

  #[test]
  fn test_is_main_camera() {}
}
