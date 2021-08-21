use crate::platform::keyboard::*;
use downcast_rs::{impl_downcast, Downcast};
use std::fmt::{Debug, Formatter};
// use nalgebra_glm::*;
use std::collections::HashSet;

pub trait InputBackend: Downcast {
  fn pressed_scancodes(&self) -> &HashSet<Scancode>;
  fn pressed_keycodes(&self) -> &HashSet<Keycode>;
}
impl_downcast!(InputBackend);

#[derive(Debug, Clone)]
pub struct InputState {
  pressed_scancodes: HashSet<Scancode>,
  pressed_keycodes: HashSet<Keycode>,
  mouse_state: MouseButtonState,
  relative_mouse_state: MouseButtonState,
  mouse_delta: TVec2<i32>,
  keymod: KeyMod,

  pub(crate) current_mouse_pos: TVec2<i32>,
  pub(crate) previous_frame_mouse_pos: Option<TVec2<i32>>,
}

impl Default for InputState {
  fn default() -> Self {
    Self {
      pressed_keycodes: Default::default(),
      pressed_scancodes: Default::default(),
      mouse_state: MouseButtonState::new(0),
      relative_mouse_state: MouseButtonState::new(0),
      current_mouse_pos: vec2(0, 0),
      mouse_delta: vec2(0, 0),
      previous_frame_mouse_pos: Some(vec2(0, 0)),
      keymod: KeyMod::empty(),
    }
  }
}

impl InputState {
  pub fn on_start_frame(&mut self) {
    self.previous_frame_mouse_pos = Some(self.current_mouse_pos);
    self.mouse_delta = TVec2::zeros();
  }

  pub fn mouse_delta(&self) -> TVec2<i32> {
    self.mouse_delta
  }

  #[inline]
  pub fn mouse_state(&self) -> MouseButtonState {
    self.mouse_state
  }
}

impl InputBackend for InputState {
  fn pressed_scancodes(&self) -> &HashSet<Scancode> {
    &self.pressed_scancodes
  }

  fn pressed_keycodes(&self) -> &HashSet<Keycode> {
    &self.pressed_keycodes
  }
}

pub struct InputResource {
  pub backend: InputState,
}

impl InputResource {
  pub fn new(backend: InputState) -> Self {
    Self { backend }
  }
  pub fn is_mouselook_enabled(&self) -> bool {
    self.backend.mouse_state.contains(MouseButton::Middle)
      || self.backend.keymod.contains(KeyMod::LALTMOD)
  }
}

impl Debug for InputResource {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("InputResource")
      .field("backend", &format!("{:?}", self.backend))
      .finish()
  }
}

#[cfg(feature = "sdl2")]
mod sdl2_input {
  use super::*;
  use crate::platform::mouse::*;
  use sdl2::{event::Event, video::Window};

  impl InputState {
    pub fn new() -> Self {
      Self::default()
    }

    pub fn sync_input(&mut self, sdl: &sdl2::Sdl, event_pump: &sdl2::EventPump) {
      let key_state = event_pump.keyboard_state();
      self.pressed_keycodes.clear();
      self.pressed_scancodes.clear();
      let keymods = sdl.keyboard().mod_state();

      for code in key_state.pressed_scancodes() {
        self.pressed_scancodes.insert(code.into());
        if let Some(keycode) = sdl2::keyboard::Keycode::from_scancode(code) {
          self.pressed_keycodes.insert(keycode.into());
        }
      }

      // info!("pressed keys: {:?}", self.pressed_keycodes);
    }

    pub fn handle_sdl_event(&mut self, event: &sdl2::event::Event, window: &Window) {
      let mouse_util = window.subsystem().sdl().mouse();
      match event {
        Event::Quit { .. } => {}
        Event::AppWillEnterBackground { .. } => {}
        Event::AppDidEnterBackground { .. } => {}
        Event::AppWillEnterForeground { .. } => {}
        Event::AppDidEnterForeground { .. } => {}
        Event::KeyDown { keymod, .. } => self.keymod = (*keymod).into(),
        Event::KeyUp { keymod, .. } => self.keymod = (*keymod).into(),
        Event::TextEditing { .. } => {}
        Event::TextInput { .. } => {}
        Event::MouseMotion {
          x,
          y,
          xrel,
          yrel,
          mousestate,
          ..
        } => {
          self.mouse_state = MouseButtonState::new(mousestate.to_sdl_state());
          self.current_mouse_pos = vec2(*x, *y);
          self.mouse_delta = vec2(*xrel, *yrel);
        }
        Event::MouseButtonDown { mouse_btn, .. } => {
          let btn: MouseButton = (*mouse_btn).into();
          self.mouse_state.mask |= (btn as u32);
        }
        Event::MouseButtonUp { mouse_btn, .. } => {
          let btn: MouseButton = (*mouse_btn).into();
          self.mouse_state.mask &= !(btn as u32);
        }
        Event::MouseWheel { .. } => {}
        Event::ControllerAxisMotion { .. } => {}
        Event::ControllerButtonDown { .. } => {}
        Event::ControllerButtonUp { .. } => {}
        Event::FingerDown { .. } => {}
        Event::FingerUp { .. } => {}
        Event::FingerMotion { .. } => {}
        Event::DollarGesture { .. } => {}
        Event::DollarRecord { .. } => {}
        Event::MultiGesture { .. } => {}
        Event::ClipboardUpdate { .. } => {}
        Event::DropFile { .. } => {}
        Event::DropText { .. } => {}
        Event::DropBegin { .. } => {}
        Event::DropComplete { .. } => {}
        _ => {}
      }
    }
  }
}

use crate::{
  nalgebra_glm::{vec2, TVec2},
  platform::{
    keyboard::KeyMod,
    mouse::{MouseButton, MouseButtonState},
  },
};
#[cfg(feature = "sdl2")]
pub use sdl2_input::*;
