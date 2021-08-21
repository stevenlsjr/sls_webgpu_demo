pub use crate::renderer_common::geometry::MeshGeometry;
use crate::renderer_common::geometry::{self, Vertex};

use crate::{
  error::Error,
  imgui::__core::ops::Range,
  renderer_common::{
    handle::{Handle, HandleIndex},
    render_context::DrawModel,
  },
  wgpu_renderer::material::Material,
};
use genmesh::{
  generators::{IcoSphere, SharedVertex},
  Indexer, LruIndexer, MapToVertices, Triangulate, Vertices,
};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[derive(Debug)]
pub struct Mesh {
  geometry: MeshGeometry,
  buffers: Option<MeshBuffers>,
  material: Option<Handle<Material>>,
}

impl Mesh {
  pub fn new(geometry: MeshGeometry, buffers: Option<MeshBuffers>) -> Self {
    Self {
      geometry,
      buffers,
      material: None,
    }
  }

  pub fn from_geometry(geometry: MeshGeometry, device: &wgpu::Device) -> Result<Self, Error> {
    let buffers = Some(geometry.create_buffers(device)?);
    Ok(Self {
      buffers,
      geometry,
      material: None,
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

  #[inline]
  pub fn material(&self) -> Option<Handle<Material>> {
    self.material
  }
  #[inline]
  pub fn set_material(&mut self, handle: Option<Handle<Material>>) {
    self.material = handle
  }
}

#[derive(Debug)]
pub struct MeshBuffers {
  pub index_buffer: wgpu::Buffer,
  pub vertex_buffer: wgpu::Buffer,
}

impl<'a, 'b> DrawModel<'a, 'b> for wgpu::RenderPass<'a>
where
  'b: 'a,
{
  type Model = Mesh;

  fn draw_model(&mut self, model: &'b Self::Model) {
    self.draw_model_instanced(model, 0..1);
  }

  fn draw_model_instanced(&mut self, model: &'b Self::Model, instances: Range<u32>) {
    match model.buffers.as_ref() {
      Some(mesh) => {
        let n_indices = model.geometry().indices.len() as u32;
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        // instance matrix data
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        self.draw_indexed(0..n_indices, 0, instances);
      }
      None => {
        log::error!("missing gpu resources");
      }
    }
  }
}
