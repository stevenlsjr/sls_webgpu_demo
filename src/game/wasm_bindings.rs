use crate::game::GameState;
use std::sync::{Arc, RwLock, Weak};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GameStateView {
  game_state: Weak<RwLock<GameState>>,
}

impl From<&Arc<RwLock<GameState>>> for GameStateView {
  fn from(arc: &Arc<RwLock<GameState>>) -> Self {
    Self {
      game_state: Arc::downgrade(arc),
    }
  }
}

#[wasm_bindgen]
impl GameStateView {}
