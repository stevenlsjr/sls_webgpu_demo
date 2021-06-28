use nalgebra_glm::*;
use crate::camera::Camera;
#[repr(C)]
#[derive(Debug,Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
  pub view_projection: [[f32; 4]; 4],
}

impl Default for Uniforms {
  fn default() -> Self {
    let view_projection = Mat4::identity();
    Self {
      view_projection: view_projection.as_ref().clone()
    }
  }
}

impl Uniforms {
  pub fn update_from_camera(&mut self, camera: &Camera){
    self.view_projection = camera.view_projection().into();
  }
}