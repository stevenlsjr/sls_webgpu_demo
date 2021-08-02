use std::fmt;

#[derive(Clone)]
pub enum AssetLoadedMessage {
  GltfModel {
    model_name: String,
    documents: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
  }
}

impl fmt::Debug for AssetLoadedMessage {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AssetLoadedMessage::GltfModel { model_name, ..} => {
        f.debug_struct("AssetLoadedMessage::GltfModel")
          .field("model_name", model_name)
          .field("documents", &format!("..."))
          .field("buffers", &format!("..."))
          .field("images", &format!("..."))
          .finish()
      }
    }
  }
}