use wasm_bindgen::prelude::*;

use crate::game::GameState;
use legion::World;
use nalgebra_glm as glm;
use std::sync::{Arc, RwLock};

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct GameRef(Arc<RwLock<GameState>>);

#[wasm_bindgen]
impl GameRef {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    let game = GameState::new();
    Self(Arc::new(RwLock::new(game)))
  }

  #[wasm_bindgen(js_name=toString)]
  pub fn js_to_string(&self) -> String {
    format!("{:?}", self)
  }
}
