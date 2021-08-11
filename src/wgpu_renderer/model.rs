use super::{material::Material, mesh::Mesh};
use crate::wgpu_renderer::mesh::MeshGeometry;
use anyhow::anyhow;
use gltf::Document;
use std::iter::Zip;
use crate::anyhow::Error;

#[derive(Debug)]
pub struct Model {
  pub meshes: Vec<Mesh>,
  pub materials: Vec<Material>,
  pub load_state: ModelLoadState,
}

impl Model {
  pub fn from_gltf_import(documents: Document) -> anyhow::Result<Self> {
    todo!()
  }
  pub fn load_sample_model() -> anyhow::Result<Self> {
    let (document, buffers, images) = gltf::import("assets/sheen-chair/SheenChair.glb")?;
    for mesh in document.meshes() {
      let geom = MeshGeometry::from_gltf_mesh(&mesh, &buffers)?;
    }
    todo!()
  }
}

#[derive(Clone, Debug)]
pub enum ModelLoadState {
  Loaded,
  Loading,
  Failed(String),
}

impl Default for ModelLoadState {
  fn default() -> Self {
    Self::Loaded
  }
}

///
/// A mesh container for asynchronously loaded models
#[derive(Debug)]
pub struct StreamingMesh {
  pub(crate) path: String,
  pub(crate) mesh_index: usize,
  pub(crate) state: ModelLoadState,
  pub(crate) primitives: Vec<Mesh>,
}

impl StreamingMesh {
  pub fn new(path: String) -> Self {
    Self::new_with_index(path, 0)
  }
  pub fn new_with_index(path: String, index: usize) -> Self {
    Self {
      path,
      state: ModelLoadState::Loading,
      primitives: Vec::new(),
      mesh_index: index,
    }
  }

  pub fn load_gltf_geometry(&mut self, document: &gltf::Document, buffers: &[gltf::buffer::Data]) -> anyhow::Result<()> {
    let loaded_primitives = document
      .meshes()
      .nth(self.mesh_index)
      .ok_or_else(|| anyhow!("no mesh found at index {}, '{}'", self.mesh_index, self.path))
      .and_then(|mesh| {
        let primitives = MeshGeometry::from_gltf_mesh(&mesh, &buffers)?.into_iter().map(|geometry|
          Mesh::new(geometry, None)
        ).collect::<Vec<_>>();
        Ok(primitives)
      })?;
    self.primitives = loaded_primitives;
    Ok(())
  }
}