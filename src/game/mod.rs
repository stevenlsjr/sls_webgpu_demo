use std::{fmt, time::Duration};

use legion::*;

#[cfg(feature = "wgpu_imgui")]
pub use wgpu_imgui::*;

use crate::game::{
  components::{DebugShowScene, GameLoopTimer},
  input::{InputBackend, InputResource},
  resources::{Scene, UIDataIn, UIDataOut},
  systems::*,
};
use std::borrow::BorrowMut;

pub mod components;
pub mod input;
pub mod resources;
pub mod systems;

pub mod asset_loading;

#[cfg(feature = "html5_backend")]
pub mod html5_backend;

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
}

impl fmt::Debug for GameState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("GameState")
      .field("world", &self.world)
      .field("is_running", &self.is_running)
      .finish()
  }
}

pub struct CreateGameParams {}

impl GameState {
  pub fn new(options: CreateGameParams) -> Self {
    let world = World::default();
    let is_running = false;
    let fixed_schedule = Schedule::builder()
      .add_system(systems::fixed_update_logging_system())
      .add_system(systems::write_camera_ui_data_system())
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
    Self {
      world,
      is_running,
      fixed_schedule,
      per_frame_schedule,
      resources,
      on_resize_schedule,
    }
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
      use asset_loading::AssetLoaderResource;
      resources.insert(AssetLoaderResource::new())
    }

    resources
  }

  /// Lifecycle functions
  /// on_start, update, fixed_update, resize, etc

  pub fn on_start(&mut self) {
    let mut scheduler = Schedule::builder()
      .add_thread_local(setup_scene_system())
      .add_thread_local(systems::model_systems::create_models_system())
      .build();
    scheduler.execute(&mut self.world, &mut self.resources);
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

use crate::game::{input::InputState, resources::MeshLookup};
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
  use crate::game::input::InputState;

  use super::*;

  struct Suite {
    game: GameState,
  }

  fn setup() -> Suite {
    let game = GameState::new(CreateGameParams {});
    Suite { game }
  }

  #[test]
  fn test_is_main_camera() {
    let _foo = setup();
  }
}
