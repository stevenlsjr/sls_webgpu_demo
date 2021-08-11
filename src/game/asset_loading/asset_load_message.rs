use crate::renderer_common::allocator::Handle;
use std::fmt;

pub enum AssetLoadRequest {
  GltfModel { path: String, model_id: Handle },
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
pub struct AssetLoadedMessage {
  pub payload: AssetLoadedMessagePayload,
  pub id: Handle,
}

impl AssetLoadedMessage {
  pub fn new(id: Handle, payload: AssetLoadedMessagePayload) -> Self {
    Self { id, payload }
  }
}

