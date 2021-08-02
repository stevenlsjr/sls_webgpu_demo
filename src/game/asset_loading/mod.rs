// asset loading has to happen asynchronously. In native
// environments, we can use multithreading.
// for html, we can either use web workers
// or browser event-based IO

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

pub mod asset_load_message;
#[cfg(not(target_arch = "wasm32"))]
mod native;
#[cfg(not(target_arch = "wasm32"))]
pub mod systems;
