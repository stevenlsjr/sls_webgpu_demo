use crate::{renderer_common::handle::Handle, wgpu_renderer::model::StreamingMesh, Context};
use nalgebra_glm::Mat4;
use std::sync::{Arc, RwLock, Weak};

use super::uniforms::PointLightUniform;
use crate::game::components::{LightSource, LightType, Transform3D};

#[derive(Debug)]
pub enum DrawCommand {
  Model {
    handle: Handle<StreamingMesh>,
    transform: Mat4,
  },
}

#[derive(Debug)]
pub struct WgpuFrame {
  context: Weak<RwLock<Context>>,
  point_lights: RwLock<Vec<PointLightUniform>>,
}

impl WgpuFrame {
  pub fn new(context: &Arc<RwLock<Context>>) -> Self {
    Self {
      context: Arc::downgrade(context),
      point_lights: Default::default(),
    }
  }
  pub fn push_light<I>(&mut self, light: &LightSource, transform: &Transform3D) {
    let mut point_lights = self.point_lights.write().unwrap();
    match light.light_type {
      LightType::Point => {
        let uniform = PointLightUniform {
          position: transform.position.into(),
          _padding: 0,
          color: light.color.into(),
        };
        point_lights.push(uniform);
      }
      _ => {}
    }
  }

  pub fn clear(&mut self) {
    // self.draw_list.clear();
    // self.lights.clear();
  }
}
