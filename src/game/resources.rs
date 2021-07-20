use crate::camera::Camera;
use legion::*;

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
