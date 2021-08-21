use std::{
  borrow::BorrowMut,
  path::Path,
  sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, RwLock, RwLockWriteGuard,
  },
  thread::spawn,
};

use anyhow::anyhow;
use crossbeam::channel::{unbounded, Receiver, Sender, TryIter};

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
  renderer_common::{
    allocator::{ResourceManager, SparseArrayAllocator},
    handle::{Handle, HandleIndex},
  },
  wgpu_renderer::model::{ModelLoadState, StreamingMesh},
};

use super::asset_load_message::AssetLoadedMessage;
use std::collections::HashMap;
use uuid::Uuid;

type ChannelType = anyhow::Result<AssetLoadedMessage>;

pub struct MultithreadedAssetLoaderQueue {
  sender: Sender<ChannelType>,
  receiver: Receiver<ChannelType>,
  open_requests: HashMap<Uuid, AssetLoadRequest>,
}

impl AssetLoaderQueue for MultithreadedAssetLoaderQueue {
  fn submit_task(&mut self, request: AssetLoadRequest) {
    let cloned = request.clone();
    let sender = self.sender.clone();

    match request {
      AssetLoadRequest::GltfModel { path, uuid, entity } => {
        self.open_requests.insert(uuid, cloned);
        rayon::spawn(move || {
          if let Err(e) = Self::load_gltf_model(uuid.clone(), path, &sender) {
            sender.send(Err(e));
          }
        });
      }
    };
  }

  fn poll_completed(&mut self) -> Vec<anyhow::Result<AssetLoadedMessage>> {
    self.receiver.try_iter().collect()
  }
}

impl MultithreadedAssetLoaderQueue {
  pub fn new() -> Self {
    let (sender, receiver) = unbounded();
    Self {
      sender,
      receiver,
      open_requests: Default::default(),
    }
  }
  fn load_gltf_model(uuid: Uuid, path: String, sender: &Sender<ChannelType>) -> anyhow::Result<()> {
    let (documents, buffers, images) = gltf::import(&path)?;
    sender.send(Ok(AssetLoadedMessage::new(
      uuid,
      AssetLoadedMessagePayload::GltfModel {
        uuid,
        model_name: "".to_string(),
        documents,
        buffers,
        images,
      },
    )))?;
    Ok(())
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

  fn load_model(
    &self,
    models: &mut ResourceManager<StreamingMesh>,
    handle: Handle<StreamingMesh>,
    documents: &gltf::Document,
    buffers: &[gltf::buffer::Data],
  ) -> anyhow::Result<()> {
    let mut model = models.mut_ref(handle)?;
    // model.load_from_gltf()
    Ok(())
  }
}
