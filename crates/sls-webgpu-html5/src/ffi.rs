use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS_DEFS: &'static str = r#"
type RenderBackendType = 'WebGL' | 'WebGPU';
interface CreateAppOptions {
  renderer: RenderBackendType;
}

"#;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(typescript_type = "CreateAppOptions")]
  pub type CreateAppOptionsJs;

  #[wasm_bindgen(typescript_type = "RenderBackendType")]
  pub type RenderBackendTypeJs;
  #[wasm_bindgen(method, getter, js_name = "renderer")]
  pub fn renderer(this: &CreateAppOptionsJs) -> RenderBackendTypeJs;

}
