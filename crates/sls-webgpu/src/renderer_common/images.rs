use wasm_bindgen::prelude::*;
///
/// A structure containing raw rgba data.
/// Useful when the image is decoded to pixel data
/// externally, such as on the web via browser apis
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RawImageRbga {
  pub(crate) data: Vec<u8>,
  pub(crate) size: (usize, usize),
}

#[wasm_bindgen]
impl RawImageRbga {
  #[wasm_bindgen(getter, js_name = "data")]
  pub fn data_js(&self) -> Vec<u8> {
    self.data.clone()
  }

  #[wasm_bindgen(getter, js_name = "width")]
  pub fn width(&self) -> usize {
    self.size.0
  }
  #[wasm_bindgen(setter, js_name = "width")]
  pub fn set_width(&mut self, width: usize) {
    self.size.0 = width;
  }

  #[wasm_bindgen(getter, js_name = "height")]
  pub fn height(&self) -> usize {
    self.size.1
  }
  #[wasm_bindgen(setter, js_name = "height")]
  pub fn set_height(&mut self, height: usize) {
    self.size.1 = height;
  }

  #[wasm_bindgen(constructor)]
  pub fn new_js(data: &[u8], width: usize, height: usize) -> Self {
    Self {
      data: data.to_vec(),
      size: (width, height),
    }
  }
}

impl RawImageRbga {
  pub fn data(&self) -> &Vec<u8> {
    &self.data
  }
  pub fn size(&self) -> (usize, usize) {
    self.size
  }
}

#[cfg(target_arch = "wasm32")]
pub use html5_backend::*;
#[cfg(target_arch = "wasm32")]
mod html5_backend {}
