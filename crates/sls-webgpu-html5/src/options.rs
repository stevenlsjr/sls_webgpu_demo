use crate::ffi::{CreateAppOptionsJs};
use serde::*;
// use wasm_bindgen::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum RendererBackend {
  WebGL,
  WebGPU,
}


#[derive(Debug)]
pub struct CreateAppOptions {
  pub renderer: RendererBackend,
}

impl CreateAppOptions {
  pub fn from_js(options: CreateAppOptionsJs) -> Result<Self, js_sys::Error> {
    let renderer = options
      .renderer()
      .into_serde::<RendererBackend>()
      .unwrap_or(RendererBackend::WebGL);

    Ok(Self {
      renderer,
    })
  }
}

impl Default for CreateAppOptions {
  fn default() -> Self {
    Self {
      renderer: RendererBackend::WebGL,
    }
  }
}
