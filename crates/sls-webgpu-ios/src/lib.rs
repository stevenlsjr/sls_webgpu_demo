pub struct App {
  num: i32,
}

impl App {
  pub fn new() -> anyhow::Result<Self> {
    let app = App { num: 1 };
    Ok(app)
  }
}

#[repr(C)]
pub enum AppResult {
  Ok(*mut App),
  Err(AppError),
}

#[repr(u8)]
pub enum AppError {
  CouldNotCreate,
}

#[no_mangle]
pub extern "C" fn sls_app_make() -> AppResult {
  match App::new() {
    Ok(app) => {
      let boxed = Box::new(app);
      let ptr = Box::into_raw(boxed);
      AppResult::Ok(ptr)
    }
    Err(reason) => AppResult::Err(AppError::CouldNotCreate),
  }
}
#[no_mangle]
pub unsafe extern "C" fn sls_app_release(app: *mut App) {
  unsafe { drop(Box::from_raw(app)) }
}

#[no_mangle]
pub unsafe extern "C" fn sls_app_num(app: &App) -> i32 {
  app.num
}
#[no_mangle]
pub extern "C" fn get_cpu_count() -> i32 {
  sdl2_sys::SDL_GetCpuCount()
}
