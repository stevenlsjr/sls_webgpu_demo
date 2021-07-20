// use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MouseButton {
  Unknown = 0x1,
  Left = 0x1 << 1,
  Middle = 0x1 << 2,
  Right = 0x1 << 3,
  X1 = 0x1 << 4,
  X2 = 0x1 << 5,
}

#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct MouseButtonState {
  mask: u32,
}

impl MouseButtonState {
  pub fn new(mask: u32) -> Self {
    Self { mask }
  }
  pub fn clear(&mut self) {
    self.mask = 0x0;
  }
  pub fn insert(&mut self, button: MouseButton) {
    self.mask |= button as u32;
  }
  pub fn remove(&mut self, button: MouseButton) {
    self.mask &= !(button as u32);
  }

  pub fn contains(&self, button: MouseButton) -> bool {
    self.mask & (button as u32) == (button as u32)
  }

  #[inline]
  pub fn set_mask(&mut self, mask: u32) {
    self.mask = mask;
  }
  #[inline]
  pub fn mask(&self) -> u32 {
    self.mask
  }
}

#[cfg(feature = "sdl2_backend")]
mod sdl_backend {
  use crate::platform::mouse::MouseButton;
  use sdl2::mouse::MouseButton as SdlMouseButton;

  impl From<SdlMouseButton> for MouseButton {
    fn from(button: SdlMouseButton) -> Self {
      match button {
        SdlMouseButton::Unknown => MouseButton::Unknown,
        SdlMouseButton::Left => MouseButton::Left,
        SdlMouseButton::Middle => MouseButton::Middle,
        SdlMouseButton::Right => MouseButton::Right,
        SdlMouseButton::X1 => MouseButton::X1,
        SdlMouseButton::X2 => MouseButton::X2,
      }
    }
  }

  impl Into<SdlMouseButton> for MouseButton {
    fn into(self) -> SdlMouseButton {
      match self {
        MouseButton::Unknown => SdlMouseButton::Unknown,
        MouseButton::Left => SdlMouseButton::Left,
        MouseButton::Middle => SdlMouseButton::Middle,
        MouseButton::Right => SdlMouseButton::Right,
        MouseButton::X1 => SdlMouseButton::X1,
        MouseButton::X2 => SdlMouseButton::X2,
      }
    }
  }
}

#[cfg(feature = "sdl2_backend")]
pub use sdl_backend::*;
