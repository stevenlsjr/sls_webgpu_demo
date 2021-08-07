
use wasm_bindgen::prelude::*;
use std::sync::{Weak, RwLock, Arc};
use crate::game::GameState;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GameStateView {
  game_state: Weak<RwLock<GameState>>
}

impl From<&Arc<RwLock<GameState>>> for GameStateView {
  fn from(arc: &Arc<RwLock<GameState>>) -> Self {
    Self{
      game_state: Arc::downgrade(arc)
    }
  }
}

#[wasm_bindgen]
impl GameStateView {

}

