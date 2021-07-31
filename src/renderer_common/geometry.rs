use genmesh::{MapToVertices, Triangulate, Vertices};
use nalgebra_glm::*;

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
  use crate::{
    error::Error,
    wgpu::util::{BufferInitDescriptor, DeviceExt},
    wgpu_renderer::mesh::MeshBuffers,
  };

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

  impl MeshGeometry {
    pub fn create_buffers(&self, device: &wgpu::Device) -> Result<MeshBuffers, Error> {
      let label = match &self.label {
        Some(l) => Some(l.as_str()),
        None => None,
      };
      let ibo = device.create_buffer_init(&BufferInitDescriptor {
        label,
        contents: bytemuck::cast_slice(&self.indices),
        usage: wgpu::BufferUsage::INDEX,
      });

      let vbo = device.create_buffer_init(&BufferInitDescriptor {
        label,
        contents: bytemuck::cast_slice(&self.vertices),
        usage: wgpu::BufferUsage::VERTEX,
      });
      return Ok(MeshBuffers {
        vertex_buffer: vbo,
        index_buffer: ibo,
      });
    }
  }
}

#[derive(Debug, Clone)]
pub struct MeshGeometry {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u16>,
  pub label: Option<String>,
}

impl Default for MeshGeometry {
  fn default() -> Self {
    Self {
      vertices: vec![],
      indices: vec![],
      label: None,
    }
  }
}

impl MeshGeometry {
  pub fn subdivision_plane(n_divisions: usize) -> Self {
    let mut vertices: Vec<Vertex> = vec![Vertex::default(); (n_divisions + 1).pow(2)];
    let mut indices = Vec::with_capacity(n_divisions * n_divisions * 6);
    let mut i = 0usize;
    for y in 0..=n_divisions {
      for x in 0..=n_divisions {
        let u = (x as f32) / ((n_divisions) as f32);
        let v = (y as f32) / ((n_divisions) as f32);
        let position = vec3(u - 0.5, v - 0.5, 0f32);
        vertices[i].position = position.into();
        vertices[i].uv = [u, v];
        vertices[i].normal = vec4(0.0, 0.0, 1.0, 1.0).into();
        i += 1;
      }
    }

    let mut vi = 0;
    'start: for y in 0..n_divisions {
      for x in 0..n_divisions {
        let mut quad = [0u16; 6];
        quad[0] = vi;
        quad[2] = vi + (n_divisions as u16) + 1;
        quad[1] = vi + 1;
        quad[3] = quad[2];
        quad[4] = quad[1];
        quad[5] = vi + (n_divisions as u16) + 2;
        indices.extend_from_slice(&quad);

        vi += 1;
      }
    }

    Self {
      indices: indices,
      vertices,
      label: Some("subdivision plane".to_owned()),
    }
  }

  pub fn unit_sphere(u: usize, v: usize) -> Self {
    use genmesh::{generators::SphereUv, Vertex as GMVertex};

    let sphere: Vec<Vertex> = SphereUv::new(u, v)
      .vertex(|GMVertex { pos, normal }| {
        let pi = std::f32::consts::PI;
        let u = 0.5 + (f32::atan2(pos.x, pos.y) / 2.0 * pi);
        let v = 0.5 + (f32::asin(pos.y) / pi);
        Vertex {
          position: [pos.x, pos.y, pos.z],
          normal: [normal.x, normal.y, normal.z, 1.0],
          uv: [u, v],
          color: [1.0; 4],
        }
      })
      .triangulate()
      // wrap triangles counter-clockwise
      .vertices()
      .collect();

    Self::from_vertices(sphere)
  }

  pub fn unit_plane() -> Self {
    let verts = [
      Vertex {
        position: [-0.5, -0.5, 0.0],
        uv: [0.0, 1.0],
        normal: [0.0, 1.0, 0.0, 1.0],
        ..Default::default()
      },
      Vertex {
        position: [-0.5, 0.5, 0.0],
        uv: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0, 1.0],
        ..Default::default()
      },
      Vertex {
        position: [0.5, 0.5, 0.0],
        uv: [1.0, 0.0],
        normal: [0.0, 1.0, 0.0, 1.0],
        ..Default::default()
      },
      Vertex {
        position: [0.5, -0.5, 0.0],
        uv: [1.0, 1.0],
        normal: [0.0, 1.0, 0.0, 1.0],
        ..Default::default()
      },
    ];
    Self {
      label: Some("unit plane".to_owned()),
      vertices: verts.to_vec(),
      indices: vec![0, 2, 1, 2, 0, 3],
    }
  }

  pub fn cube() -> Self {
    let cube = genmesh::generators::Cube::new()
      .vertex(|genmesh::Vertex { pos, normal }| {
        let pi = std::f32::consts::PI;
        let u = 0.5 + (f32::atan2(pos.x, pos.y) / 2.0 * pi);
        let v = 0.5 + (f32::asin(pos.y) / pi);
        Vertex {
          position: [pos.x, pos.y, pos.z],
          normal: [normal.x, normal.y, normal.z, 1.0],
          uv: [u, v],
          color: [1.0; 4],
        }
      })
      .triangulate()
      // wrap triangles counter-clockwise
      .vertices()
      .collect();

    Self::from_vertices(cube)
  }

  fn from_vertices(verts: Vec<Vertex>) -> Self {
    let len = verts.len() as u16;
    Self {
      label: Some("unit sphere".to_owned()),
      vertices: verts,
      indices: (0u16..len).collect(),
    }
  }
}
