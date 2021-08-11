use crate::renderer_common::allocator::Handle;
use std::{borrow::Cow, sync::Arc};

/// Loads a single gltf
pub struct RenderGltfMesh {
  path: String,
  task_handle: Handle,
  meshes: Option<Vec<Handle>>,
  mesh_index: usize,
}
