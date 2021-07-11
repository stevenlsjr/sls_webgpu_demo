use crate::platform::gui;
use crate::platform::gui::{ImguiPlatform, ImguiRefMut};
use imgui::Context;
use imgui_wgpu::Renderer;
use sdl2::mouse::MouseWheelDirection;
use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;
use crate::imgui::Ui;

#[derive()]
pub struct ImguiSdlPlatform<'a> {
  pub context: imgui::Context,
  pub frame: Option<imgui::Ui<'a>>,
  pub renderer: imgui_wgpu::Renderer,
}

impl<'a> ImguiSdlPlatform<'a> {
  pub fn new(
    context_options: gui::Options,
    renderer_options: imgui_wgpu::RendererConfig,
    render_context: &crate::Context<sdl2::video::Window>,
  ) -> Result<Self, crate::Error> {
    let mut context = gui::create_imgui(context_options);
    let texture_format = render_context.adapter
      .get_swap_chain_preferred_format(&render_context.surface)
      .unwrap_or(renderer_options.texture_format);
    let renderer_options = imgui_wgpu::RendererConfig {
      texture_format,
      ..renderer_options
    };

    let renderer = imgui_wgpu::Renderer::new(&mut context, &render_context.device,
                                             &render_context.queue, renderer_options);

    let mut platform = Self { renderer, context, frame: None };
    platform.set_size_from_window(&render_context.window);
    Ok(platform)
  }

  fn set_size_from_window(&mut self, window: &sdl2::video::Window) {
    let io = self.context.io_mut();
    let (width, height) = window.drawable_size();
    io.display_size = [width as f32, height as f32];
  }

  fn on_resize(&mut self, size: (u32, u32)) {
    let io = self.context.io_mut();
    io.display_size = [size.0 as f32, size.1 as f32];
  }

  pub fn handle_event(&mut self, event: &sdl2::event::Event) {
    use sdl2::event::*;
    let mut io = self.context.io_mut();
    match event {
      Event::Window { win_event, .. } => match win_event {
        WindowEvent::Resized(w, h) => {
          self.on_resize((*w as u32, *h as u32));
        }
        _ => {}
      },
      Event::MouseMotion { mousestate, .. } => {
        io.mouse_pos = [mousestate.x() as f32, mousestate.y() as f32];
      }
      Event::MouseWheel {
        which,
        x,
        y,
        direction,
        ..
      } => {
        io.mouse_wheel = *x as f32;
        if *direction == MouseWheelDirection::Flipped {
          io.mouse_wheel *= -1f32;
        }
        io.mouse_wheel_h = *y as f32;
      }
      _ => {}
    }
  }
  // update, should be run on per-frame update timer
  pub fn update(&mut self, per_frame_dt: Duration) {
    self
      .context
      .io_mut()
      .update_delta_time(per_frame_dt.clone());
  }
}

impl<'a> fmt::Debug for ImguiSdlPlatform<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("ImguiSdlPlatform")
      .field("context", &self.context)
      .field("renderer", &"<...>".to_owned())
      .finish()
  }
}

impl<'a> ImguiPlatform for ImguiSdlPlatform<'a> {
  fn context(&self) -> &Context {
    &self.context
  }

  fn context_mut(&mut self) -> &mut Context {
    &mut self.context
  }

  fn renderer(&self) -> &Renderer {
    &self.renderer
  }

  fn renderer_mut(&mut self) -> &mut Renderer {
    &mut self.renderer
  }

  fn frame(&self) -> &Option<Ui> {
    &self.frame
  }

  fn new_frame(&mut self) {
    let frame = self.context.frame();
    self.frame = Some(frame)
  }

  fn imgui_ref_mut(&mut self) -> ImguiRefMut {
    ImguiRefMut {
      renderer: &mut self.renderer,
      context: &mut self.context,
      frame: &self.frame
    }
  }
}
