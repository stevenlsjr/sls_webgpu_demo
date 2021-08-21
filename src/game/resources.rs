use std::collections::HashMap;

use legion::*;

use crate::{
  camera::Camera,
  game::components::{RenderModel, Transform3D},
  nalgebra_glm::{vec3, TVec2, Vec3},
  renderer_common::handle::HandleIndex,
};

#[derive(Clone, Debug, Default)]
pub struct Scene {
  pub(crate) main_camera: Option<Entity>,
}

impl Scene {
  pub fn set_main_camera(&mut self, main_camera: Option<Entity>) {
    self.main_camera = main_camera;
  }
  pub fn main_camera(&self) -> Option<Entity> {
    self.main_camera
  }

  pub fn is_main_camera(&self, entity: Option<Entity>) -> bool {
    self.main_camera == entity
  }

  pub fn main_camera_components<'a>(
    &self,
    world: &'a World,
  ) -> Result<Option<&'a Camera>, crate::Error> {
    if let Some(entity) = self.main_camera {
      let entry = world.entry_ref(entity)?;
      let camera = entry.into_component::<Camera>()?;
      return Ok(Some(camera));
    };
    Ok(None)
  }

  pub fn main_camera_components_mut<'a>(
    &self,
    world: &'a mut World,
  ) -> Result<Option<&'a mut Camera>, crate::Error> {
    if let Some(entity) = self.main_camera {
      let entry = world.entry_mut(entity)?;
      let camera = entry.into_component_mut::<Camera>()?;
      return Ok(Some(camera));
    };
    Ok(None)
  }
}

#[derive(Debug, Clone)]
pub struct CameraDisplayData {
  pub position: Vec3,
  pub forward: Vec3,
}
impl Default for CameraDisplayData {
  fn default() -> Self {
    Self {
      position: Vec3::zeros(),
      forward: vec3(0.0, 0.0, 1.0),
    }
  }
}

#[derive(Debug, Clone)]
pub struct UIDataIn {
  pub camera: CameraDisplayData,
  pub drawable_meshes: Vec<(RenderModel, Transform3D)>,
  pub mouse_pos: TVec2<i32>,
  pub mouse_delta: TVec2<i32>,
}
impl Default for UIDataIn {
  fn default() -> Self {
    Self {
      camera: Default::default(),
      drawable_meshes: Default::default(),
      mouse_pos: TVec2::zeros(),
      mouse_delta: TVec2::zeros(),
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct UIDataOut {}

#[derive(Debug, Clone, Default)]
pub struct ScreenResolution {
  pub drawable_size: (usize, usize),
  pub window_size: (usize, usize),
}

impl ScreenResolution {
  pub fn aspect_ratio(&self) -> f32 {
    (self.drawable_size.0 as f32) / (self.drawable_size.1 as f32)
  }
}

#[derive(Debug, Clone, Default)]
pub struct MeshLookup(pub HashMap<String, HandleIndex>);
