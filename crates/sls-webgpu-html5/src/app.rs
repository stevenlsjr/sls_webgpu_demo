use super::platform;
use js_sys;
use sls_webgpu::game::input::{InputBackend, InputResource};
use sls_webgpu::game::{html5_backend::Html5Backend, CreateGameParams, GameState};
use sls_webgpu::platform::keyboard::Keycode::CapsLock;
use sls_webgpu::platform::keyboard::Scancode::App1;
use std::cell::{RefCell, UnsafeCell};
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::time::{Duration, Instant};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::convert::{IntoWasmAbi, RefFromWasmAbi};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, EventTarget, HtmlCanvasElement, HtmlElement, Element};
use js_sys::*;

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
  pub fn new_js(app_root: Option<HtmlElement>) -> Result<SlsWgpuDemo, JsValue> {
    let app = SlsWgpuDemo::new(app_root)?;

    Ok(app)
  }

  #[wasm_bindgen]
  pub fn run(&mut self) -> Result<(), JsValue> {
    let mut cloned = self.clone();

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
    Ok(())
  }

  #[wasm_bindgen(getter)]
  pub fn is_running(&self) -> bool {
    return self.app.borrow().is_running;
  }

  #[wasm_bindgen(setter)]
  pub fn set_is_running(&mut self, value: bool) {
    self.app.borrow_mut().is_running = value;
  }

    #[wasm_bindgen(getter)]

  pub fn canvas(&self) -> Option<HtmlCanvasElement> {
    return self.app.borrow().canvas.clone()
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
  pub fn new(app_root: Option<HtmlElement>) -> Result<Self, String> {
    let input_backend = Html5Backend::new();
    let game_state = GameState::new(CreateGameParams {
      input_backend: Box::new(input_backend),
    });
    let mut app = AppInternal {
      game_state,
      is_running: false,
      last_frame_ms: js_sys::Date::now(),
      update_lag: Duration::from_millis(0),
      ms_per_update: Duration::from_millis(16),
      canvas: None,
      on_resize: Default::default(),
      on_keyup: Default::default(),
      on_keydown: Default::default(),
    };

    if let Some(app_root) = app_root {
      let canvas = create_canvas().map_err(|e| {
        log::error!("error creating canvas: {:?}", e);
        format!("{:?}", e)
      })?;
      app_root.append_child(&canvas);
      app.canvas = Some(canvas);

    }
    Ok(Self {
      app: Rc::new(RefCell::new(app)),
    })
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
  }

  fn setup_callbacks(&mut self) {
    {
      let window = platform::window();

      let mut cloned = self.clone();
      let on_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut app = cloned.app.borrow_mut();
        app.game_state.map_input_backend_mut(|backend: &mut Html5Backend| {}).expect(
          "input backend is an incorrect type");
        if let Err(e) = call_event_cb(&app.on_keydown, event.clone()) {
          log::error!("app callback failed: '{:?}'", e);
        }
      }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

      EventTarget::from(window)
        .add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref());

      on_keydown.forget();
    }

    {
      let window = platform::window();

      let mut cloned = self.clone();
      let on_keyup = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let mut app = cloned.app.borrow_mut();
        app.game_state.map_input_backend_mut(|backend: &mut Html5Backend| {}).expect(
          "input backend is an incorrect type");
        if let Err(e) = call_event_cb(&app.on_keyup, event.clone()) {
          log::error!("app callback failed: '{:?}'", e);
        }
      }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
      EventTarget::from(window)
        .add_event_listener_with_callback("keyup", on_keyup.as_ref().unchecked_ref());

      on_keyup.forget();
    }
  }
}

#[derive(Debug)]
pub(crate) struct AppInternal {
  game_state: GameState,
  is_running: bool,

  last_frame_ms: f64,
  update_lag: Duration,
  ms_per_update: Duration,

  canvas: Option<HtmlCanvasElement>,

  on_keyup: Option<js_sys::Function>,
  on_keydown: Option<js_sys::Function>,
  on_resize: Option<js_sys::Function>,
}

fn call_event_cb<Event: Into<JsValue>>(cb: &Option<js_sys::Function>, event: Event) -> Result<JsValue, JsValue> {
  match cb {
    Some(cb) => {
      cb.call1(&JsValue::undefined(), &(Into::<JsValue>::into(event)))
    }
    None => Ok(JsValue::undefined())
  }
}

fn create_canvas()->Result<HtmlCanvasElement, JsValue> {
  let elt = platform::document().create_element("canvas")?.unchecked_into::<HtmlCanvasElement>();
  elt.set_id("app-canvas");
  elt.set_class_name("slswebgpu-canvas");
  Ok(elt)
}