use legion::*;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct GameState {
  world: World,
  // /// legion schedule triggered every fixed time update
  // fixed_schedule: Schedule,
  //
  // /// legion schedule triggered every frame
  // per_frame_schedule: Schedule,
  //
  // /// legion schedule triggered when the window size changes
  // on_resize_schedule: Schedule,
}

impl GameState {
  pub fn new() -> Self {
    let world = World::default();
    Self { world }
  }
  pub fn world(&self) -> &World {
    &self.world
  }

  pub fn world_mut(&mut self) -> &mut World {
    &mut self.world
  }
}
