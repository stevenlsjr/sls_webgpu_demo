use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

use imgui::Context;
use imgui_wgpu::Renderer;
use sdl2::mouse::{MouseWheelDirection, MouseButton};

use crate::imgui::Ui;
use crate::platform::gui;
use crate::platform::gui::ImguiPlatform;

#[derive()]
pub struct ImguiSdlPlatform {
  pub renderer: imgui_wgpu::Renderer,
}

impl ImguiSdlPlatform {
  pub fn new(
    context: &mut imgui::Context,
    renderer_options: imgui_wgpu::RendererConfig,
    render_context: &crate::Context<sdl2::video::Window>,
  ) -> Result<Self, crate::Error> {
    let texture_format = render_context.adapter
      .get_swap_chain_preferred_format(&render_context.surface)
      .unwrap_or(renderer_options.texture_format);
    let renderer_options = imgui_wgpu::RendererConfig {
      texture_format,
      ..renderer_options
    };


    let renderer = imgui_wgpu::Renderer::new(context, &render_context.device,
                                             &render_context.queue, renderer_options);

    let mut platform = Self { renderer };
    {
      platform.setup_io(context.io_mut());
    }
    platform.set_size_from_window(context.io_mut(), &render_context.window);
    Ok(platform)
  }

  fn set_size_from_window(&self, io: &mut imgui::Io, window: &sdl2::video::Window) {
    let (width, height) = window.drawable_size();
    io.display_size = [width as f32, height as f32];
  }

  fn on_resize(&self, io: &mut imgui::Io, size: (u32, u32)) {
    io.display_size = [size.0 as f32, size.1 as f32];
  }


  pub fn handle_event(&self, context: &mut imgui::Context, event: &sdl2::event::Event) {
    use sdl2::event::*;
    let mut io = context.io_mut();
    match event {
      Event::Window { win_event, .. } => match win_event {
        WindowEvent::Resized(w, h) => {
          self.on_resize(io, (*w as u32, *h as u32));
        }
        _ => {}
      },
      Event::MouseButtonDown { mouse_btn, clicks, x, y, .. } => {
        if let Some(imgui_mouse_index) = sdl_mouse_button_to_imgui(*mouse_btn) {
          io.mouse_down[imgui_mouse_index as usize] = true
        }
      }
      Event::MouseButtonUp { mouse_btn, clicks, x, y, .. } => {
        if let Some(imgui_mouse_index) = sdl_mouse_button_to_imgui(*mouse_btn) {
          io.mouse_down[imgui_mouse_index as usize] = false
        }
      }
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
  pub fn update(&mut self, io: &mut imgui::Io, per_frame_dt: Duration) {
    io.update_delta_time(per_frame_dt.clone());
  }
}

impl fmt::Debug for ImguiSdlPlatform {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    f.debug_struct("ImguiSdlPlatform")
      .field("renderer", &"<...>".to_owned())
      .finish()
  }
}


impl ImguiPlatform for ImguiSdlPlatform {
  fn renderer(&self) -> &Renderer {
    &self.renderer
  }

  fn renderer_mut(&mut self) -> &mut Renderer {
    &mut self.renderer
  }

  fn setup_io(&self, io: &mut imgui::Io) {
    use imgui::Key as imKey;
    use sdl2::keyboard::Scancode;
    io.key_map[imKey::Tab as usize] = Scancode::Tab as u32;
    io.key_map[imKey::LeftArrow as usize] = Scancode::Left as u32;
    io.key_map[imKey::RightArrow as usize] = Scancode::Right as u32;
    io.key_map[imKey::UpArrow as usize] = Scancode::Up as u32;
    io.key_map[imKey::DownArrow as usize] = Scancode::Down as u32;
    io.key_map[imKey::PageUp as usize] = Scancode::PageUp as u32;
    io.key_map[imKey::PageDown as usize] = Scancode::PageDown as u32;
    io.key_map[imKey::Home as usize] = Scancode::Home as u32;
    io.key_map[imKey::End as usize] = Scancode::End as u32;
    io.key_map[imKey::Delete as usize] = Scancode::Delete as u32;
    io.key_map[imKey::Backspace as usize] = Scancode::Backspace as u32;
    io.key_map[imKey::Enter as usize] = Scancode::Return as u32;
    io.key_map[imKey::Escape as usize] = Scancode::Escape as u32;
    io.key_map[imKey::Space as usize] = Scancode::Space as u32;
    io.key_map[imKey::A as usize] = Scancode::A as u32;
    io.key_map[imKey::C as usize] = Scancode::C as u32;
    io.key_map[imKey::V as usize] = Scancode::V as u32;
    io.key_map[imKey::X as usize] = Scancode::X as u32;
    io.key_map[imKey::Y as usize] = Scancode::Y as u32;
    io.key_map[imKey::Z as usize] = Scancode::Z as u32;
  }
}

fn sdl_mouse_button_to_imgui(button: sdl2::mouse::MouseButton) -> Option<imgui::MouseButton> {
  match button {
    sdl2::mouse::MouseButton::Right => { Some(imgui::MouseButton::Right) }
    sdl2::mouse::MouseButton::Left => { Some(imgui::MouseButton::Right) }
    sdl2::mouse::MouseButton::Middle => { Some(imgui::MouseButton::Right) }
    sdl2::mouse::MouseButton::Unknown => { None }
    sdl2::mouse::MouseButton::X1 => { Some(imgui::MouseButton::Extra1) }
    sdl2::mouse::MouseButton::X2 => { Some(imgui::MouseButton::Extra2) }
  }
}
