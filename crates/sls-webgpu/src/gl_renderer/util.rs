use glow::HasContext;

pub static GL_NO_ERROR: &str = "NO_ERROR";
pub static GL_INVALID_ENUM: &str = "GL_INVALID_ENUM";
pub static GL_INVALID_VALUE: &str = "GL_INVALID_VALUE";
pub static GL_INVALID_OPERATION: &str = "GL_INVALID_OPERATION";
pub static GL_INVALID_FRAMEBUFFER_OPERATION: &str = "GL_INVALID_FRAMEBUFFER_OPERATION";
pub static GL_OUT_OF_MEMORY: &str = "GL_OUT_OF_MEMORY";
pub static GL_STACK_UNDERFLOW: &str = "GL_STACK_UNDERFLOW";
pub static GL_STACK_OVERFLOW: &str = "GL_STACK_OVERFLOW";
pub static UNKNOWN_ERROR_CODE: &str = "UNKNOWN_ERROR_CODE!";

pub fn error_to_str(error: u32) -> Option<&'static str> {
  match error {
    glow::NO_ERROR => Some(GL_NO_ERROR),
    glow::INVALID_ENUM => Some(GL_INVALID_ENUM),
    glow::INVALID_VALUE => Some(GL_INVALID_VALUE),
    glow::INVALID_OPERATION => Some(GL_INVALID_OPERATION),
    glow::INVALID_FRAMEBUFFER_OPERATION => Some(GL_INVALID_FRAMEBUFFER_OPERATION),
    glow::OUT_OF_MEMORY => Some(GL_OUT_OF_MEMORY),
    glow::STACK_UNDERFLOW => Some(GL_STACK_UNDERFLOW),
    glow::STACK_OVERFLOW => Some(GL_STACK_OVERFLOW),
    _ => None,
  }
}

pub fn check_errors<GL: HasContext>(gl: &GL) -> Result<(), Vec<(u32, String)>> {
  let mut errors = vec![];
  let mut checking = true;
  while checking {
    let err = unsafe { gl.get_error() };
    match err {
      glow::NO_ERROR => {
        checking = false;
      }
      other => errors.push((
        other,
        error_to_str(other).unwrap_or(UNKNOWN_ERROR_CODE).to_owned(),
      )),
    }
  }
  if errors.len() == 0 {
    Ok(())
  } else {
    Err(errors)
  }
}
