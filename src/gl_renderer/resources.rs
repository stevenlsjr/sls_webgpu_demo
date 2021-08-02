use super::util;
use crate::Error;
/// RAII-safe opengl resources
use glow::HasContext;
use smallvec::{smallvec, SmallVec};
use std::{
  cell::RefCell,
  error, fmt,
  ops::Deref,
  rc::{Rc, Weak},
};

const DEFAULT_SHADER_CAPACITY: usize = 5;

#[derive(Debug)]
pub struct GlResourceError {
  pub reason: String,
  pub gl_code: Option<u32>,
}

impl GlResourceError {
  pub fn new(reason: String, gl_code: Option<u32>) -> Self {
    Self { reason, gl_code }
  }
}

impl fmt::Display for GlResourceError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "GlResourceError {}, gl error code: {:?}",
      &self.reason, self.gl_code
    )
  }
}

impl error::Error for GlResourceError {}

impl From<GlResourceError> for crate::Error {
  fn from(e: GlResourceError) -> Self {
    let e = Self::from_error(Box::new(e));
    e
  }
}

impl From<Vec<(u32, String)>> for GlResourceError {
  fn from(v: Vec<(u32, String)>) -> Self {
    let last_code = v.first().map(|tuple| tuple.0);
    let reasons: Vec<String> = v
      .into_iter()
      .map(|(a, b)| format!("{}, {}", a, b))
      .collect();
    let reason = reasons.join(", ");
    Self {
      gl_code: last_code,
      reason,
    }
  }
}

#[derive(Debug)]
pub struct Shaders<GL: HasContext> {
  gl: Weak<RefCell<GL>>,
  shaders: SmallVec<[<GL as HasContext>::Shader; DEFAULT_SHADER_CAPACITY]>,
}

impl<GL: HasContext> Shaders<GL> {
  pub fn create(gl: &Rc<RefCell<GL>>, shader_types: &[u32]) -> Result<Self, GlResourceError> {
    let mut resource = Self {
      gl: Rc::downgrade(gl),
      shaders: smallvec![],
    };
    {
      let gl = gl.borrow();
      for &shader_type in shader_types {
        unsafe {
          let shader = gl
            .create_shader(shader_type)
            .map_err(|e| GlResourceError::new(e, None))?;
          resource.shaders.push(shader);
          util::check_errors(gl.deref())?;
        }
      }
    }
    Ok(resource)
  }
  pub fn shaders(&self) -> &[<GL as HasContext>::Shader] {
    return &self.shaders;
  }
}

impl<GL: HasContext> Drop for Shaders<GL> {
  fn drop(&mut self) {
    let gl = self.gl.upgrade();
    if let Some(gl) = gl {
      let gl = gl.borrow();
      for shader in self.shaders.iter() {
        unsafe {
          gl.delete_shader(*shader);
        }
      }
    }
    self.shaders.clear();
  }
}

#[derive(Debug)]
pub struct Program<GL: HasContext> {
  gl: Weak<RefCell<GL>>,
  program: <GL as HasContext>::Program,
}

impl<GL: HasContext> Program<GL> {
  pub fn create(gl: &Rc<RefCell<GL>>) -> Result<Self, GlResourceError> {
    let program = {
      let cloned = gl.clone();
      let gl = cloned.borrow();
      unsafe { gl.create_program() }.map_err(|e| GlResourceError::new(e, None))?
    };

    Ok(Self {
      program,
      gl: Rc::downgrade(gl),
    })
  }
  pub fn program(&self) -> &<GL as HasContext>::Program {
    return &self.program;
  }
}
