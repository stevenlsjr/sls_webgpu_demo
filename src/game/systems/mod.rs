use legion::{systems::CommandBuffer, *};
use log::*;
use nalgebra_glm as glm;

use crate::{
  camera::Camera,
  game::{
    components::{CameraEntityRow, GameLoopTimer, Transform3D},
    resources::{Scene, UIDataIn},
  },
  Context,
};

pub mod camera_systems;
pub mod model_systems;
pub mod renderer;

use super::components::RenderModel;
use crate::{
  error::Error::Render,
  game::{
    asset_loading::resources::AssetLoaderQueue,
    components::{LightSource, LightType},
    input::InputResource,
    resources::{MeshLookup, ScreenResolution},
  },
  nalgebra_glm::vec3,
  renderer_common::{allocator::ResourceManager, handle::Handle},
  wgpu_renderer::{
    model::{ModelLoadState, StreamingMesh},
    resource_view::ReadWriteResources,
  },
};
pub use camera_systems::*;
use legion::world::SubWorld;
use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

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
  #[resource] context: &Arc<RwLock<Context>>,
  command_buffer: &mut CommandBuffer,
) {
  let context_lock = context.read().unwrap();
  let mut models = context_lock.resources.models.write().unwrap();

  let mut main_camera: CameraEntityRow = (
    Transform3D::default(),
    Camera::new(resolution.aspect_ratio()),
  );
  main_camera.0.set_position(glm::vec3(0f32, 0f32, 3f32));

  let main_camera_entity = command_buffer.push(main_camera);
  scene.main_camera = Some(main_camera_entity);

  let light_entity = (
    {
      let mut tx = Transform3D::default();
      tx.set_position(vec3(2.0, 4.0, 1.0));
      tx
    },
    LightSource {
      light_type: LightType::Point,
      color: vec3(1.0, 1.0, 0.0),
      ..Default::default()
    },
    RenderModel {
      model: Some(models.insert(StreamingMesh {
        path: ":CUBE:".to_string(),
        mesh_index: 0,
        state: ModelLoadState::NotLoaded,
        primitives: vec![],
        materials: None,
      })),
      model_id: ":CUBE:".to_string(),
      is_shown: true,
      shading_model: Default::default(),
    },
  );
  command_buffer.push(light_entity);

  // trigger load asset tasks

  // assets.spawn_load_gltf_model("assets/sheen-chair/SheenChair.glb", "chair");
}

#[system(for_each)]
#[write_component(RenderModel)]
pub fn load_procedural_meshes(
  #[resource] context: &Arc<RwLock<Context>>,
  #[resource] models: &Arc<RwLock<ResourceManager<StreamingMesh>>>,
  #[resource] mesh_lookup: &mut MeshLookup,
  model: &mut RenderModel,
) {
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
    ui_data.camera.position = transform.position().clone_owned();
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
