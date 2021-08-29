use legion::Entity;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub enum AssetLoadRequest {
  GltfModel {
    path: String,
    uuid: Uuid,
    entity: Option<Entity>,
  },
}

impl AssetLoadRequest {
  pub fn uuid(&self) -> &Uuid {
    match self {
      AssetLoadRequest::GltfModel { uuid, .. } => uuid,
    }
  }
  pub fn entity(&self) -> Option<Entity> {
    match self {
      AssetLoadRequest::GltfModel { entity, .. } => *entity,
    }
  }
}

#[derive(Clone)]
pub enum AssetLoadedMessagePayload {
  GltfModel {
    uuid: Uuid,
    model_name: String,
    documents: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
  },
}

impl fmt::Debug for AssetLoadedMessagePayload {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AssetLoadedMessagePayload::GltfModel {
        model_name, uuid, ..
      } => f
        .debug_struct("AssetLoadedMessage::GltfModel")
        .field("model_name", model_name)
        .field("uuid", uuid)
        .finish(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct AssetLoadedMessage {
  pub payload: AssetLoadedMessagePayload,
  pub entity: Option<Entity>,
  pub id: Uuid,
}

impl AssetLoadedMessage {
  pub fn new(id: Uuid, payload: AssetLoadedMessagePayload, entity: Option<Entity>) -> Self {
    Self { payload, entity, id }
  }
}
