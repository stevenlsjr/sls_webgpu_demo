

use wasm_bindgen::prelude::*;

use nalgebra_glm as glm;
use std::sync::{RwLock, Arc};

#[wasm_bindgen]
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Vec3Val(glm::Vec3);

#[wasm_bindgen]
impl Vec3Val {
  pub fn make(x: f32, y: f32, z: f32) -> Self{
    Self(glm::vec3(x, y, z))
  }
  pub fn x(&self)-> f32 {
    return self.0.x
  }
   pub fn y(&self)-> f32 {
    return self.0.y
  }
   pub fn z(&self)-> f32 {
    return self.0.z
  }
}