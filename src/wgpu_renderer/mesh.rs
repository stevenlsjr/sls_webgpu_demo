use super::geometry::{Vertex, self};

use crate::error::Error;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use genmesh::{Vertices, Triangulate};
use genmesh::generators::{SharedVertex, IcoSphere};

#[derive(Debug)]
pub struct Mesh {
  geometry: MeshGeometry,
  buffers: Option<MeshBuffers>,
}

impl Mesh {
  pub fn new(geometry: MeshGeometry, buffers: Option<MeshBuffers>) -> Self {
    Self { geometry, buffers }
  }

  pub fn from_geometry(geometry: MeshGeometry, device: &wgpu::Device) -> Result<Self, Error> {
    let buffers = Some(geometry.create_buffers(device)?);
    Ok(Self { buffers, geometry })
  }

  #[inline]
  pub fn buffers(&self) -> Option<&MeshBuffers> {
    self.buffers.as_ref()
  }
  #[inline]
  pub fn geometry(&self) -> &MeshGeometry {
    &self.geometry
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
  fn create_buffers(&self, device: &wgpu::Device) -> Result<MeshBuffers, Error> {
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

  pub fn unit_shere(u: usize, v: usize) -> Self {
    use genmesh::{generators::{SphereUv}, *};
    type MyVertex = geometry::Vertex;
    let mut found = false;
    let mut indexer = LruIndexer::new(u * v * 4, |a, b| {
      found = true;
    });

    let sphere: Vec<MyVertex> = SphereUv::new(u, v)
      .vertex(|Vertex { pos, normal }| {
        let pi = std::f32::consts::PI;
        let u = 0.5 + (f32::atan2(pos.x, pos.y)/2.0*pi);
        let v = 0.5 + (f32::asin(pos.y)/pi);
        MyVertex {
          position: [pos.x, pos.y, pos.z],
          normal: [normal.x, normal.y, normal.z, 1.0],
          uv: [u, v],
          color: [1.0; 4],
        }
      })
      .triangulate()
      // wrap triangles counter-clockwise
      .vertices().collect();
    let mut vertices: Vec<MyVertex> = Vec::with_capacity(sphere.len());
    let mut indices: Vec<u16> = Vec::with_capacity(sphere.len());

    for (i, vert) in sphere.iter().enumerate() {
      let shared_index = indexer.index(vert.clone());

      vertices.push(vert.clone());
      indices.push(i as u16);
    }


    MeshGeometry {
      label: Some("unit sphere".to_owned()),
      vertices,
      indices: indices,
    }
  }
}

#[derive(Debug)]
pub struct MeshBuffers {
  pub index_buffer: wgpu::Buffer,
  pub vertex_buffer: wgpu::Buffer,
}
