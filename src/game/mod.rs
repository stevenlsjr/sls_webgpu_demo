mod transform;

use legion::*;
use crate::game::transform::Transform;
use log::Level::Trace;
use crate::camera::Camera;

pub struct GameState {
  world: World,
}

impl GameState {
  pub fn new() -> Self {
    let mut world = World::default();
    let camera = (
      Transform::default(),
      Camera::default()
    );
    world.push(camera);
    Self { world }
  }

  pub fn update() {}
}