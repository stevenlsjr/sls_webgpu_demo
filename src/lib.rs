// publically export imgui
#[cfg(feature = "wgpu_imgui")]
pub use imgui;
#[cfg(feature = "wgpu_imgui")]
pub use imgui_wgpu;
pub use legion;

pub use error::Error;

pub mod camera;
pub mod error;
pub mod game;
pub mod platform;
pub mod window;
#[cfg(feature="wgpu_renderer")]
pub mod wgpu_renderer;
#[cfg(feature="wgpu_renderer")]
pub use wgpu_renderer::context::Context;

#[cfg(test)]
mod tests;
