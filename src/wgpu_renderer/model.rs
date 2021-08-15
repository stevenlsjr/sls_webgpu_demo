use super::{material::Material, mesh::Mesh};
use crate::wgpu_renderer::mesh::MeshGeometry;
use anyhow::anyhow;
use gltf::Document;
use std::iter::Zip;
use crate::anyhow::Error;
use std::collections::HashMap;
use crate::renderer_common::handle::HandleIndex;
use crate::wgpu_renderer::Context;

#[derive(Debug)]
pub struct Model {
  pub meshes: Vec<HandleIndex>,
  pub materials: HashMap<usize, HandleIndex>,
  pub load_state: ModelLoadState,
}

impl Model {
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
  pub(crate) primitives: Vec<HandleIndex>,
  pub(crate) material_handles: HashMap<usize, HandleIndex>,
}

impl StreamingMesh {}

/// accessor implementations
impl StreamingMesh {
  #[inline]
  pub fn path(&self) -> &str {
    &self.path
  }
  #[inline]
  pub fn mesh_index(&self) -> usize {
    self.mesh_index
  }
  #[inline]
  pub fn state(&self) -> &ModelLoadState {
    &self.state
  }
  #[inline]
  pub fn primitives(&self) -> &Vec<HandleIndex> {
    &self.primitives
  }
  #[inline]
  pub fn material_handles(&self) -> &HashMap<usize, HandleIndex> {
    &self.material_handles
  }
  #[inline]
  pub fn set_path(&mut self, path: String) {
    self.path = path;
  }
  #[inline]
  pub fn set_mesh_index(&mut self, mesh_index: usize) {
    self.mesh_index = mesh_index;
  }
  #[inline]
  pub fn set_state(&mut self, state: ModelLoadState) {
    self.state = state;
  }
  #[inline]
  pub fn primitives_mut(&mut self) -> &mut Vec<HandleIndex> {
    &mut self.primitives
  }
  #[inline]
  pub fn material_handles_mut(&mut self) -> &mut HashMap<usize, HandleIndex> {
    &mut self.material_handles
  }
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
      material_handles: Default::default(),
    }
  }


  pub fn load_from_gltf_impl(&mut self, context: &mut Context, document: Document, buffers: Vec<gltf::buffer::Data>, images: Vec<gltf::image::Data>) -> anyhow::Result<()> {
    let mesh = document
      .meshes()
      .nth(self.mesh_index)
      .ok_or(anyhow!("Document does not have a mesh"))?;


    let geometry = MeshGeometry::from_gltf_mesh(&mesh, &buffers)?;
    let materials = Material::from_gltf(&document, &images)?;
    let mut material_handles: HashMap<usize, HandleIndex> = HashMap::default();
    let mut meshes: Vec<HandleIndex> = Vec::with_capacity(geometry.len());
    {

      let mut mesh_loader = context.meshes
        .write().map_err(|e| anyhow!("{:?}", e))?;
      let mut material_loader = context.materials.write()
        .map_err(|e| anyhow!("{:?}", e))?;
      for mat in materials {
        let index = mat.index;
        let handle = material_loader.insert(mat);
        material_handles.insert(index, handle);
      }
      for mesh_geom in geometry.into_iter() {
        let mut mesh = Mesh::from_geometry(mesh_geom, &context.device)?;
        if let Some(material_idx) = mesh.geometry().gltf_mat_index {
          mesh.set_material(material_handles.get(&material_idx).cloned());
        }
        let handle = mesh_loader.insert(mesh);
        meshes.push(handle);
      }
    }
    self.primitives = meshes;
    self.material_handles = material_handles;
    self.state = ModelLoadState::Loaded;

    Ok(())
  }

  pub fn load_from_gltf(&mut self, context: &mut Context, document: Document, buffers: Vec<gltf::buffer::Data>, images: Vec<gltf::image::Data>) -> anyhow::Result<()> {
    match self.load_from_gltf_impl(context, document, buffers, images) {
      Err(e) => {
        self.state = ModelLoadState::Failed(format!("{:?}", e));
        Err(e)
      }
      ok => ok
    }
  }
}