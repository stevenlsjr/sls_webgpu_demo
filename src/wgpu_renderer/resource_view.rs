use std::sync::{RwLockWriteGuard, RwLockReadGuard, PoisonError};
use crate::renderer_common::allocator::ResourceManager;
use std::borrow::{Borrow, BorrowMut};
use crate::wgpu_renderer::model::StreamingMesh;
use crate::wgpu_renderer::material::Material;
use crate::wgpu::Texture;
use crate::wgpu_renderer::textures::TextureResource;
use super::context::Context;
use crate::wgpu_renderer::mesh::Mesh;

#[derive(Debug)]
pub struct ResourceView<'a> {
  pub models: RwLockReadGuard<'a, ResourceManager<StreamingMesh>>,
  pub meshes: RwLockReadGuard<'a, ResourceManager<Mesh>>,
  pub materials: RwLockReadGuard<'a, ResourceManager<Material>>,
  pub textures: RwLockReadGuard<'a, ResourceManager<TextureResource>>,
}

#[derive(Debug)]
pub struct MutResourceView<'a> {
  pub models: RwLockWriteGuard<'a, ResourceManager<StreamingMesh>>,
  pub meshes: RwLockWriteGuard<'a, ResourceManager<Mesh>>,
  pub materials: RwLockWriteGuard<'a, ResourceManager<Material>>,
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
    let models = self.streaming_models.read();
    let meshes = self.meshes.read();
    let materials = self.materials.read();
    let textures = self.textures.read();
    match (models, meshes, materials, textures) {
      (Ok(models),
        Ok(meshes),
        Ok(materials),
        Ok(textures), ) => {
        Ok(ResourceView {
          models,
          meshes,
          materials,
          textures,
        })
      }

      (models,
        meshes,
        materials,
        textures, ) => {
        Err(anyhow::anyhow!("a read lock is poisoned! models: {:?} meshes: {:?} materials: {:?} textures: {:?}", models,
        meshes, materials, textures))
      }
    }
  }

  fn write_resources(&self) -> Result<MutResourceView, Self::Error> {
    let models = self.streaming_models.write();
    let meshes = self.meshes.write();
    let materials = self.materials.write();
    let textures = self.textures.write();
    match (models, meshes, materials, textures) {
      (Ok(models),
        Ok(meshes),
        Ok(materials),
        Ok(textures), ) => {
        Ok(MutResourceView {
          models,
          meshes,
          materials,
          textures,
        })
      }

      (models,
        meshes,
        materials,
        textures, ) => {
        Err(anyhow::anyhow!("a write lock is poisoned! models: {:?} meshes: {:?} materials: {:?} textures: {:?}", models,
        meshes, materials, textures))
      }
    }
  }
}