pub mod game;
#[cfg(feature = "wasm_bindgen")]
pub mod wasm_ffi;

use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
