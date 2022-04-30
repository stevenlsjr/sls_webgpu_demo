use std::collections::HashMap;

use crossbeam::channel::{unbounded, Receiver, Sender};
use uuid::Uuid;

#[cfg(feature = "wgpu_renderer")]
use crate::{
  game::asset_loading::{
    asset_load_message::{AssetLoadRequest, AssetLoadedMessagePayload},
    resources::AssetLoaderQueue,
  },
  renderer_common::{allocator::ResourceManager, handle::Handle},
};

use super::asset_load_message::AssetLoadedMessage;
use crate::wgpu_renderer::model::StreamingMesh;

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
      AssetLoadRequest::GltfModel {
        path,
        uuid,
        entity: _,
      } => {
        self.open_requests.insert(uuid, cloned);
        rayon::spawn(move || {
          if let Err(e) = Self::load_gltf_model(uuid, path, &sender) {
            sender.send(Err(e)).unwrap();
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
      None,
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
    _documents: &gltf::Document,
    _buffers: &[gltf::buffer::Data],
  ) -> anyhow::Result<()> {
    let _model = models.try_mut_ref(handle)?;
    // model.load_from_gltf()
    Ok(())
  }
}
