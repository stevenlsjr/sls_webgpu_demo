use crate::renderer_common::{allocator::Handle, model::Model};
use std::fmt;

pub enum AssetLoadRequest<M: Model> {
  GltfModel { path: String, model_id: Handle<M> },
}

#[derive(Clone)]
pub enum AssetLoadedMessagePayload {
  GltfModel {
    model_name: String,
    documents: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
  },
}

impl fmt::Debug for AssetLoadedMessagePayload {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AssetLoadedMessagePayload::GltfModel { model_name, .. } => f
        .debug_struct("AssetLoadedMessage::GltfModel")
        .field("model_name", model_name)
        .field("documents", &format!("..."))
        .field("buffers", &format!("..."))
        .field("images", &format!("..."))
        .finish(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct AssetLoadedMessage<M: Model> {
  pub payload: AssetLoadedMessagePayload,
  pub id: Handle<M>,
}

impl<M: Model> AssetLoadedMessage<M> {
  pub fn new(id: Handle<M>, payload: AssetLoadedMessagePayload) -> Self {
    Self { id, payload }
  }
}
