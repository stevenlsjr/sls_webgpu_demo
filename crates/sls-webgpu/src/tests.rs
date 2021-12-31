use super::camera;
use nalgebra_glm::*;

#[derive(Debug)]
struct Suite {
  camera: camera::Camera,
}

impl Suite {
  fn new() -> Self {
    Self {
      camera: camera::Camera::default(),
    }
  }
}

#[test]
fn test_camera_view() {
  let mut suite = Suite::new();
  suite.camera.position = vec3(1f32, 0f32, 1f32);
  let view = suite.camera.view();
  #[rustfmt::skip]
  let expected = Mat4::new(
    1.0, 0.0, 0.0, 1.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 1.0,
    0.0, 0.0, 0.0, 1.0,
  );
  for i in 0..16 {
    assert!(
      f32::abs(view.data[i]) - f32::abs(expected.data[i]) <= f32::EPSILON,
      "{} != {}, i={}",
      &view,
      &expected,
      i
    );
  }
}
