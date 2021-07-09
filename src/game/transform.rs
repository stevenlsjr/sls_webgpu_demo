use nalgebra_glm::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
  pub position: Vec3,
  pub scale: Vec3,
  pub rotation: Quat,
}

impl Default for Transform {
  fn default() -> Self {
    Self {
      position: vec3(0.0, 0.0, 0.0),
      scale: vec3(1.0, 1.0, 1.0),
      rotation: Quat::identity(),
    }
  }
}
