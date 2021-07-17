use wasm_bindgen::prelude::*;
use js_sys;
use sls_webgpu::game::{GameState, CreateGameParams, html5_backend::Html5Backend};
use sls_webgpu::game::input::InputBackend;
use std::ops::DerefMut;
use std::cell::UnsafeCell;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "'handle-input'")]
    pub type InputEventType;
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SlsWgpuDemo {
  game_state: GameState,

  on_handle_input: Option<js_sys::Function>
}

#[wasm_bindgen]
impl SlsWgpuDemo {
  #[wasm_bindgen(constructor)]
  pub fn new_js() -> Result<SlsWgpuDemo, JsValue> {
    let app = SlsWgpuDemo::new()?;
    Ok(app)
  }

  #[wasm_bindgen]
  pub fn run(&self) -> Result<(), JsValue> {

    Ok(())
  }

  #[wasm_bindgen]
  pub fn on(mut self,event: &str, callback: js_sys::Function) -> Result<SlsWgpuDemo, JsValue> {
    match event {
      "handle-input" => {
        self.on_handle_input = Some(callback)
      }
      _ => return Err(format!("event type '{}' not supported", event).into())
    };
    Ok(self)
  }


}

impl SlsWgpuDemo {
  pub fn new() -> Result<Self, String> {
    let input_backend = Html5Backend::new();
    let game_state = GameState::new(CreateGameParams { input_backend: Box::new(input_backend) });
    Ok(Self { game_state , on_handle_input: None})
  }
}