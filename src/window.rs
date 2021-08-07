use raw_window_handle::HasRawWindowHandle;

pub trait AsWindow: HasRawWindowHandle {
  fn size(&self) -> (u32, u32);
  fn set_size(&mut self, size: (u32, u32));

  ///
  /// If value is true, cursor should be hidden and mouse delta should
  /// be tracked even if cursor is outside the window boundary.
  /// If false, use normal mouse behavior
  fn set_relative_mouse_mode(&mut self, value: bool) {}
}

#[cfg(feature = "sdl2")]
mod sdl_impl {
  use crate::window::AsWindow;
  use log::warn;
  use sdl2::video::Window;

  impl AsWindow for Window {
    #[inline]
    fn size(&self) -> (u32, u32) {
      self.vulkan_drawable_size()
    }

    #[inline]
    fn set_size(&mut self, size: (u32, u32)) {
      if let Err(e) = self.set_size(size.0, size.1) {
        warn!("could not set size: {:?}", e);
      }
    }

    fn set_relative_mouse_mode(&mut self, value: bool) {
      self.subsystem().sdl().mouse().set_relative_mouse_mode(true)
    }
  }
}

#[cfg(feature = "sdl2")]
pub use sdl_impl::*;

#[cfg(feature = "html5_backend")]
mod html5_backend {
  use super::*;
  use raw_window_handle::{RawWindowHandle, RawWindowHandle::Web};
  use std::{
    ops::Deref,
    sync::atomic::{AtomicU32, Ordering},
  };
  use wasm_bindgen::JsValue;
  use web_sys::HtmlCanvasElement;

  // using an atomic counter, which is overkill for
  // now since wgpu-rs on the browser is single-threaded
  // right now
  static SLS_WGPU_ID_COUNTER: AtomicU32 = AtomicU32::new(0);

  #[derive(Clone, Debug)]
  pub struct HtmlWindowImpl<'a> {
    canvas: &'a web_sys::HtmlCanvasElement,
  }

  impl<'a> HtmlWindowImpl<'a> {
    // wgpu queries the dom for canvas with
    // data attribute data-raw-handle of a
    // given id to implement raw window handles
    const SLS_WGPU_ID_KEY: &'static str = "rawHandle";

    pub fn new(canvas: &'a HtmlCanvasElement) -> Self {
      Self { canvas }
    }

    fn id_lazy(&self) -> Result<u32, JsValue> {
      let dataset = self.canvas.dataset();
      let value: Option<String> = dataset.get(Self::SLS_WGPU_ID_KEY);
      match value {
        // if the id is already bound to data-raw-handle,
        // return the id
        Some(s) => s
          .parse::<u32>()
          .map_err(|e| js_sys::Error::new(&e.to_string()).into()),
        // otherwise, set the canvas raw-handle to the current count,
        // and increment the id counter
        None => {
          let count = SLS_WGPU_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
          dataset.set(Self::SLS_WGPU_ID_KEY, &count.to_string())?;
          Ok(count)
        }
      }
    }
  }
  // Window trait implementations
  unsafe impl<'a> HasRawWindowHandle for HtmlWindowImpl<'a> {
    fn raw_window_handle(&self) -> RawWindowHandle {
      use raw_window_handle::{web::WebHandle, RawWindowHandle};
      let id = self
        .id_lazy()
        .unwrap_or_else(|e| panic!("fatal error: cannot get window handle from canvas {:?}", e));
      RawWindowHandle::Web(WebHandle {
        id,
        ..WebHandle::empty()
      })
    }
  }

  impl<'a> AsWindow for HtmlWindowImpl<'a> {
    fn size(&self) -> (u32, u32) {
      let width = self.canvas.width() as u32;
      let height = self.canvas.height() as u32;
      (width, height)
    }

    fn set_size(&mut self, size: (u32, u32)) {
      self.canvas.set_width(size.0);
      self.canvas.set_height(size.1);
    }
  }

  // implementing type conversions from canvasElement
  pub trait AsHtmlWindowWrapper {
    fn as_gamewindow(&self) -> HtmlWindowImpl;
  }
  impl AsHtmlWindowWrapper for HtmlCanvasElement {
    fn as_gamewindow(&self) -> HtmlWindowImpl {
      HtmlWindowImpl { canvas: self }
    }
  }
}

#[cfg(feature = "html5_backend")]
pub use html5_backend::*;
