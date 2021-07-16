// publically export imgui
pub use imgui;
pub use imgui_wgpu;
pub use legion;

pub use error::Error;

pub mod camera;
pub mod error;
pub mod game;
pub mod platform;
pub mod window;

pub mod wgpu_renderer;
pub use wgpu_renderer::context::Context;

#[cfg(test)]
mod tests;
