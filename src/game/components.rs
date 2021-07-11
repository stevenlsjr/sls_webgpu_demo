use crate::camera::Camera;
use legion::*;
use nalgebra_glm::*;
use std::sync::{Arc, RwLock};
use std::time::Duration;

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

pub type CameraEntityRow = (Transform3D, Camera);

/// Resource when true, flags the system to print the scene as json
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DebugShowScene(pub bool);
