use super::{resources, util::*};
use crate::gl_renderer::resources::GlResourceError;
use glow::{Context, HasContext};
use std::{cell::RefCell, rc::Rc};

struct CreateProgram {}

pub fn compile_basic_program<GL: HasContext>(
  gl: &Rc<RefCell<GL>>,
  frag_src: &str,
  vert_src: &str,
) -> Result<resources::Program<GL>, resources::GlResourceError> {
  compile_program(
    gl,
    &[
      (glow::FRAGMENT_SHADER, frag_src),
      (glow::VERTEX_SHADER, vert_src),
    ],
  )
}

pub fn compile_program<GL: HasContext>(
  gl: &Rc<RefCell<GL>>,
  sources: &[(u32, &str)],
) -> Result<resources::Program<GL>, resources::GlResourceError> {
  let program = resources::Program::create(gl)?;
  let shader_types: Vec<_> = sources.iter().map(|t| t.0).collect();
  let shaders = resources::Shaders::create(gl, &shader_types)?;
  let vs = shaders.shaders()[0];
  let fs = shaders.shaders()[1];
  {
    let gl = gl.borrow();
    let iter: Vec<_> = shaders
      .shaders()
      .iter()
      .zip(sources)
      .map(|(shader, source)| (shader, source.1))
      .collect();
    for (&shader, src) in iter {
      unsafe {
        gl.shader_source(shader, src);
        check_errors(&*gl)?;
        gl.compile_shader(shader);
        if !gl.get_shader_compile_status(shader) {
          let shader_info_log = gl.get_shader_info_log(shader);
          return Err(GlResourceError {
            reason: format!("shader compilation error: {}", shader_info_log),
            gl_code: None,
          });
        }

        check_errors(&*gl)?;
      }
    }
    unsafe {
      gl.attach_shader(*program.program(), vs);
      gl.attach_shader(*program.program(), fs);

      gl.link_program(*program.program());
    }
    if !unsafe { gl.get_program_link_status(*program.program()) } {
      let info_log = unsafe { gl.get_program_info_log(*program.program()) };
      let fs_info_log = unsafe { gl.get_shader_info_log(fs) };
      log::error!("vs compile status: {:?}", unsafe {
        gl.get_shader_compile_status(vs)
      });
      log::error!(
        "fs compile status: {:?}\n{}",
        unsafe { gl.get_shader_compile_status(fs) },
        fs_info_log
      );
      return Err(GlResourceError::new(
        format!("program link error: {}\n", info_log),
        None,
      ));
    }
  }

  Ok(program)
}
