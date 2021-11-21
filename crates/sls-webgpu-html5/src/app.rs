use std::{cell::RefCell, future::Future, rc::Rc, time::Duration};

use js_sys;
use wasm_bindgen::{closure::Closure, prelude::*, JsCast};
use web_sys::{EventTarget, HtmlCanvasElement, HtmlElement, WebGl2RenderingContext};

use super::platform;
use crate::{
  ffi::CreateAppOptionsJs,
  options::{self, CreateAppOptions, RendererBackend},
};
use sls_webgpu::{
  game::{
    input::InputState, resources::ScreenResolution,
    GameStateBuilder, GameState,
  },
  gl_renderer::GlContext,
  nalgebra_glm::*,
  platform::html5::FromCanvas,
  renderer_common::RenderContext,
  wgpu_renderer,
  window::{AsHtmlWindowWrapper, AsWindow, HtmlWindowImpl},
  wgpu,
};

#[wasm_bindgen]
pub enum EventType {
  KeyDown,
  KeyUp,
  Resize,
  Undefined,
}

impl<T: AsRef<str>> From<T> for EventType {
  fn from(s: T) -> Self {
    match s.as_ref() {
      "keydown" => Self::KeyDown,
      "keyup" => Self::KeyUp,
      "resize" => Self::Resize,
      _ => Self::Undefined,
    }
  }
}

impl Into<JsValue> for EventType {
  fn into(self) -> JsValue {
    match self {
      EventType::KeyDown => "keydown".into(),
      EventType::KeyUp => "keyup".into(),
      EventType::Resize => "resize".into(),
      EventType::Undefined => JsValue::undefined(),
    }
  }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct SlsWgpuDemo {
  app: Rc<RefCell<AppInternal>>,
}

#[wasm_bindgen]
impl SlsWgpuDemo {
  #[wasm_bindgen(constructor)]
  pub fn new_js(
    app_root: Option<HtmlElement>,
    options: Option<CreateAppOptionsJs>,
  ) -> Result<SlsWgpuDemo, JsValue> {
    let app_options = options
      .map(|options| CreateAppOptions::from_js(options))
      .unwrap_or(Ok(CreateAppOptions::default()))?;
    SlsWgpuDemo::new(app_root, app_options.renderer).map_err(|e| js_sys::Error::new(&e).into())
  }

  fn on_start(&self) {
    let app_ptr = self.app.clone();
    let mut app = app_ptr.borrow_mut();
    let canvas = app
      .canvas
      .as_ref()
      .expect("Canvas should have been attached already");
    let window: HtmlWindowImpl = canvas.as_gamewindow();
    let (width, height) = window.size();
    let resources = app.game_state.resources_mut();
    resources.insert(ScreenResolution {
      window_size: (width as usize, height as usize),
      drawable_size: (width as usize, height as usize),
    });
  }

  #[wasm_bindgen]
  pub fn run(&self) -> js_sys::Promise {
    use wasm_bindgen_futures::future_to_promise;
    let sself = self.clone();
    future_to_promise(sself.run_impl())
  }

  fn run_impl(mut self) -> impl Future<Output=Result<JsValue, JsValue>> {
    let cloned = self.clone();

    async move {
      self.setup_render_context().await?;

      {
        let renderer = self.app.borrow().renderer.as_ref().unwrap().clone();
        let mut renderer = renderer.borrow_mut();
        renderer.set_clear_color(vec4(1f32, 0.0, 1.0, 1.0));
      }
      self.on_start();

      {
        let mut app = self.app.borrow_mut();
        app.is_running = true;

        app.game_state.on_start();
        app.last_frame_ms = js_sys::Date::now();
      }
      {
        self.setup_callbacks();
      }
      let frame_cb = Rc::new(RefCell::new(None));
      let g = frame_cb.clone();
      *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        cloned.run_frame();
        if cloned.is_running() {
          platform::request_animation_frame(frame_cb.borrow().as_ref().unwrap());
        };
      }) as Box<dyn FnMut()>));
      platform::request_animation_frame(g.borrow().as_ref().unwrap());
      Ok(JsValue::undefined())
    }
  }

  #[wasm_bindgen(getter, js_name = "isRunning")]
  pub fn is_running(&self) -> bool {
    return self.app.borrow().is_running;
  }

  #[wasm_bindgen(setter, js_name = "isRunning")]
  pub fn set_is_running(&mut self, value: bool) {
    self.app.borrow_mut().is_running = value;
  }

  #[wasm_bindgen(getter)]
  pub fn canvas(&self) -> Option<HtmlCanvasElement> {
    return self.app.borrow().canvas.clone();
  }

  #[wasm_bindgen(getter, js_name = "webGlContext")]
  pub fn webgl_ctx(&self) -> Option<WebGl2RenderingContext> {
    let app = self.app.borrow();
    let renderer = app.renderer.as_ref();
    renderer
      .map(|r| {
        r.borrow()
          .downcast_ref::<GlContext>()
          .and_then(|gl| gl.webgl_context())
      })
      .flatten()
  }

  pub fn set_canvas(&mut self, canvas: Option<HtmlCanvasElement>) {
    self.app.borrow_mut().canvas = canvas;
  }

  #[wasm_bindgen]
  pub fn on(&self, event: &str, callback: Option<js_sys::Function>) -> Result<(), JsValue> {
    let mut app = self.app.borrow_mut();
    match EventType::from(event) {
      EventType::KeyUp => app.on_keyup = callback,
      EventType::KeyDown => app.on_keydown = callback,
      EventType::Resize => app.on_resize = callback,
      _ => return Err(format!("event type '{}' not supported", event).into()),
    };
    Ok(())
  }
}

impl SlsWgpuDemo {
  pub fn new(
    app_root: Option<HtmlElement>,
    render_backend: RendererBackend,
  ) -> Result<Self, String> {
    let game_state = GameState::new(GameStateBuilder { asset_loader_queue: None });
    let mut app = AppInternal {
      game_state,
      renderer: None,
      is_running: false,
      last_frame_ms: js_sys::Date::now(),
      update_lag: Duration::from_millis(0),
      ms_per_update: Duration::from_millis(16),
      canvas: None,
      on_resize: Default::default(),
      on_keyup: Default::default(),
      on_keydown: Default::default(),
      render_backend,
    };

    if let Some(app_root) = app_root {
      dbg!(&app_root);
      let canvas = create_canvas().map_err(|e| {
        log::error!("error creating canvas: {:?}", e);
        format!("{:?}", e)
      })?;
      dbg!(&canvas);
      app_root
        .append_child(&canvas)
        .map_err(|e| format!("could not append canvas to root {:?}", e))?;
      app.canvas = Some(canvas);
    }
    log::warn!("canvas: {:?}", app.canvas);
    Ok(Self {
      app: Rc::new(RefCell::new(app)),
    })
  }

  async fn setup_render_context(&self) -> Result<(), JsValue> {
    let mut app = self.app.borrow_mut();
    if app.renderer.is_some() {
      return Ok(());
    }
    let ctx_backends = match app.render_backend {
      RendererBackend::WebGL => { wgpu::Backends::GL }
      RendererBackend::WebGPU => { wgpu::Backends::BROWSER_WEBGPU }
    };

    match &app.canvas {
      Some(canvas) => {
        let game_window = canvas.as_gamewindow();
        let wgpu_context = wgpu_renderer::Context::new(&game_window)
          .with_backends(Some(ctx_backends))
          .build()
          .await
          .map_err(|e| JsValue::from(js_sys::Error::new(&format!("{:?}", e))))?;
        app.renderer = Some(Rc::new(RefCell::new(wgpu_context)));
      }
      (None) => {
        return Err(JsValue::from_str(&format!(
          "Canvas is not defined! {:?}",
          self
        )));
      }
    };

    Ok(())
  }

  pub fn run_frame(&self) {
    let mut app = self.app.borrow_mut();
    let current_time_ms = js_sys::Date::now();
    let elapsed_time = Duration::from_millis((current_time_ms - app.last_frame_ms) as u64);
    app.last_frame_ms = current_time_ms;
    app.update_lag += elapsed_time;
    let ms_per_update = app.ms_per_update.clone();

    app.game_state.update(&elapsed_time.clone());

    while app.update_lag >= ms_per_update {
      app.game_state.fixed_update(&ms_per_update);
      app.update_lag -= ms_per_update;
    }

    if let Some(renderer) = app.renderer.clone() {
      let mut renderer = renderer.borrow_mut();
      if let Err(e) = renderer.on_render(&mut app.game_state) {
        log::error!("render failed: {:?}", e);
      }
    }
  }

  fn setup_callbacks(&mut self) {
    {
      let window = platform::window();

      let cloned = self.clone();
      let on_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut app = cloned.app.borrow_mut();
        app
          .game_state
          .map_input_backend_mut(|_backend| {})
          .expect("input backend is an incorrect type");
        if let Err(e) = call_event_cb(&app.on_keydown, event.clone()) {
          log::error!("app callback failed: '{:?}'", e);
        }
      }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

      if let Err(e) = EventTarget::from(window)
        .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())
      {
        log::error!("could not bind keydown event target: {:?}", e);
      }

      on_keydown.forget();
    }

    {
      let window = platform::window();

      let cloned = self.clone();
      let on_keyup = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut app = cloned.app.borrow_mut();
        app
          .game_state
          .map_input_backend_mut(|_backend: &mut InputState| {})
          .expect("input backend is an incorrect type");
        if let Err(e) = call_event_cb(&app.on_keyup, event.clone()) {
          log::error!("app callback failed: '{:?}'", e);
        }
      }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
      if let Err(e) = EventTarget::from(window)
        .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref())
      {
        log::error!("could not bind keyup event: {:?}", e);
      }

      on_keyup.forget();
    }
  }
}

#[derive(Debug)]
pub(crate) struct AppInternal {
  render_backend: RendererBackend,
  game_state: GameState,
  is_running: bool,

  renderer: Option<Rc<RefCell<dyn RenderContext>>>,

  last_frame_ms: f64,
  update_lag: Duration,
  ms_per_update: Duration,

  canvas: Option<HtmlCanvasElement>,

  on_keyup: Option<js_sys::Function>,
  on_keydown: Option<js_sys::Function>,
  on_resize: Option<js_sys::Function>,
}

fn call_event_cb<Event: Into<JsValue>>(
  cb: &Option<js_sys::Function>,
  event: Event,
) -> Result<JsValue, JsValue> {
  match cb {
    Some(cb) => cb.call1(&JsValue::undefined(), &(Into::<JsValue>::into(event))),
    None => Ok(JsValue::undefined()),
  }
}

fn create_canvas() -> Result<HtmlCanvasElement, JsValue> {
  let elt = platform::document()
    .create_element("canvas")?
    .unchecked_into::<HtmlCanvasElement>();
  elt.set_id("app-canvas");
  elt.set_class_name("slswebgpu-canvas");
  Ok(elt)
}
