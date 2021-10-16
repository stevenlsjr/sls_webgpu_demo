use std::time::Duration;

use nalgebra_glm::*;

use crate::camera::Camera;

use crate::{
  renderer_common::handle::Handle,
  wgpu_renderer::{
    model::StreamingMesh, pipeline_state::ShadingModel, uniforms::PointLightUniform,
  },
};
use serde::{
  ser::{Error, SerializeStruct},
  Deserialize, Deserializer, Serialize, Serializer,
};
use std::sync::atomic::AtomicBool;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct GameLoopTimer {
  pub fixed_dt: Duration,
  pub per_frame_dt: Duration,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Transform3D {
  position: Vec3,
  rotation: Quat,
  scale: Vec3,
}

impl Transform3D {
  pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
    Self {
      position,
      rotation,
      scale,
      ..Default::default()
    }
  }

  #[inline]
  pub fn with_position(mut self, position: Vec3) -> Self {
    self.position = position;
    self
  }

  #[inline]
  pub fn with_scale(mut self, scale: Vec3) -> Self {
    self.scale = scale;
    self
  }

  #[inline]
  pub fn with_rotation(mut self, rotation: Quat) -> Self {
    self.rotation = rotation;
    self
  }

  #[inline]
  pub fn position(&self) -> &Vec3 {
    &self.position
  }
  #[inline]
  pub fn rotation(&self) -> &Quat {
    &self.rotation
  }
  #[inline]
  pub fn scale(&self) -> &Vec3 {
    &self.scale
  }
  #[inline]
  pub fn set_position(&mut self, position: Vec3) {
    self.position = position;
  }
  #[inline]
  pub fn set_rotation(&mut self, rotation: Quat) {
    self.rotation = rotation;
  }
  #[inline]
  pub fn set_scale(&mut self, scale: Vec3) {
    self.scale = scale;
  }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderModel {
  /// Handle to the model resource
  #[serde(skip_serializing)]
  pub model: Option<Handle<StreamingMesh>>,
  /// path or identifier for the model. This will identify the model in
  /// (de)serialization
  pub model_id: String,
  /// if false, skip rendering this model
  pub is_shown: bool,
  pub shading_model: ShadingModel,
}

impl Default for RenderModel {
  fn default() -> Self {
    Self::new(None, bool::default(), String::default())
  }
}

impl RenderModel {
  pub fn new(handle: Option<Handle<StreamingMesh>>, is_shown: bool, model_id: String) -> Self {
    Self {
      is_shown,
      model: handle,
      model_id,
      shading_model: ShadingModel::Pbr,
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
