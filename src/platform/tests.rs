use crate::platform::mouse::*;

#[test]
fn test_mouse_state_contains() {
  let cases = &[
    (
      MouseButton::Left,
      MouseButtonState::new(MouseButton::Left as u32 | MouseButton::Right as u32),
      true,
    ),
    (
      MouseButton::Left,
      MouseButtonState::new(MouseButton::Right as u32),
      false,
    ),
    (
      MouseButton::Right,
      MouseButtonState::new(MouseButton::Right as u32),
      true,
    ),
    (
      MouseButton::Right,
      MouseButtonState::new(
        MouseButton::Right as u32 | MouseButton::X1 as u32 | MouseButton::Unknown as u32,
      ),
      true,
    ),
  ];
  for (button, mask, expected) in cases {
    assert_eq!(mask.contains(*button), *expected);
  }
}

#[test]
fn test_insert() {
  use rand::prelude::*;
  let mut rng = rand::thread_rng();
  let buttons = &[
    MouseButton::Left,
    MouseButton::Middle,
    MouseButton::Right,
    MouseButton::X1,
    MouseButton::X2,
  ];
  for button in buttons {
    let mut state = MouseButtonState::new(rng.gen::<u32>());
    state.insert(*button);
    assert!(state.contains(*button));
  }
}

#[cfg(feature = "sdl2_backend")]
#[test]
fn test_sdl_into() {
  use sdl2::mouse::MouseButton as SdlMouse;
  let buttons = &[
    (SdlMouse::Right, MouseButton::Right),
    (SdlMouse::Left, MouseButton::Left),
    (SdlMouse::Middle, MouseButton::Middle),
    (SdlMouse::X1, MouseButton::X1),
    (SdlMouse::X2, MouseButton::X2),
  ];
  for (sdl_button, button) in buttons {
    assert_eq!(*sdl_button.into(), *button)
  }
}
