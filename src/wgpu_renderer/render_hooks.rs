use crate::{
  wgpu::{Device, Queue, RenderPass},
  Error,
};
/// Closure called per frame to render the UI.
/// Note, this is closure will need to be re-allocated per frame (since it will be
/// handling non-persistent data like imgui UI frames)
pub type OnRenderUiClosure =
  Box<dyn FnOnce(&Queue, &Device, &mut RenderPass) -> Result<(), crate::Error>>;
