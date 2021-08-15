use crate::game::asset_loading::asset_load_message::{AssetLoadedMessage, AssetLoadedMessagePayload};
use crate::renderer_common::handle::HandleIndex;

use super::asset_load_message::AssetLoadRequest;

///
///

pub trait AssetLoaderQueue {
  type PollComplete: IntoIterator<Item = anyhow::Result<AssetLoadedMessage>>;

  /// Submits an asset loading request, returning the handle to the object to request
  fn submit_task(&mut self, request: AssetLoadRequest) -> HandleIndex;
  /// Returns an iterator of completed load requests
  fn poll_completed(&mut self) -> Self::PollComplete;
}

