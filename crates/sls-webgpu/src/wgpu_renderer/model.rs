use super::{material::Material, mesh::Mesh};
use crate::{
  anyhow::Error,
  renderer_common::{
    allocator::ResourceManager,
    handle::{Handle, HandleIndex, ResourceStore},
  },
  wgpu_renderer::{
    material::{RenderMaterial, WgpuMaterial},
    mesh::MeshGeometry,
    resource_view::{ReadWriteResources, ResourceView},
    textures::TextureResource,
    Context,
  },
};
use anyhow::anyhow;
use gltf::Document;
use std::{
  collections::{hash_map::RandomState, HashMap},
  iter::{Iterator, Zip},
  sync::{Arc, RwLock, Weak},
};

#[derive(Debug)]
pub struct Model {
  pub meshes: Vec<HandleIndex>,
  pub materials: HashMap<usize, HandleIndex>,
  pub load_state: ModelLoadState,
}

impl Model {
  pub fn load_sample_model() -> anyhow::Result<Self> {
    let (document, buffers, _images) = gltf::import("assets/sheen-chair/SheenChair.glb")?;
    for mesh in document.meshes() {
      let _geom = MeshGeometry::from_gltf_mesh(&mesh, &buffers)?;
    }
    todo!()
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ModelLoadState {
  NotLoaded,
  Loaded,
  Loading,
  Failed(String),
}

impl Default for ModelLoadState {
  fn default() -> Self {
    Self::NotLoaded
  }
}

///
/// A mesh container for asynchronously loaded models
#[derive(Debug)]
pub struct StreamingMesh {
  pub(crate) path: String,
  pub(crate) mesh_index: usize,
  pub(crate) state: ModelLoadState,
  pub(crate) primitives: Vec<Handle<Mesh>>,
  pub(crate) materials: Option<Weak<RwLock<ResourceManager<WgpuMaterial>>>>,
}

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
  pub fn primitives(&self) -> &Vec<Handle<Mesh>> {
    &self.primitives
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
  pub fn primitives_mut(&mut self) -> &mut Vec<Handle<Mesh>> {
    &mut self.primitives
  }

  pub fn new(path: String) -> Self {
    Self::new_with_index(path, 0)
  }
  pub fn new_with_index(path: String, index: usize) -> Self {
    Self {
      path,
      state: ModelLoadState::Loading,
      primitives: Vec::new(),
      mesh_index: index,
      materials: None,
    }
  }

  pub fn load_from_gltf_impl(
    &mut self,
    context: &mut Context,
    document: &Document,
    buffers: &Vec<gltf::buffer::Data>,
    images: &Vec<gltf::image::Data>,
  ) -> anyhow::Result<()> {
    let mesh = document
      .meshes()
      .nth(self.mesh_index)
      .ok_or(anyhow!("Document does not have a mesh"))?;

    let geometry = MeshGeometry::from_gltf_mesh(&mesh, buffers)?;
    let materials = Material::from_gltf(document, images)?;
    let mut material_handles: HashMap<usize, _> = HashMap::default();
    let mut meshes: Vec<Handle<Mesh>> = Vec::with_capacity(geometry.len());
    {
      let mut mesh_loader = context
        .resources
        .meshes
        .write()
        .expect("RwLock is poisoned!");
      let mut material_loader = context.resources.materials.write().unwrap();
      let mut tex_loader = context.resources.textures.write().unwrap();
      for mat in materials {
        let index = mat.index;
        let gpu_material = WgpuMaterial::from_material(
          &mat,
          &context.queue,
          &context.device,
          &context.texture_bind_group_layout,
          &mut *tex_loader,
          context.fallback_texture,
        )?;
        let gpu_material = material_loader.insert(gpu_material);
        material_handles.insert(index, gpu_material);
      }
      for mesh_geom in geometry.into_iter() {
        let mut mesh = Mesh::from_geometry(mesh_geom, &context.device)?;
        match mesh.geometry().gltf_mat_index {
          Some(material_idx) => {
            let handle = material_handles.get(&material_idx).unwrap_or_else(|| {
              panic!(
                "could not retrieve material with index {}, existing handles: {:?}",
                material_idx, material_handles
              )
            });
            mesh.set_material(Some(*handle));
          }
          None => mesh.set_material(Some(context.default_material)),
        }

        meshes.push(mesh_loader.insert(mesh));
      }
    }
    self.primitives = meshes;
    self.state = ModelLoadState::Loaded;
    self.materials = Some(Arc::downgrade(&context.resources.materials));

    Ok(())
  }

  pub fn load_from_gltf(
    &mut self,
    context: &mut Context,
    document: &Document,
    buffers: &Vec<gltf::buffer::Data>,
    images: &Vec<gltf::image::Data>,
  ) -> anyhow::Result<()> {
    match self.load_from_gltf_impl(context, document, buffers, images) {
      Err(e) => {
        self.state = ModelLoadState::Failed(format!("{:?}", e));
        Err(e)
      }
      ok => ok,
    }
  }

  pub fn iter_primitives<'a, 'b, T: ResourceStore<Mesh>>(
    &'a self,
    primitive_mgr: &'a T,
  ) -> impl Iterator<Item = Option<&'b Mesh>>
  where
    'a: 'b,
  {
    self
      .primitives
      .iter()
      .map(move |handle| primitive_mgr.get_ref(*handle))
  }
}
