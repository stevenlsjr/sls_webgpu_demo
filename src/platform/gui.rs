use std::default::Default;

#[derive(Debug, Clone)]
pub struct Options {
  pub font_size: f32,
  pub hidpi_factor: f32,
}

impl Default for Options {
  fn default() -> Self {
    Self {
      font_size: 12f32,
      hidpi_factor: 1f32,
    }
  }
}

#[cfg(feature = "wgpu_renderer")]
pub trait WgpuRenderableGui {
  fn on_render(
    &mut self,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    render_pass: &mut wgpu::RenderPass,
  ) -> Result<(), String>;
}

#[cfg(feature = "wgpu_renderer")]
impl WgpuRenderableGui for () {
  fn on_render(
    &mut self,
    _queue: &wgpu::Queue,
    _device: &wgpu::Device,
    _render_pass: &mut wgpu::RenderPass,
  ) -> Result<(), String> {
    Ok(())
  }
}

#[cfg(feature = "wgpu_imgui")]
pub use self::wgpu_imgui::*;

#[cfg(feature = "wgpu_imgui")]
pub mod wgpu_imgui {
  use super::*;
  pub fn create_imgui(options: Options) -> imgui::Context {
    let mut ctx = imgui::Context::create();
    let font_size_pixels = options.font_size * options.hidpi_factor;
    ctx.io_mut().font_global_scale = 1.0 / options.hidpi_factor;
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

  ///
  /// Trait for objects and functions that define the UI
  /// for a given frame
  pub trait DrawUi {
    fn draw_ui(&self, ui: &mut imgui::Ui);
  }

  impl<F: Fn(&mut imgui::Ui)> DrawUi for F {
    fn draw_ui(&self, ui: &mut imgui::Ui) {
      self(ui)
    }
  }
}
