use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use glow::{Context, HasContext};
use std::fmt;
use nalgebra_glm::*;

static MAIN_FRAG_SRC: &str = include_str!("../shaders/main_gl.frag");
static MAIN_VERT_SRC: &str = include_str!("../shaders/main_gl.vert");


#[wasm_bindgen]
pub struct GlContext {
  #[cfg(feature = "html5_backend")]
  webgl_context: Option<web_sys::WebGl2RenderingContext>,
  gl: Context,
  shader_version_header: String,


  clear_color: Vec4,
}

const WEBGL_VERSION_HEADER: &str = "#version 300 es";
const GL410_VERSION_HEADER: &str = "#version 410";

pub struct FrameContext<'a> {
  ctx: &'a mut GlContext,
}

impl<'a> FrameContext<'a> {
  /// This is the actual
  /// Opengl rendering logic
  pub fn render(mut self) {
    let Self { ctx } = self;
    let gl = &ctx.gl;

  }
}

impl GlContext {
  pub fn prepare_frame(&mut self) -> FrameContext {
    unsafe {
      self.gl.clear(glow::COLOR_BUFFER_BIT);
    }
    let frame = FrameContext { ctx: self };

    frame
  }
}

impl fmt::Debug for GlContext {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
    f.debug_struct("GlContext")
      .finish()
  }
}


#[cfg(feature = "html5_backend")]
pub use html5::*;
use nalgebra_glm::Vec3;

#[cfg(feature = "html5_backend")]
pub mod html5 {
  use super::*;
  use web_sys::*;
  use std::rc::Rc;
  use glow::HasContext;

  impl GlContext {
    pub fn from_canvas(canvas: HtmlCanvasElement) -> Result<Self, crate::Error> {
      let webgl_context = canvas.get_context("webgl2")
        .map_err(|e| crate::Error::from_other(format!("{:?}", e)))?
        .ok_or_else(|| crate::Error::from_other(format!("could not get gl context for canvas {:?}", canvas)))?
        .unchecked_into::<WebGl2RenderingContext>();

      let gl: Context = Context::from_webgl2_context(webgl_context.clone());
      Ok(Self {
        gl,
        webgl_context: Some(webgl_context),
        shader_version_header: WEBGL_VERSION_HEADER.to_owned(),
        clear_color: vec4(0.0, 0.0, 0.0, 1.0),
      })
    }
    pub fn webgl_context(&self) -> Option<WebGl2RenderingContext> {
      self.webgl_context.clone()
    }
    pub fn set_webgl_context(&mut self, context: Option<WebGl2RenderingContext>) {
      self.webgl_context = context
    }

    pub fn clear_color(&self) -> &Vec4 {
      &self.clear_color
    }

    pub fn set_clear_color(&mut self, color: Vec4) {
      self.clear_color = color;
      unsafe {
        self.gl.clear_color(self.clear_color.x, self.clear_color.y, self.clear_color.z, self.clear_color.w);
      }
    }
  }
}

