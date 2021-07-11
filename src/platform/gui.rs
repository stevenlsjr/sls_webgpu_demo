use std::default::Default;
use wgpu::{Queue, Device, RenderPass};

#[derive(Debug, Clone)]
pub struct Options {
  pub font_size: f32,
  pub hidpi_factor: f32,
}

pub struct ImguiRefMut<'a> {
  pub context: &'a mut imgui::Context,
  pub renderer: &'a mut imgui_wgpu::Renderer,
  pub frame: &'a Option<imgui::Ui<'a>>
}

pub struct ImguiRef<'a> {
  pub context: &'a  imgui::Context,
  pub renderer: &'a imgui_wgpu::Renderer,
    pub frame: &'a Option<imgui::Ui<'a>>

}

pub trait ImguiPlatform {
  fn context(&self) -> &imgui::Context;
  fn context_mut(&mut self) -> &mut imgui::Context;
  fn renderer(&self) -> &imgui_wgpu::Renderer;
  fn renderer_mut(&mut self) -> &mut imgui_wgpu::Renderer;
  fn frame(&self) -> &Option<imgui::Ui>;
  fn new_frame(&mut self);

  fn imgui_ref(&self) -> ImguiRef {
    ImguiRef {
      context: self.context(),
      renderer: self.renderer(),
      frame: self.frame()
    }
  }

  fn imgui_ref_mut(&mut self) -> ImguiRefMut;

  fn render(&mut self, queue: &Queue, device: &Device, render_pass: &mut RenderPass) {

  }
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
