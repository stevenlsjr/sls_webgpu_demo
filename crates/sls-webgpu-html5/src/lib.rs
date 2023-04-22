use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
pub mod app;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
  // This provides better error messages in debug mode.
  // It's disabled in release mode so it doesn't bloat up the file size.
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  // Your code goes here!
  console_log::init().map_err(|e| JsValue::from_str(&e.to_string()))?;

  Ok(())
}

#[wasm_bindgen]
pub fn features() -> Vec<JsValue> {
  FEATURES_LIST.iter().map(|s| JsValue::from_str(s)).collect()
}

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &str = r#"
export type AppEventType = 'handle-input' | 'resize'
"#;

lazy_static! {
  static ref FEATURES_LIST: Vec<&'static str> = {
    let mut features = vec![];
    features.push("opengl_renderer".into());
    #[cfg(feature = "wgpu_renderer")]
    {
      features.push("wgpu_renderer".into())
    }
    features
  };
}
