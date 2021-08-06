use super::{material::Material, mesh::Mesh};
use crate::wgpu_renderer::mesh::MeshGeometry;
use anyhow::anyhow;
use gltf::Document;
use std::iter::Zip;

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
