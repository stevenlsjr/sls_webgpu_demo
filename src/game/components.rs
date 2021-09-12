use std::time::Duration;

use nalgebra_glm::*;

use crate::camera::Camera;

use crate::{
  renderer_common::handle::Handle,
  wgpu_renderer::{model::StreamingMesh, uniforms::PointLightUniform},
};
use serde::{
  ser::{Error, SerializeStruct},
  Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct GameLoopTimer {
  pub fixed_dt: Duration,
  pub per_frame_dt: Duration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    let rotation = quat_to_mat4(&self.rotation.normalize());
    let translation = translation(&self.position);
    let scale = scale(&Mat4::identity(), &self.scale);
    translation * rotation * scale
  }
}

/// Represents a 3d gltf model to be loaded and rendered by the wgpu Context
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RenderModel {
  /// Handle to the model resource
  #[serde(skip_serializing, skip_deserializing)]
  pub model: Option<Handle<StreamingMesh>>,
  /// path or identifier for the model. This will identify the model in
  /// (de)serialization
  pub model_id: String,
  /// if false, skip rendering this model
  pub is_shown: bool,
}

impl RenderModel {
  pub fn new(handle: Option<Handle<StreamingMesh>>, is_shown: bool, model_id: String) -> Self {
    Self {
      is_shown,
      model: handle,
      model_id,
    }
  }
}

pub type CameraEntityRow = (Transform3D, Camera);

/// Resource when true, flags the system to print the scene as json
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct DebugShowScene(pub bool);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum LightType {
  Point,
  Directional,
  Spotlight,
}

impl Default for LightType {
  fn default() -> Self {
    Self::Point
  }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LightSource {
  pub light_type: LightType,
  pub color: Vec3,
  pub cutoff: f32,
}

impl LightSource {
  pub fn as_uniform(&self, transform: &Transform3D) -> LightSourceUniform {
    match &self.light_type {
      LightType::Point => LightSourceUniform::Point(PointLightUniform {
        position: transform.position.into(),
        _padding: 0,
        color: self.color.into(),
      }),
      _ => LightSourceUniform::Unsupported,
    }
  }
}

impl Default for LightSource {
  fn default() -> Self {
    Self {
      light_type: Default::default(),
      cutoff: f32::default(),
      color: vec3(1.0, 1.0, 1.0),
    }
  }
}

pub enum LightSourceUniform {
  Point(PointLightUniform),
  Unsupported,
}
