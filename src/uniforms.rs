use nalgebra_glm as glm;
use crate::camera::Camera;
#[repr(C)]
#[derive(Debug, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
  pub view_projection: [f32; 16],
}

impl Default for Uniforms {
  fn default() -> Self {
    let view_projection = glm::identity::<f32, 4>();

    Self {
      view_projection: view_projection.into()
    }
  }
}

impl Uniforms {
  pub fn update_from_camera(&mut self, camera: &Camera){
    self.view_projection = camera.view_projection().into();
  }
}