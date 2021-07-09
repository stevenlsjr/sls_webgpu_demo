use crate::camera::Camera;
use crate::game::components::{
  CameraEntityRow, DebugShowScene, GameLoopTimer, IsMainCamera, Transform3D,
};
use crate::game::input::InputResource;
use crate::game::transform::Transform;
use crate::legion::world::SubWorld;
use crate::platform::keyboard::Keycode;
use legion::systems::CommandBuffer;
use legion::*;
use legion::*;
use log::*;

#[system]
pub fn fixed_update_logging(#[resource] game_loop: &GameLoopTimer) {
  debug!("fixed time update! {:?}", game_loop.fixed_dt)
}

#[system]
pub fn per_frame_logging(#[resource] game_loop: &GameLoopTimer) {
  debug!("update! {:?}", game_loop.per_frame_dt)
}

#[system]
pub fn setup_scene(command_buffer: &mut CommandBuffer) {
  let main_camera: CameraEntityRow = (
    Transform3D::default(),
    IsMainCamera(true),
    Camera::default(),
  );
  command_buffer.push(main_camera);
}

pub use super::camera_systems::*;
