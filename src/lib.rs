#![feature(
  const_ptr_offset_from,
  const_maybe_uninit_as_ptr,
  const_raw_ptr_deref,
  const_refs_to_cell
)]

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
#[cfg(feature = "wgpu_renderer")]
pub mod wgpu_renderer;
pub mod window;
#[cfg(feature = "wgpu_renderer")]
pub use wgpu_renderer::context::Context;

#[cfg(feature = "wgpu_renderer")]
pub use wgpu;

#[cfg(test)]
mod tests;

#[cfg(feature = "opengl_renderer")]
pub mod gl_renderer;
pub mod math;
pub mod renderer_common;

pub use renderer_common::images::RawImageRbga;

// crate re-imports
pub use anyhow;
pub use gltf;
pub use image;
pub use nalgebra_glm;
