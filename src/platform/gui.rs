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

#[cfg(feature = "wasm_imgui")]
pub use self::wasm_imgui::*;
#[cfg(feature = "wasm_imgui")]
mod wasm_imgui {
  use super::*;
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