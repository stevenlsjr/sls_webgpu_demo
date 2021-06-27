use nalgebra_glm::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    #[inline]
    pub fn view(&self) -> Mat4 {
        look_at_rh(&self.eye, &self.target, &self.up)
    }

    #[inline]
    pub fn projection(&self) -> Mat4 {
        perspective(self.aspect, self.fovy, self.znear, self.zfar)
    }

    pub fn view_projection(&self) -> Mat4 {
        let view = self.view();
        let proj = self.projection();
        proj * view
    }
}

impl Default for Camera {
  fn default() -> Self {
    Self {
      eye: (0.0, 1.0, 2.0).into(),
      target: (0.0, 0.0, 0.0).into(),
      up: (0.0, 1.0, 0.0).into(),
      aspect: 1.0,
      fovy: 45.0,
      znear: 0.1,
      zfar: 100.0
    }
  }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);
