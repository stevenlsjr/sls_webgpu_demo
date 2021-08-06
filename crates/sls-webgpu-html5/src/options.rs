use wasm_bindgen::prelude::*;
use serde::*;
use super::ffi;
use sls_webgpu::error::Error::Create;
use crate::ffi::{CreateAppOptionsJs, RenderBackendTypeJs};
use sls_webgpu::wgpu;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum RendererBackend {
  WebGL,
  WebGPU,
}

#[derive(Debug)]
pub struct CreateAppOptions {
  pub renderer: RendererBackend,
  pub webgpu_context: Option<(wgpu::Device)>,
}

impl CreateAppOptions {
  pub fn from_js(options: CreateAppOptionsJs) -> Result<Self, js_sys::Error> {
    let default_opts = Self::default();
    let renderer = options.renderer().into_serde::<RendererBackend>.unwrap_or(RendererBackend::WebGL);
    let wgpu_ctx_js: Option<ffi::WgpuContext> = options.wgpu_context();
    let webgpu_context = wgpu_ctx_js.map(|ctx|

    )
    todo!()
}
}

impl Default for CreateAppOptions {
  fn default() -> Self {
    Self {
      renderer: RendererBackend::WebGL
    }
  }
}

#[wasm_bindgen]
pub fn log_options(options: ffi::CreateAppOptionsJs) {
  let opts: CreateAppOptions = JsValue::from(options).into_serde().unwrap();
  log::info!("options {:?}", opts);
}