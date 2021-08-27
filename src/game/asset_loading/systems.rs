#[allow(unused_imports)]
use legion::*;

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
    renderer_common::allocator::Handle,
  };
  use smallvec::SmallVec;
}
