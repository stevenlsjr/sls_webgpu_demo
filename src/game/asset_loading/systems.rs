use super::asset_load_message::AssetLoadRequest;
use legion::*;

use crate::{
  anyhow::Error,
  game::asset_loading::{
    asset_load_message::{AssetLoadedMessage, AssetLoadedMessagePayload},
    resources::AssetLoaderQueue,
  },
  legion::world::{EntityAccessError, EntryMut},
  wgpu_renderer::model::ModelLoadState,
};
use legion::world::SubWorld;
#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

#[cfg(not(target_arch = "wasm32"))]
mod native {
  use super::*;
  use crate::{
    anyhow::Error,
    game::asset_loading::{
      asset_load_message::{AssetLoadedMessage, AssetLoadedMessagePayload},
      MultithreadedAssetLoaderQueue,
    },
    renderer_common::handle::HandleIndex,
  };
  use smallvec::SmallVec;
}
