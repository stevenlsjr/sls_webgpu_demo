use std::fmt;
use std::fmt::Formatter;
use std::time::Duration;

use crate::imgui::{Io, Ui};
use crate::platform::gui;
use crate::platform::gui::ImguiPlatform;
use imgui::Context;
use imgui_wgpu::Renderer;
use sdl2::event::Event;
use sdl2::mouse::MouseState;
use sdl2::{
  mouse::{MouseButton, MouseWheelDirection},
  video::Window,
};

#[derive(Debug)]
pub struct ImguiSdlPlatform {
  ignore_mouse: bool,
  ignore_keyboard: bool,
  mouse_press: [bool; 6],
}

impl ImguiSdlPlatform {
  pub fn new(context: &mut imgui::Context) -> Result<Self, crate::Error> {
    let mut platform = Self {
      ignore_keyboard: true,
      ignore_mouse: true,
      mouse_press: [false; 6],
    };
    {
      platform.setup_io(context.io_mut());
    }
    Ok(platform)
  }

  fn setup_size_from_window(&self, io: &mut imgui::Io, window: &sdl2::video::Window) {
    let (width, height) = window.drawable_size();
    io.display_size = [width as f32, height as f32];
  }

  pub fn on_start(&mut self, io: &mut imgui::Io, window: &Window) {
    self.setup_size_from_window(io, window);
  }

  pub fn on_resize(&self, io: &mut imgui::Io, size: (u32, u32)) {
    io.display_size = [size.0 as f32, size.1 as f32];
  }

  pub fn handle_event(&mut self, context: &mut imgui::Context, event: &sdl2::event::Event) {
    use sdl2::event::*;
    let mut io = context.io_mut();
    match event {
      Event::Window { win_event, .. } => match win_event {
        WindowEvent::Resized(w, h) => {
          self.on_resize(io, (*w as u32, *h as u32));
        }
        _ => {}
      },
      Event::MouseButtonDown {
        mouse_btn,
        clicks,
        x,
        y,
        ..
      } => {
        if let Some(imgui_mouse_index) = sdl_mouse_button_to_imgui(*mouse_btn) {
          io.mouse_down[imgui_mouse_index as usize] = true
        }
      }
      Event::MouseButtonUp {
        mouse_btn,
        clicks,
        x,
        y,
        ..
      } => {
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
      Event::KeyDown {
        keycode,
        scancode,
        keymod,
        repeat,
        ..
      } => {
        self.map_keymod(io, *keymod);
        if let Some(scancode) = scancode {
          io.keys_down[*scancode as usize] = true;
        }
      }
      Event::KeyUp {
        keycode,
        scancode,
        keymod,
        ..
      } => {
        self.map_keymod(io, *keymod);

        if let Some(scancode) = scancode {
          io.keys_down[*scancode as usize] = false;
        }
      }
      _ => {}
    }
  }
  // update, should be run on per-frame update timer
  pub fn update(&mut self, io: &mut imgui::Io, per_frame_dt: Duration) {
    io.update_delta_time(per_frame_dt.clone());
  }

  fn map_keymod(&mut self, io: &mut Io, keymod: sdl2::keyboard::Mod) {
    use sdl2::keyboard::Mod;
    io.key_ctrl = keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD);
    io.key_alt = keymod.intersects(Mod::RALTMOD | Mod::LALTMOD);
    io.key_shift = keymod.intersects(Mod::RSHIFTMOD | Mod::LSHIFTMOD);
  }

  pub fn prepare_frame(&mut self, io: &mut Io, window: &Window, mouse_state: &MouseState) {
    let window_size = window.size();
    let drawable_size = window.drawable_size();
    let mouse_util = window.subsystem().sdl().mouse();

    io.display_size = [window_size.0 as f32, window_size.1 as f32];
    io.display_framebuffer_scale = [
      (drawable_size.0 as f32) / (window_size.0 as f32),
      (drawable_size.1 as f32) / (window_size.1 as f32),
    ];

    io.mouse_down = [
      self.mouse_press[0] || mouse_state.left(),
      self.mouse_press[1] || mouse_state.right(),
      self.mouse_press[2] || mouse_state.middle(),
      self.mouse_press[3] || mouse_state.x1(),
      self.mouse_press[4] || mouse_state.x2(),
    ];
    self.mouse_press = [false; 6];
    let any_mouse_down = io.mouse_down.iter().any(|&b| b);
    mouse_util.capture(any_mouse_down);

    io.mouse_pos = [mouse_state.x() as f32, mouse_state.y() as f32];

    self.ignore_mouse = io.want_capture_mouse;
    self.ignore_keyboard = io.want_capture_keyboard;
  }
  pub fn prepare_render(&mut self, ui: &imgui::Ui, window: &Window) {}

  ///
  /// Returns true if a given SDL event should be ignored
  pub fn ignore_event(&self, event: &Event) -> bool {
    match *event {
      Event::KeyDown { .. }
      | Event::KeyUp { .. }
      | Event::TextEditing { .. }
      | Event::TextInput { .. } => self.ignore_keyboard,
      Event::MouseMotion { .. }
      | Event::MouseButtonDown { .. }
      | Event::MouseButtonUp { .. }
      | Event::MouseWheel { .. }
      | Event::FingerDown { .. }
      | Event::FingerUp { .. }
      | Event::FingerMotion { .. }
      | Event::DollarGesture { .. }
      | Event::DollarRecord { .. }
      | Event::MultiGesture { .. } => self.ignore_mouse,
      _ => false,
    }
  }
}

impl ImguiPlatform for ImguiSdlPlatform {
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
    sdl2::mouse::MouseButton::Right => Some(imgui::MouseButton::Right),
    sdl2::mouse::MouseButton::Left => Some(imgui::MouseButton::Right),
    sdl2::mouse::MouseButton::Middle => Some(imgui::MouseButton::Right),
    sdl2::mouse::MouseButton::Unknown => None,
    sdl2::mouse::MouseButton::X1 => Some(imgui::MouseButton::Extra1),
    sdl2::mouse::MouseButton::X2 => Some(imgui::MouseButton::Extra2),
  }
}
