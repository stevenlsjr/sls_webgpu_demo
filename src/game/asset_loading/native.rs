use super::asset_load_message::AssetLoadedMessage;
use crate::game::asset_loading::asset_load_message::AssetLoadedMessagePayload;
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::{
  path::Path,
  sync::atomic::{AtomicUsize, Ordering},
};

pub struct AssetLoaderResource {
  sender: Sender<anyhow::Result<AssetLoadedMessage>>,
  receiver: Receiver<anyhow::Result<AssetLoadedMessage>>,
  pub next_id: AtomicUsize,
}

impl AssetLoaderResource {
  pub fn new() -> Self {
    let (sender, receiver) = unbounded();
    Self {
      sender,
      receiver,
      next_id: AtomicUsize::new(0),
    }
  }

  pub fn sender(&self) -> &Sender<anyhow::Result<AssetLoadedMessage>> {
    &self.sender
  }
  pub fn receiver(&self) -> &Receiver<anyhow::Result<AssetLoadedMessage>> {
    &self.receiver
  }
  pub fn receiver_mut(&mut self) -> &mut Receiver<anyhow::Result<AssetLoadedMessage>> {
    &mut self.receiver
  }
  /// loads a gltf model on a global worker thread
  pub fn spawn_load_gltf_model<P: AsRef<Path>>(&self, path: P, model_name: &str) -> usize {
    let id = self.next_id.fetch_add(1, Ordering::Relaxed);
    let sender = self.sender.clone();
    let path = path.as_ref().to_owned();
    let model_name = model_name.to_owned();
    rayon::spawn(move || {
      let res = gltf::import(path).and_then(|(documents, buffers, images)| {
        Ok(AssetLoadedMessage::new(
          id,
          AssetLoadedMessagePayload::GltfModel {
            model_name,
            documents,
            buffers,
            images,
          },
        ))
      });
      sender.send(res.map_err(|e| e.into()));
    });
    id
  }
}
