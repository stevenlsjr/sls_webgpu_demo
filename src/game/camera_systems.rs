use super::components::*;
use super::input::InputResource;
use crate::camera::Camera;
use legion::*;
use crate::platform::keyboard::Keycode;
use nalgebra_glm::*;
use log::*;

#[system(for_each)]
#[read_component(IsMainCamera)]
#[write_component(Transform3D)]
#[write_component(Camera)]
pub fn camera_move(
  is_main_camera: &IsMainCamera,
  transform: &mut Transform3D,
  camera: &mut Camera,
  #[resource] input: &InputResource,
  #[resource] game_loop: &GameLoopTimer,
) {
  if !is_main_camera.0 { return; }
  camera.position = transform.position.clone_owned() ;
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

  if movement_step.magnitude().abs()  >= f32::EPSILON {
    movement_step.normalize_mut();
    movement_step *= frame_speed;
    camera.position += movement_step;
    transform.position = camera.position.clone_owned();
    info!("position is {:?}", &camera.position);
    let view_mat = camera.view();
    for row in view_mat.row_iter() {
      info!("{} {} {} {}", row[0], row[1], row[2], row[3]);
    }
  }
}
