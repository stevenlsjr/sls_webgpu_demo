use genmesh::{MapToVertices, Triangulate, Vertices};
use nalgebra_glm::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, PartialEq)]
pub struct Vertex {
  pub position: [f32; 3],
  pub color: [f32; 4],
  pub uv: [f32; 2],
  pub uv_1: [f32; 2],
  pub normal: [f32; 3],
  pub tangent: [f32; 4],
  pub bitangent: [f32; 4],
}

impl Default for Vertex {
  fn default() -> Self {
    Self {
      position: [0.0; 3],
      color: [1.0; 4],
      uv: [0.0, 0.0],
      uv_1: [0.0, 0.0],
      normal: [0.0, 0.0, 1.0],
      tangent: [0.0; 4],
      bitangent: [0.0; 4],
    }
  }
}

impl Vertex {}

use crate::renderer_common::gltf_loader::LoadPrimitive;
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
    3=>Float32x2,
    4=>Float32x3,
    5=>Float32x3,
    6=>Float32x3
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
  pub gltf_mat_index: Option<usize>,
}

impl Default for MeshGeometry {
  fn default() -> Self {
    Self {
      vertices: vec![],
      indices: vec![],
      label: None,
      gltf_mat_index: None,
    }
  }
}

impl MeshGeometry {
  pub fn unit_sphere(u: usize, v: usize) -> Self {
    use genmesh::{generators::SphereUv, Vertex as GMVertex};

    let sphere: Vec<Vertex> = SphereUv::new(u, v)
      .vertex(|GMVertex { pos, normal }| {
        let pi = std::f32::consts::PI;
        let u = 0.5 + (f32::atan2(pos.x, pos.y) / 2.0 * pi);
        let v = 0.5 + (f32::asin(pos.y) / pi);
        Vertex {
          position: [pos.x, pos.y, pos.z],
          normal: [normal.x, normal.y, normal.z],
          uv: [u, v],
          color: [1.0; 4],
          ..Default::default()
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
        normal: [0.0, 1.0, 0.0],
        ..Default::default()
      },
      Vertex {
        position: [-0.5, 0.5, 0.0],
        uv: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        ..Default::default()
      },
      Vertex {
        position: [0.5, 0.5, 0.0],
        uv: [1.0, 0.0],
        normal: [0.0, 1.0, 0.0],
        ..Default::default()
      },
      Vertex {
        position: [0.5, -0.5, 0.0],
        uv: [1.0, 1.0],
        normal: [0.0, 1.0, 0.0],
        ..Default::default()
      },
    ];
    Self {
      label: Some("unit plane".to_owned()),
      vertices: verts.to_vec(),
      indices: vec![0, 2, 1, 2, 0, 3],
      ..Default::default()
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
          normal: [normal.x, normal.y, normal.z],
          uv: [u, v],
          color: [1.0; 4],
          ..Default::default()
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
      ..Default::default()
    }
  }

  pub fn from_gltf_mesh(
    mesh: &gltf::Mesh,
    buffers: &[gltf::buffer::Data],
  ) -> anyhow::Result<Vec<Self>> {
    let mut meshes = Vec::new();
    for prim in mesh.primitives() {
      match MeshGeometry::load_primitive(&prim, buffers) {
        Ok(p) => meshes.push(p),
        Err(e) => log::error!("could not load primitive {:?}", e),
      }
    }
    Ok(meshes)
  }
}
