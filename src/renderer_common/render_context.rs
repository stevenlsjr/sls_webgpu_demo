use crate::{
  game::GameState,
  nalgebra_glm::Vec4,
  wgpu_renderer::{material::RenderMaterial, textures::TextureResource, uniforms::Uniforms},
};
use downcast_rs::*;
use std::{fmt::Debug, ops::Range};

/// Common trait for generic rendering backends
pub trait RenderContext: Debug + Downcast {
  fn set_clear_color(&mut self, color: Vec4) {}
  fn on_render(&mut self, game: &mut GameState) -> Result<(), crate::Error>;
}

impl_downcast!(RenderContext);

pub trait DrawModel<'a, 'b>
where
  'b: 'a,
{
  type Model;
  type Material;
  type Uniforms;
  fn draw_model(
    &mut self,
    model: &'b Self::Model,
    material: &'a Self::Material,
    uniforms: &'a Self::Uniforms,
  );
  fn draw_model_instanced(
    &mut self,
    model: &'b Self::Model,
    material: &'a Self::Material,
    uniforms: &'a Self::Uniforms,
    instances: Range<u32>,
  );
}
