pub mod keyboard;
pub mod mouse;
pub use mouse::{MouseButton, MouseButtonState};

#[cfg(test)]
pub mod tests;

pub mod gui;
#[cfg(all(feature = "sdl2_backend", feature = "wgpu_imgui"))]
pub mod sdl2_backend;

pub mod draw_ui;
#[cfg(target_arch = "wasm32")]
pub mod html5;
