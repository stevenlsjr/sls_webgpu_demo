use crate::camera::Camera;
use crate::game::components::{CameraEntityRow, GameLoopTimer, Transform3D};
use legion::systems::CommandBuffer;
use legion::*;
use log::*;
use nalgebra_glm as glm;

#[system]
pub fn fixed_update_logging(#[resource] game_loop: &GameLoopTimer) {
  debug!("fixed time update! {:?}", game_loop.fixed_dt)
}

#[system]
pub fn per_frame_logging(#[resource] game_loop: &GameLoopTimer) {
  debug!("update! {:?}", game_loop.per_frame_dt)
}

/**
 * This is executed when the scene is initialized
 */
#[system]
pub fn setup_scene(#[resource] scene: &mut Scene, command_buffer: &mut CommandBuffer) {
  let mut main_camera: CameraEntityRow = (Transform3D::default(), Camera::default());
  main_camera.0.position = glm::vec3(0f32, 0f32, 1f32);

  let main_camera_entity = command_buffer.push(main_camera);
  scene.main_camera = Some(main_camera_entity);
}

pub use super::camera_systems::*;
use crate::game::resources::Scene;
