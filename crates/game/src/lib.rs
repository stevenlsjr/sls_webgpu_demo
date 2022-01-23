pub mod game;
#[cfg(target_arch = "wasm32")]
pub mod wasm_ffi;

use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
