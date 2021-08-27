use crate::{
  game::asset_loading::asset_load_message::{AssetLoadedMessage, AssetLoadedMessagePayload},
  renderer_common::allocator::Handle,
};

use super::asset_load_message::AssetLoadRequest;

///
///

pub trait AssetLoaderQueue {
  type PollComplete: IntoIterator<Item = anyhow::Result<AssetLoadedMessage>>;

  /// Submits an asset loading request, returning the handle to the object to request
  fn submit_task(&mut self, request: AssetLoadRequest) -> Handle;
  /// Returns an iterator of completed load requests
  fn poll_completed(&mut self) -> Self::PollComplete;
}
