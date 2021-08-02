pub use crate::renderer_common::geometry::MeshGeometry;
use crate::renderer_common::geometry::{self, Vertex};

use crate::error::Error;
use genmesh::{
  generators::{IcoSphere, SharedVertex},
  Indexer, LruIndexer, MapToVertices, Triangulate, Vertices,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Debug)]
pub struct Mesh {
  geometry: MeshGeometry,
  buffers: Option<MeshBuffers>,
  material_index: Option<usize>,
}

impl Mesh {
  pub fn new(geometry: MeshGeometry, buffers: Option<MeshBuffers>) -> Self {
    Self {
      geometry,
      buffers,
      material_index: None,
    }
  }

  pub fn from_geometry(geometry: MeshGeometry, device: &wgpu::Device) -> Result<Self, Error> {
    let buffers = Some(geometry.create_buffers(device)?);
    Ok(Self {
      buffers,
      geometry,
      material_index: None,
    })
  }

  #[inline]
  pub fn buffers(&self) -> Option<&MeshBuffers> {
    self.buffers.as_ref()
  }
  #[inline]
  pub fn geometry(&self) -> &MeshGeometry {
    &self.geometry
  }
  #[inline]
  pub fn n_elements(&self) -> usize {
    self.geometry.indices.len()
  }
}

#[derive(Debug)]
pub struct MeshBuffers {
  pub index_buffer: wgpu::Buffer,
  pub vertex_buffer: wgpu::Buffer,
}
