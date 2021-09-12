use super::context::Context;
use crate::{
  renderer_common::allocator::ResourceManager,
  wgpu::Texture,
  wgpu_renderer::{
    material::{Material, WgpuMaterial},
    mesh::Mesh,
    model::StreamingMesh,
    pipeline_state::PipelineProgram,
    textures::TextureResource,
  },
};
use legion::any;
use std::{
  borrow::{Borrow, BorrowMut},
  collections::HashMap,
  sync::{Arc, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
use uuid::Uuid;
use wgpu::RenderPipeline;
use crate::util::anyhow_from_poisoned;

#[derive(Debug)]
pub struct ResourceView<'a> {
  pub models: RwLockReadGuard<'a, ResourceManager<StreamingMesh>>,
  pub meshes: RwLockReadGuard<'a, ResourceManager<Mesh>>,
  pub materials: RwLockReadGuard<'a, ResourceManager<WgpuMaterial>>,
  pub textures: RwLockReadGuard<'a, ResourceManager<TextureResource>>,
  pub render_pipelines: RwLockReadGuard<'a, HashMap<Uuid, PipelineProgram>>,
  pub shaders: RwLockReadGuard<'a, ResourceManager<wgpu::ShaderModule>>,
}

#[derive(Debug)]
pub struct MutResourceView<'a> {
  pub models: RwLockWriteGuard<'a, ResourceManager<StreamingMesh>>,
  pub meshes: RwLockWriteGuard<'a, ResourceManager<Mesh>>,
  pub materials: RwLockWriteGuard<'a, ResourceManager<WgpuMaterial>>,
  pub textures: RwLockWriteGuard<'a, ResourceManager<TextureResource>>,
}

pub trait ReadWriteResources {
  type Error;
  fn read_resources(&self) -> Result<ResourceView, Self::Error>;
  fn write_resources(&self) -> Result<MutResourceView, Self::Error>;
}

impl ReadWriteResources for Context {
  type Error = anyhow::Error;
  fn read_resources(&self) -> Result<ResourceView, Self::Error> {
    self.resources.read_resources()
  }

  fn write_resources(&self) -> Result<MutResourceView, Self::Error> {
    self.resources.write_resources()
  }
}

#[derive(Debug, Clone, Default)]
pub struct ResourceContext {
  pub models: Arc<RwLock<ResourceManager<StreamingMesh>>>,
  pub meshes: Arc<RwLock<ResourceManager<Mesh>>>,
  pub materials: Arc<RwLock<ResourceManager<WgpuMaterial>>>,
  pub textures: Arc<RwLock<ResourceManager<TextureResource>>>,
  pub render_pipelines: Arc<RwLock<HashMap<Uuid, PipelineProgram>>>,
  pub shaders: Arc<RwLock<ResourceManager<wgpu::ShaderModule>>>,
}

impl ReadWriteResources for ResourceContext {
  type Error = anyhow::Error;

  fn read_resources(&self) -> Result<ResourceView, Self::Error> {
    let models = self.models.read().map_err(anyhow_from_poisoned)?;
    let meshes = self.meshes.read().map_err(anyhow_from_poisoned)?;

    let materials = self.materials.read().map_err(anyhow_from_poisoned)?;

    let textures = self.textures.read().map_err(anyhow_from_poisoned)?;
    let shaders = self.shaders.read().map_err(anyhow_from_poisoned)?;
    let render_pipelines = self.render_pipelines.read().map_err(anyhow_from_poisoned)?;
    Ok(ResourceView {
      models,
      meshes,
      materials,
      textures,
      render_pipelines,
      shaders,
    })
  }

  fn write_resources(&self) -> Result<MutResourceView, Self::Error> {
    let models = self.models.write().map_err(anyhow_from_poisoned)?;
    let meshes = self.meshes.write().map_err(anyhow_from_poisoned)?;

    let materials = self.materials.write().map_err(anyhow_from_poisoned)?;

    let textures = self.textures.write().map_err(anyhow_from_poisoned)?;
    Ok(MutResourceView {
      models,
      meshes,
      materials,
      textures,
    })
  }
}
