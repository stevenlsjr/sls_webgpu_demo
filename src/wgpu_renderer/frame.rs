use crate::{renderer_common::handle::Handle, wgpu_renderer::model::StreamingMesh};
use nalgebra_glm::Mat4;

#[derive(Debug)]
pub enum DrawCommand {
  Model {
    handle: Handle<StreamingMesh>,
    transform: Mat4,
  },
}

#[derive(Debug)]
pub struct WgpuFrame {
  draw_list: Vec<DrawCommand>,
}

impl WgpuFrame {
  pub fn new() -> Self {
    Self {
      draw_list: Vec::new(),
    }
  }

  pub fn clear(&mut self) {
    self.draw_list.clear();
  }
}
