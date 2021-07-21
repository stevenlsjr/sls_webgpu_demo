use memoffset::offset_of;
use lazy_static::lazy_static;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 4],
  pub normal: [f32; 4],
}

impl Default for Vertex {
  fn default() -> Self {
    Self {
      position: [0.0; 3],
      color: [1.0; 4],
      normal: [0.0, 0.0, 1.0, 1.0],
    }
  }
}

lazy_static! {

  static ref VERTEX_ATTRIBUTES: Vec<wgpu::VertexAttribute> = vec![
        // position
        wgpu::VertexAttribute {
          offset: 0,
          shader_location: 0,
          format: wgpu::VertexFormat::Float32x3,
        },
        // color
        wgpu::VertexAttribute {
          offset: offset_of!(Vertex, color) as wgpu::BufferAddress,
          shader_location: 1,
          format: wgpu::VertexFormat::Float32x4,
        },
        // normal
        wgpu::VertexAttribute {
          offset: offset_of!(Vertex, normal) as wgpu::BufferAddress,
          shader_location: 2,
          format: wgpu::VertexFormat::Float32x4,
        },
  ];

}

impl Vertex {
  pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
    wgpu::VertexBufferLayout {
      array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
      step_mode: wgpu::InputStepMode::Vertex,
      attributes: &VERTEX_ATTRIBUTES,
    }
  }
}

pub const TRIANGLE_VERT: &[Vertex] = &[
  Vertex {
    position: [0.0, 0.5, 0.0],
    color: [1.0, 0.0, 0.0, 1.0],
    normal: [0.0, 0.0, 1.0, 1.0],
  },
  Vertex {
    position: [-0.5, -0.5, 0.0],
    color: [0.0, 1.0, 0.0, 1.0],
    normal: [0.0, 0.0, 1.0, 1.0],
  },
  Vertex {
    position: [0.5, -0.5, 0.0],
    color: [0.0, 0.0, 1.0, 1.0],
    normal: [0.0, 0.0, 1.0, 1.0],
  },
];
pub const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];

pub const QUAD_INDICES: &[u16] = &[];

