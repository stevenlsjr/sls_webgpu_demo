use legion::{systems::CommandBuffer, *};
use log::*;
use nalgebra_glm as glm;

use crate::{
  camera::Camera,
  game::{
    components::{CameraEntityRow, GameLoopTimer, Transform3D},
    resources::{Scene, UIDataIn},
  },
};

pub mod camera_systems;
pub mod model_systems;

use super::components::RenderModel;
use crate::game::{
  asset_loading::resources::AssetLoaderQueue, input::InputResource, resources::ScreenResolution,
};
pub use camera_systems::*;
use legion::world::SubWorld;

#[system]
pub fn fixed_update_logging(#[resource] game_loop: &GameLoopTimer) {
  debug!("fixed time update! {:?}", game_loop.fixed_dt)
}

#[system]
pub fn per_frame_logging(#[resource] game_loop: &GameLoopTimer) {
  debug!("update! {:?}", game_loop.per_frame_dt)
}
/**
 * This is executed when the scene is initialized
 */
#[system]
pub fn setup_scene(
  #[resource] scene: &mut Scene,
  #[resource] resolution: &ScreenResolution,
  #[resource] _assets: &Box<dyn AssetLoaderQueue>,
  command_buffer: &mut CommandBuffer,
) {
  let mut main_camera: CameraEntityRow = (
    Transform3D::default(),
    Camera::new(resolution.aspect_ratio()),
  );
  main_camera.0.position = glm::vec3(0f32, 0f32, 3f32);

  let main_camera_entity = command_buffer.push(main_camera);
  scene.main_camera = Some(main_camera_entity);

  // trigger load asset tasks

  // assets.spawn_load_gltf_model("assets/sheen-chair/SheenChair.glb", "chair");
}

#[system(for_each)]
#[read_component(Entity)]
#[read_component(Camera)]
#[read_component(Transform3D)]
pub fn write_camera_ui_data(
  #[resource] scene: &Scene,
  #[resource] ui_data: &mut UIDataIn,
  entity: &Entity,
  camera: &Camera,
  transform: &Transform3D,
) {
  if Some(entity) == scene.main_camera.as_ref() {
    ui_data.camera.position = transform.position.clone_owned();
    ui_data.camera.forward = camera.front.clone_owned();
  }
}

#[system]
#[read_component(RenderModel)]
#[read_component(Transform3D)]
pub fn write_renderable_ui_data(
  #[resource] _scene: &Scene,
  #[resource] input: &InputResource,
  #[resource] ui_data: &mut UIDataIn,
  world: &SubWorld,
) {
  let mut query = <(&RenderModel, &Transform3D)>::query();
  ui_data.drawable_meshes = Vec::new();
  ui_data.mouse_pos = input.backend.current_mouse_pos;
  ui_data.mouse_delta = input.backend.mouse_delta();
  for (render_model, xform) in query.iter(world) {
    ui_data
      .drawable_meshes
      .push((render_model.clone(), xform.clone()))
  }
}
