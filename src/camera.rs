use crate::game::components::*;
use lazy_static::lazy_static;
use nalgebra_glm::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Camera {
  pub position: Vec3,
  pub up: Vec3,
  pub world_up: Vec3,

  pub(crate) front: Vec3,

  pub yaw: f32,
  pub pitch: f32,

  pub aspect: f32,
  pub fovy: f32,
  pub znear: f32,
  pub zfar: f32,

  pub movement_speed: f32,
  pub mouse_sensitivity: f32,

  /// If true, aspect ratio should
  /// be the same as the main window's drawable
  /// size
  pub aspect_matches_window: bool,
}

impl Camera {
  pub(crate) fn get_front_vector(&self) -> Vec3 {
    vec3(
      f32::cos(self.yaw) * f32::cos(self.pitch),
      f32::sin(self.pitch),
      f32::sin(self.yaw) * f32::cos(self.pitch),
    )
    .normalize()
  }
}

impl Camera {
  #[inline]
  pub fn view(&self) -> Mat4 {
    
    look_at(&self.position, &(&self.position + &self.front), &self.up)
  }

  #[inline]
  pub fn projection(&self) -> Mat4 {
    perspective(self.aspect, self.fovy, self.znear, self.zfar)
  }

  pub fn view_projection(&self) -> Mat4 {
    let view = self.view();
    let proj = self.projection();
    proj * view
  }
  #[inline]
  pub fn front(&self) -> &Vec3 {
    &self.front
  }

  pub fn update_front(&mut self) -> &Vec3 {
    self.front = self.get_front_vector();
    &self.front
  }

  pub fn update_transformation(&mut self, transform: &Transform3D) {
    self.position.clone_from(&transform.position);
  }
}

impl Default for Camera {
  fn default() -> Self {
    let up: Vec3 = vec3(0.0, 1.0, 0.0);

    let mut cam = Self {
      position: vec3(0.0, 0.0, 0.0),
      world_up: up as Vec3,
      up,
      front: Vec3::zeros(),
      aspect: 1.0,
      fovy: 45.0,
      znear: 0.1,
      zfar: 100.0,
      movement_speed: 2.0,
      pitch: 0f32,
      yaw: -f32::pi() / 2f32,
      mouse_sensitivity: 0.0,
      aspect_matches_window: true,
    };
    cam.front = cam.get_front_vector();
    cam
  }
}

lazy_static! {
  #[rustfmt::skip]
  pub static ref OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
      1.0, 0.0, 0.0, 0.0,
      0.0, 1.0, 0.0, 0.0,
      0.0, 0.0, 0.5, 0.0,
      0.0, 0.0, 0.5, 1.0,
  );
}
