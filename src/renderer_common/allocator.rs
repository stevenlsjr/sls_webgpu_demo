// Renderer resource management and handles.

use hibitset::{BitSet, BitSetAll};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Handle(u32);
const HANDLE_INDEX_N_BITS: u32 = 20;
const HANDLE_INDEX_MASK: u32 = (1 << HANDLE_INDEX_N_BITS) - 1;
const HANDLE_GENERATION_MASK: u32 = !HANDLE_INDEX_MASK;

impl Handle {
  pub fn new(index: u32, generation: u16) -> Self {
    Self(((generation as u32) << HANDLE_INDEX_N_BITS) | (index & HANDLE_INDEX_MASK))
  }
  pub fn index(&self) -> u32 {
    self.0 & HANDLE_INDEX_MASK
  }

  pub fn generation(&self) -> u32 {
    (self.0 & HANDLE_GENERATION_MASK) >> HANDLE_INDEX_N_BITS
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {
  OutOfSpace,
  HandleFreed,
}

///
/// A generational allocator for resources
/// with a handle
pub struct SparseResourceAllocator<T: Sized> {
  generations: Vec<Handle>,
  values: Vec<Option<T>>, // todo: use a free list
  generation_count: u32,
}

impl<T: Sized> SparseResourceAllocator<T> {
  pub fn new(capacity: usize) -> Self {
    let generations = vec![Handle(0); capacity];
    let mut values = Vec::with_capacity(capacity);
    for i in 0..(capacity - 1) {
      values.push(None);
    }
    let generation_count = 1;
    Self {
      generations,
      values,
      generation_count,
    }
  }
  pub fn insert(&mut self, value: T) -> Result<Handle, AllocatorError> {
    let index = 0;
  }
}
