pub use crate::renderer_common::geometry::MeshGeometry;

use std::ops::Range;

use crate::{
  error::Error,
  renderer_common::{
    allocator::ResourceManager,
    handle::{Handle, HandleIndex, ResourceStore},
    render_context::DrawModel,
  },
  wgpu_renderer::{
    material::{Material, RenderMaterial, WgpuMaterial},
    model::StreamingMesh,
    resource_view::{ResourceContext, ResourceView},
    textures::TextureResource,
  },
};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Mesh {
  geometry: MeshGeometry,
  buffers: Option<MeshBuffers>,
  material: Option<Handle<WgpuMaterial>>,
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
  pub fn material(&self) -> Option<Handle<WgpuMaterial>> {
    self.material
  }
  #[inline]
  pub fn set_material(&mut self, handle: Option<Handle<WgpuMaterial>>) {
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
  type Mesh = Mesh;
  type Model = StreamingMesh;
  type Material = wgpu::BindGroup;
  type Uniforms = wgpu::BindGroup;

  fn draw_model<MeshStore: ResourceStore<Self::Mesh>>(
    &mut self,
    mesh_mgr: &'b MeshStore,
    model: &'b Self::Model,
    uniforms: &'a Self::Uniforms,
  ) {
    self.draw_model_instanced(mesh_mgr, model, uniforms, 0..1)
  }

  fn draw_model_instanced<MeshStore: ResourceStore<Self::Mesh>>(
    &mut self,
    mesh_mgr: &'b MeshStore,
    model: &'b Self::Model,
    uniforms: &'a Self::Uniforms,
    instances: Range<u32>,
  ) {
    for (i, mesh) in model.iter_primitives(mesh_mgr).enumerate() {
      let mesh = match mesh {
        Some(x) => x,
        None => {
          log::warn!("mesh {} cannot be retrieved", i);
          continue;
        }
      };
    }
  }

  fn draw_mesh(
    &mut self,
    model: &'b Self::Mesh,
    material: &'a Self::Material,
    uniforms: &'a Self::Uniforms,
  ) {
    self.draw_mesh_instanced(model, material, uniforms, 0..1);
  }

  fn draw_mesh_instanced(
    &mut self,
    model: &'b Self::Mesh,
    material: &'a Self::Material,
    uniforms: &'a Self::Uniforms,
    instances: Range<u32>,
  ) {
    match model.buffers.as_ref() {
      Some(mesh) => {
        let n_indices = model.geometry().indices.len() as u32;
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        // instance matrix data
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.set_bind_group(1, material, &[]);
        self.set_bind_group(0, uniforms, &[]);
        self.draw_indexed(0..n_indices, 0, instances);
      }
      None => {
        log::error!("missing gpu resources");
      }
    }
  }
}
