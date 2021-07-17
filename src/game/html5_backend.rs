use wasm_bindgen::prelude::*;
use super::input::InputBackend;
use std::collections::HashSet;
use crate::platform::keyboard::{Scancode, Keycode};

#[wasm_bindgen]
#[derive(Debug)]
pub struct Html5Backend {
  pressed_scancodes: HashSet<Scancode>,
  pressed_keycodes: HashSet<Keycode>,
}

impl Html5Backend {
  pub fn new()->Self {
    Self {
      pressed_scancodes: Default::default(),
      pressed_keycodes: Default::default(),
    }
  }
}

impl InputBackend for Html5Backend {
  fn pressed_scancodes(&self) -> &HashSet<Scancode> {
    &self.pressed_scancodes
  }

  fn pressed_keycodes(&self) -> &HashSet<Keycode> {
    &self.pressed_keycodes
  }
}
