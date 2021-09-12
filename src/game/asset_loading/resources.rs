use crate::{
  game::asset_loading::asset_load_message::{AssetLoadedMessage, AssetLoadedMessagePayload},
  renderer_common::handle::HandleIndex,
};

use super::asset_load_message::AssetLoadRequest;
use crate::{renderer_common::handle::Handle, wgpu_renderer::model::StreamingMesh};

///
///
///
pub trait AssetLoaderQueue {
  /// Submits an asset loading request, returning the handle to the object to request
  fn submit_task(&mut self, request: AssetLoadRequest);
  /// Returns an iterator of completed load requests
  fn poll_completed(&mut self) -> Vec<anyhow::Result<AssetLoadedMessage>>;
}

#[derive(Debug)]
pub struct MainSceneAssets {
  pub avocado_model: Handle<StreamingMesh>,
  pub avocado_model_path: String,
}
