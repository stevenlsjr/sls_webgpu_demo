use super::components::*;
use super::input::InputResource;
use crate::camera::Camera;
use crate::game::resources::Scene;
use crate::platform::keyboard::Keycode;
use legion::*;
use log::*;
use nalgebra_glm::*;

#[system(for_each)]
#[write_component(Transform3D)]
#[write_component(Camera)]
pub fn camera_move(
  #[resource] scene: &Scene,
  entity: &Entity,
  transform: &mut Transform3D,
  camera: &mut Camera,
  #[resource] input: &InputResource,
  #[resource] game_loop: &GameLoopTimer,
) {
  if !scene.is_main_camera(Some(*entity)) {
    return;
  }
  camera.position = transform.position.clone_owned();
  let front = camera.update_front().clone();
  let right = front.cross(&camera.world_up).normalize();
  let mut movement_step: Vec3 = vec3(0f32, 0f32, 0f32);
  let frame_speed = camera.movement_speed * game_loop.fixed_dt.as_secs_f32();

  if input.backend.pressed_keycodes().contains(&Keycode::W) {
    movement_step += front;
  }
  if input.backend.pressed_keycodes().contains(&Keycode::S) {
    movement_step -= front;
  }
  if input.backend.pressed_keycodes().contains(&Keycode::A) {
    movement_step -= &right;
  }
  if input.backend.pressed_keycodes().contains(&Keycode::D) {
    movement_step += &right;
  }

  if movement_step.magnitude().abs() >= f32::EPSILON {
    movement_step.normalize_mut();
    movement_step *= frame_speed;
    transform.position += movement_step;
  }
  camera.position.clone_from(&transform.position);
  if input.backend.pressed_keycodes().contains(&Keycode::P) {
    log::info!(
      "debugger! camera {:?}, transform {:?}",
      camera.position,
      transform.position
    );
  }
}
