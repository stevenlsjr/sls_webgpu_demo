use glow::{HasContext, Context};
use super::util::*;
use web_sys::delete_shader;

struct CreateProgram {
}

pub fn compile_basic_program<GL: HasContext>(gl: &GL,
                                             frag_src: &str,
                                             vert_src: &str) ->
                                             Result<<GL as HasContext>::Program, String> {
  let (vs, fs) = unsafe {
    let vs = gl.create_shader(glow::VERTEX_SHADER).unwrap();
    let fs = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
    (vs, fs)
  };
  let program = unsafe {gl.create_program().unwrap()};


  todo!()
}

