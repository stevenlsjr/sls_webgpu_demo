use wasm_bindgen::{prelude::*, JsCast};

pub fn window() -> web_sys::Window {
  web_sys::window().expect("This environment must support the window API")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
  window()
    .request_animation_frame(f.as_ref().unchecked_ref())
    .expect("could not register animation frame")
}

pub fn document() -> web_sys::Document {
  window()
    .document()
    .expect("This environment must support the document API")
}
