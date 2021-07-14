use std::default::Default;
use wgpu::{Queue, Device, RenderPass};

#[derive(Debug, Clone)]
pub struct Options {
  pub font_size: f32,
  pub hidpi_factor: f32,
}


pub trait ImguiPlatform {
  fn renderer(&self) -> &imgui_wgpu::Renderer;
  fn renderer_mut(&mut self) -> &mut imgui_wgpu::Renderer;
  fn setup_io(&self, io: &mut imgui::Io) {}
}

impl Default for Options {
  fn default() -> Self {
    Self {
      font_size: 12f32,
      hidpi_factor: 1f32,
    }
  }
}

pub fn create_imgui(options: Options) -> imgui::Context {
  let mut ctx = imgui::Context::create();
  let font_size_pixels = options.font_size * options.hidpi_factor;
  ctx.io_mut().font_global_scale = (1.0 / options.hidpi_factor);
  ctx.fonts().add_font(&[imgui::FontSource::DefaultFontData {
    config: Some(imgui::FontConfig {
      oversample_h: 1,
      pixel_snap_h: true,
      size_pixels: font_size_pixels,
      ..Default::default()
    }),
  }]);
  ctx
}
