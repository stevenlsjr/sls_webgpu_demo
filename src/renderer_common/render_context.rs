use crate::{game::GameState, nalgebra_glm::Vec4, renderer_common::handle::ResourceStore};
use downcast_rs::*;
use std::{fmt::Debug, ops::Range};

/// Common trait for generic rendering backends
pub trait RenderContext: Debug + Downcast {
  fn set_clear_color(&mut self, _color: Vec4) {}
  fn on_render(&mut self, game: &mut GameState) -> Result<(), crate::Error>;
}

impl_downcast!(RenderContext);

pub trait DrawModel<'a, 'b>
where
  'b: 'a,
{
  type Mesh;
  type Model;
  type Material;
  type Uniforms;
  fn draw_model<MeshStore: ResourceStore<Self::Mesh>>(
    &mut self,
    mesh_mgr: &'b MeshStore,
    model: &'b Self::Model,
    uniforms: &'a Self::Uniforms,
  );
  fn draw_model_instanced<MeshStore: ResourceStore<Self::Mesh>>(
    &mut self,
    mesh_mgr: &'b MeshStore,
    model: &'b Self::Model,
    uniforms: &'a Self::Uniforms,
    instances: Range<u32>,
  );

  fn draw_mesh(
    &mut self,
    model: &'b Self::Mesh,
    material: &'a Self::Material,
    uniforms: &'a Self::Uniforms,
  );
  fn draw_mesh_instanced(
    &mut self,
    model: &'b Self::Mesh,
    material: &'a Self::Material,
    uniforms: &'a Self::Uniforms,
    instances: Range<u32>,
  );
}
