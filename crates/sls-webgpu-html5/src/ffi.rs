use wasm_bindgen::prelude::*;
use sls_webgpu::wgpu;


#[wasm_bindgen(typescript_custom_section)]
const TS_DEFS: &'static str = r#"
type RenderBackendType = 'WebGL' | 'WebGPU';
interface CreateAppOptions {
  renderer: RenderBackendType;
}
"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type="CreateAppOptions")]
  pub type CreateAppOptionsJs;

  #[wasm_bindgen(typescript_type="RenderBackendType")]

  pub type RenderBackendTypeJs;
  #[wasm_bindgen(method, getter, js_name="renderer")]
  pub fn renderer(this: &CreateAppOptionsJs) -> RenderBackendTypeJs;

  #[wasm_bindgen(method, getter, js_name="renderer")]
  pub fn wgpu_context(this: &CreateAppOptionsJs) -> Option<WgpuContext>;

  #[wasm_bindgen(typescript_type="WasmContext")]
  pub type WgpuContext;

  #[wasm_bindgen(method, getter)]
  pub fn device(this: &WgpuContext) -> Option<wgpu::Device>;

  #[wasm_bindgen(method, getter)]
  pub fn adapter(this: &WgpuContext) -> Option<wgpu::Adapter>;
  #[wasm_bindgen(method, getter)]
  pub fn instance(this: &WgpuContext) -> Option<wgpu::Instance>;

}

