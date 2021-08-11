use super::asset_load_message::AssetLoadedMessage;
#[cfg(feature = "wgpu_renderer")]
use crate::wgpu_renderer::Context;
use crate::{
  anyhow::Error,
  game::{
    asset_loading::{
      asset_load_message::{AssetLoadRequest, AssetLoadedMessagePayload},
      resources::AssetLoaderQueue,
    },
    GameState,
  },
  renderer_common::allocator::{Handle, ResourceManager, SparseArrayAllocator},
};
use anyhow::anyhow;
use crossbeam::channel::{unbounded, Receiver, Sender, TryIter};
use std::{
  path::Path,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock,
  },
  thread::spawn,
};
use std::borrow::BorrowMut;
use crate::wgpu_renderer::model::{StreamingMesh, ModelLoadState};
use std::sync::RwLockWriteGuard;

type ChannelType = (Handle, anyhow::Result<AssetLoadedMessage>);

pub struct MultithreadedAssetLoaderQueue {
  sender: Sender<ChannelType>,
  receiver: Receiver<ChannelType>,
  task_ids: ResourceManager<AssetLoadRequest>,
}

impl MultithreadedAssetLoaderQueue {
  pub fn new() -> Self {
    let (sender, receiver) = unbounded();
    Self {
      sender,
      receiver,
      task_ids: ResourceManager::default(),
    }
  }

  pub fn sender(&self) -> &Sender<ChannelType> {
    &self.sender
  }
  pub fn receiver(&self) -> &Receiver<ChannelType> {
    &self.receiver
  }
  pub fn receiver_mut(&mut self) -> &mut Receiver<ChannelType> {
    &mut self.receiver
  }

  #[cfg(feature = "wgpu_renderer")]
  pub fn wgpu_poll_completed(
    &mut self,
    context: &mut Context,
  ) -> anyhow::Result<()> {
    for (handle, message) in self.receiver.try_iter() {
      let request = match self.task_ids.get_ref(handle) {
        Ok(r) => r,
        Err(_) => continue,
      };

      let message = message.and_then(|msg| match msg {
        AssetLoadedMessage {
          payload:
          AssetLoadedMessagePayload::GltfModel {
            model_name,
            documents,
            buffers,
            images,
          },
          ..
        } => {
          context.streaming_models
            .write().map_err(|e| anyhow!( e.to_string()))
            .and_then(|mut models| {
              self.load_model(&mut *models, handle, &documents, &buffers)
            })
        }
      });
      // handle errors where asset failed to load
      if let Err(e) = message {
        match request {
          AssetLoadRequest::GltfModel { path, model_id } => {
            let mut models: RwLockWriteGuard<ResourceManager<StreamingMesh>> = context.streaming_models.write().unwrap();
            match models.mut_ref(*model_id) {
              Ok(mut m) => {
                m.state = ModelLoadState::Failed(e.to_string());
              }
              Err(ae) => log::error!("could not update model which failed to load: {:?}, {:?}", e, ae)
            }
          }
        }
      }
      // remove task id from allocator

      if let Err(e) = self.task_ids.remove(handle) {
        log::debug!("{:?}", e);
      }
    }
    Ok(())
  }

  fn load_model(&self, models: &mut ResourceManager<StreamingMesh>, handle: Handle,
                documents: &gltf::Document,
                buffers: &[gltf::buffer::Data]) -> anyhow::Result<()> {
    let mut model = models.mut_ref(handle)?;
    model.load_gltf_geometry(documents, buffers)
  }
}
