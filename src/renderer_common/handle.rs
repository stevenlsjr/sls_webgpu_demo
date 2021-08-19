use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct HandleIndex(u32);

const HANDLE_INDEX_N_BITS: u32 = 20;
pub const HANDLE_INDEX_MASK: u32 = (1 << HANDLE_INDEX_N_BITS) - 1;
const HANDLE_GENERATION_MASK: u32 = !HANDLE_INDEX_MASK;
pub const GENERATION_MAX_SIZE: u32 = HANDLE_GENERATION_MASK >> HANDLE_INDEX_N_BITS;

impl HandleIndex {
  pub fn new(index: u32, generation: u16) -> Self {
    Self(((generation as u32) << HANDLE_INDEX_N_BITS) | (index & HANDLE_INDEX_MASK))
  }
  pub fn index(&self) -> u32 {
    self.0 & HANDLE_INDEX_MASK
  }

  pub fn generation(&self) -> u32 {
    (self.0 & HANDLE_GENERATION_MASK) >> HANDLE_INDEX_N_BITS
  }
  pub fn into_typed<T>(self) -> Handle<T> {
    Handle::from_index(self)
  }
}


/// Typed wrapper for HandleIndex
#[derive(Default, Debug, PartialEq)]
pub struct Handle<T: Sized> {
  index: HandleIndex,
  _phantom: PhantomData<*const T>,
}


impl<T: Sized> Clone for Handle<T> {
  fn clone(&self) -> Self {
    Self { index: self.index, _phantom: PhantomData }
  }
}

impl<T: Sized> Copy for Handle<T> {}

impl<T> Deref for Handle<T> {
  type Target = HandleIndex;

  fn deref(&self) -> &Self::Target {
    &self.index
  }
}

impl<T> Handle<T> {
  pub fn from_index(index: HandleIndex) -> Self {
    Self { index, _phantom: PhantomData }
  }
  pub fn to_index(self) -> HandleIndex { self.index }
}


pub trait ResourceStore<T>
  where T: Sized {
  fn get_ref(&self, handle: Handle<T>) -> Option<&T>;
  fn get_mut(&mut self, handle: Handle<T>) -> Option<&mut T>;

  fn insert(&mut self, value: T) -> Handle<T>;

  fn remove(&mut self, handle: T) -> Option<T>;
}