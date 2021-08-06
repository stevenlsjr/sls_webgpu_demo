use crate::{game::GameState, nalgebra_glm::Vec4};
use downcast_rs::*;
use std::fmt::Debug;

/// Common trait for generic rendering backends
pub trait RenderContext: Debug + Downcast {
  fn set_clear_color(&mut self, color: Vec4) {}
  fn on_render(&mut self, game: &mut GameState) -> Result<(), crate::Error>;
}

impl_downcast!(RenderContext);
