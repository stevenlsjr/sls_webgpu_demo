use crate::{game::components::Transform3D, wgpu::VertexFormat};
use nalgebra_glm::*;
use wgpu::{InputStepMode, VertexBufferLayout};

/// Instance data for model
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelInstance {
  model: [[f32; 4]; 4],
}

const VERTEX_ATTR_ARRAY: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
6 => Float32x4,
7 => Float32x4,
8 => Float32x4,
9 => Float32x4
];

impl ModelInstance {
  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    VertexBufferLayout {
      array_stride: std::mem::size_of::<ModelInstance>() as _,
      step_mode: InputStepMode::Instance,
      attributes: &VERTEX_ATTR_ARRAY,
    }
  }
}

impl From<&Transform3D> for ModelInstance {
  fn from(xform: &Transform3D) -> Self {
    let model = xform.matrix();
    Self {
      model: model.into(),
    }
  }
}
