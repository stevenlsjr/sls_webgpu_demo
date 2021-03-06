cause std::{env, process::Command};

fn main() -> Result<(), String> {
  let is_windows = env::var("CARGO_CFG_TARGET_FAMILY") == Ok("windows".to_owned());

  let path_sep = if is_windows { '\\' } else { '/' };
  let mut out_dir = env::var("OUT_DIR").unwrap();

  if !out_dir.ends_with(path_sep) {
    out_dir.push(path_sep);
  }
  if is_windows {
    out_dir.push('\\');
  }
  let ps = Command::new("make").status().unwrap();
  if ps.code() != Some(0) {
    return Err(format!("make failed with status {:?}", ps.code()));
  }
  Ok(())
}
