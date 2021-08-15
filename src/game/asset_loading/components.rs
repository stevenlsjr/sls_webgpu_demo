use crate::renderer_common::handle::HandleIndex;
use std::{borrow::Cow, sync::Arc};

/// Loads a single gltf
pub struct RenderGltfMesh {
  path: String,
  task_handle: HandleIndex,
  meshes: Option<Vec<HandleIndex>>,
  mesh_index: usize,
}
