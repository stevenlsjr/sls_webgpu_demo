use crate::platform::keyboard::{Keycode, Scancode};
use downcast_rs::{impl_downcast, Downcast};
use std::fmt::{Debug, Formatter};
// use nalgebra_glm::*;
use std::collections::HashSet;

pub trait InputBackend: Downcast {
  fn pressed_scancodes(&self) -> &HashSet<Scancode>;
  fn pressed_keycodes(&self) -> &HashSet<Keycode>;
}
impl_downcast!(InputBackend);

#[derive(Debug, Default, Clone)]
pub struct DummyInputBackend {
  pressed_scancodes: HashSet<Scancode>,
  pressed_keycodes: HashSet<Keycode>,
}
impl InputBackend for DummyInputBackend {
  fn pressed_scancodes(&self) -> &HashSet<Scancode> {
    &self.pressed_scancodes
  }

  fn pressed_keycodes(&self) -> &HashSet<Keycode> {
    &self.pressed_keycodes
  }
}

pub struct InputResource {
  pub backend: Box<dyn InputBackend>,
}

impl InputResource {
  pub fn new(backend: Box<dyn InputBackend>) -> Self {
    Self { backend }
  }
}

impl Debug for InputResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("InputResource")
      .field("backend", &format!("{:p}", self.backend))
      .finish()
  }
}

#[cfg(feature = "sdl2")]
mod sdl2_input {
  use crate::game::input::*;
  use crate::platform::keyboard::{Keycode, Scancode};
  use std::collections::HashSet;

  #[derive(Clone, Default)]
  pub struct Sdl2Input {
    pressed_scancodes: HashSet<Scancode>,
    pressed_keycodes: HashSet<Keycode>,
    mouse_state: Option<sdl2::mouse::MouseState>,
    relative_mouse_state: Option<sdl2::mouse::RelativeMouseState>,
  }

  impl Sdl2Input {
    pub fn new() -> Self {
      Self::default()
    }

    pub fn sync_input(&mut self, _sdl: &sdl2::Sdl, event_pump: &sdl2::EventPump) {
      let key_state = event_pump.keyboard_state();
      self.mouse_state = Some(event_pump.mouse_state().clone());
      self.relative_mouse_state = Some(event_pump.relative_mouse_state().clone());

      self.pressed_keycodes.clear();
      self.pressed_scancodes.clear();

      for code in key_state.pressed_scancodes() {
        self.pressed_scancodes.insert(code.into());
        if let Some(keycode) = sdl2::keyboard::Keycode::from_scancode(code) {
          self.pressed_keycodes.insert(keycode.into());
        }
      }

      // info!("pressed keys: {:?}", self.pressed_keycodes);
    }
  }

  impl InputBackend for Sdl2Input {
    fn pressed_scancodes(&self) -> &HashSet<Scancode> {
      &self.pressed_scancodes
    }

    fn pressed_keycodes(&self) -> &HashSet<Keycode> {
      &self.pressed_keycodes
    }
  }
}

#[cfg(feature = "sdl2")]
pub use sdl2_input::*;
