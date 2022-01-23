#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}

#[repr(C)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[no_mangle]
pub extern "C" fn make_point(x: i32, y: i32) -> Point {
  Point {x, y}
}
