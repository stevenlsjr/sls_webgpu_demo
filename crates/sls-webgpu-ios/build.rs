use std::env;
fn main() {
  let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap().to_string();
  let config = cbindgen::Config::from_root_or_default(&crate_dir.clone());

  cbindgen::generate_with_config(&crate_dir, config).unwrap();
}
