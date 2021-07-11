use crate::platform::gui;
use crate::platform::gui::ImguiPlatform;
use imgui::Context;
use imgui_wgpu::Renderer;
use sdl2::mouse::MouseWheelDirection;
use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

#[derive()]
pub struct ImguiSdlPlatform {
  pub context: imgui::Context,
  pub renderer: imgui_wgpu::Renderer,
}

impl ImguiSdlPlatform {
  pub fn new(
    context_options: gui::Options,
    renderer_options: imgui_wgpu::RendererConfig,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    window: &sdl2::video::Window,
  ) -> Result<Self, crate::Error> {
    let mut context = gui::create_imgui(context_options);

    let renderer = imgui_wgpu::Renderer::new(&mut context, device, queue, renderer_options);

    let mut platform = Self { renderer, context };
    platform.set_size_from_window(window);
    Ok(platform)
  }

  fn set_size_from_window(&mut self, window: &sdl2::video::Window) {
    let io = self.context.io_mut();
    let (width, height) = window.drawable_size();
    io.display_size = [width as f32, height as f32];
  }

  pub fn handle_event(&mut self, event: &sdl2::event::Event) {
    use sdl2::event::*;
    let mut io = self.context.io_mut();
    match event {
      Event::Window { win_event, .. } => match win_event {
        WindowEvent::Resized(w, h) => {}
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

impl fmt::Debug for ImguiSdlPlatform {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("ImguiSdlPlatform")
      .field("context", &self.context)
      .field("renderer", &"<...>".to_owned())
      .finish()
  }
}

impl ImguiPlatform for ImguiSdlPlatform {
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
}
