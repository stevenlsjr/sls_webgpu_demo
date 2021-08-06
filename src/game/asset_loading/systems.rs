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
      AssetLoaderResource,
    },
  };
  use smallvec::SmallVec;

  #[system]
  pub fn check_message_loaded(#[resource] asset_loader: &AssetLoaderResource) {
    let messages: SmallVec<[anyhow::Result<AssetLoadedMessage>; 5]> =
      asset_loader.receiver().try_iter().collect();
    for message in messages {
      match message {
        Ok(msg) => match msg.payload {
          AssetLoadedMessagePayload::GltfModel {
            model_name,
            documents,
            buffers,
            images,
          } => {
            println!("loaded model '{}'", model_name);
          }
        },
        Err(e) => {
          log::error!("could not load asset: {:?}", e)
        }
      }
    }
  }
}
