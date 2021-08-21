use crate::{renderer_common::handle::HandleIndex, wgpu_renderer::model::ModelLoadState};
use std::{
  borrow::Cow,
  sync::{Arc, RwLock},
};
use uuid::Uuid;

pub type GltfImportOut = (
  gltf::Document,
  Vec<gltf::buffer::Data>,
  Vec<gltf::image::Data>,
);

/// Loads a single gltf
#[derive(Debug, Clone)]
pub struct LoadGltfMesh {
  pub path: String,
  pub mesh_index: usize,
  pub loaded_data: Arc<RwLock<Option<GltfImportOut>>>,
  pub uuid: Uuid,
  pub state: ModelLoadState,
}

impl LoadGltfMesh {
  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }
  pub fn set_mesh_index(&mut self, mesh_index: usize) {
    self.mesh_index = mesh_index;
  }

  pub fn path(&self) -> &str {
    &self.path
  }
  pub fn mesh_index(&self) -> usize {
    self.mesh_index
  }
  pub fn loaded_data(&self) -> &Arc<RwLock<Option<GltfImportOut>>> {
    &self.loaded_data
  }
  pub fn uuid(&self) -> Uuid {
    self.uuid
  }
}

impl LoadGltfMesh {
  pub fn new<P: AsRef<str>>(path: P, mesh_index: usize) -> Self {
    let path = path.as_ref().to_owned();
    let uuid = Uuid::new_v4();
    Self {
      path,
      mesh_index,
      uuid,
      loaded_data: Arc::new(RwLock::new(None)),
      state: ModelLoadState::NotLoaded,
    }
  }
}
