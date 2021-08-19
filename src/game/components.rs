use std::time::Duration;

use nalgebra_glm::*;

use crate::camera::Camera;
use crate::renderer_common::handle::HandleIndex;
use crate::wgpu_renderer::model::StreamingMesh;

#[derive(Clone, Default, Debug)]
pub struct GameLoopTimer {
  pub fixed_dt: Duration,
  pub per_frame_dt: Duration,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Transform3D {
  pub position: Vec3,
  pub rotation: Quat,
  pub scale: Vec3,
}

impl Default for Transform3D {
  fn default() -> Self {
    Self {
      position: Vec3::zeros(),
      rotation: Quat::identity(),
      scale: vec3(1.0, 1.0, 1.0),
    }
  }
}

impl Transform3D {
  pub fn matrix(&self) -> Mat4 {
    let rotation = quat_to_mat4(&self.rotation);
    let translation = translation(&self.position);
    let scale = scale(&Mat4::identity(), &self.scale);
    translation * rotation * scale
  }
}

#[derive(Debug, Default, Clone)]
pub struct RenderModel {
  pub mesh: Option<HandleIndex>,
  pub is_shown: bool,
}

impl RenderModel {
  pub fn new(handle: Option<HandleIndex>,
             is_shown: bool) -> Self {
    Self {
      is_shown,
      mesh: handle,
    }
  }
}


pub type CameraEntityRow = (Transform3D, Camera);

/// Resource when true, flags the system to print the scene as json
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DebugShowScene(pub bool);
