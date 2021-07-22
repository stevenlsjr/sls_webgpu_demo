use std::process::{Command};

fn main() {
  Command::new("make").spawn().unwrap();
}
