use crate::camera::Camera;
use nalgebra_glm::*;
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
  pub view_projection: [[f32; 4]; 4],
}

impl Default for Uniforms {
  fn default() -> Self {
    let view_projection = Mat4::identity();
    Self {
      view_projection: *view_projection.as_ref(),
    }
  }
}

impl Uniforms {
  pub fn update_from_camera(&mut self, camera: &Camera) {
    let view_projection = camera.view_projection();
    let proj: [[f32; 4]; 4] = view_projection.into();
    let view = camera.view();
    self.view_projection = proj;
  }
}
