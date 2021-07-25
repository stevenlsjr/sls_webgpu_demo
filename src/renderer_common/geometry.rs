
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 4],
  pub uv: [f32; 2],
  pub normal: [f32; 4],
}

impl Default for Vertex {
  fn default() -> Self {
    Self {
      position: [0.0; 3],
      color: [1.0; 4],
      uv: [0.0, 0.0],
      normal: [0.0, 0.0, 1.0, 1.0],
    }
  }
}

impl Vertex {}

#[cfg(feature = "wgpu_renderer")]
pub use wgpu_renderer::*;
#[cfg(feature = "wgpu_renderer")]
mod wgpu_renderer {
  use super::*;
  static VERTEX_ATTR_ARRAY: &'static [wgpu::VertexAttribute] = &wgpu::vertex_attr_array![
    0=>Float32x3,
    1=>Float32x4,
    2=>Float32x2,
    3=>Float32x4
  ];

  impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
      wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
        step_mode: wgpu::InputStepMode::Vertex,
        attributes: &VERTEX_ATTR_ARRAY,
      }
    }
  }
}

pub const TRIANGLE_VERT: &[Vertex] = &[
  Vertex {
    position: [0.0, 0.5, 0.0],
    color: [1.0, 0.0, 0.0, 1.0],
    normal: [0.0, 0.0, 1.0, 1.0],
    uv: [0.5, 1.0],
  },
  Vertex {
    position: [-0.5, -0.5, 0.0],
    color: [0.0, 1.0, 0.0, 1.0],
    normal: [0.0, 0.0, 1.0, 1.0],
    uv: [0.0, 0.0],
  },
  Vertex {
    position: [0.5, -0.5, 0.0],
    color: [0.0, 0.0, 1.0, 1.0],
    normal: [0.0, 0.0, 1.0, 1.0],
    uv: [1.0, 0.0],
  },
];
pub const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];

pub const QUAD_INDICES: &[u16] = &[];
