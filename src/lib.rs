pub mod camera;
pub mod context;
pub mod error;
pub mod game;
pub mod geometry;
pub mod mesh;
pub mod platform;
pub mod uniforms;
pub mod window;

// publically export imgui
pub use imgui;
pub use imgui_wgpu;

pub use legion;

pub use context::Context;
pub use error::Error;
#[cfg(test)]
mod tests;
