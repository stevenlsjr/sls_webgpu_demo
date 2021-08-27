use crate::{renderer_common::handle::HandleIndex, wgpu_renderer::model::ModelLoadState};
use std::{
  borrow::Cow,
  sync::{Arc, RwLock},
};
use uuid::Uuid;

pub type GltfImportOut = (
  gltf::Document,
  Vec<gltf::buffer::Data>,
  Vec<gltf::image::Data>,
);
