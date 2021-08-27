// Renderer resource management and handles.

use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

use crate::renderer_common::{handle::Handle, sparse_array_allocator::AlreadyFreedError};

use super::handle::{HandleIndex, GENERATION_MAX_SIZE, HANDLE_INDEX_MASK};
pub use super::sparse_array_allocator::SparseArrayAllocator;
use crate::renderer_common::has_uuid::HasUuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AllocatorError {
  NotFound,
  HandleFreed,
}

impl Display for AllocatorError {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    let msg = match self {
      AllocatorError::NotFound => "Not found",
      AllocatorError::HandleFreed => "Handle already freed",
    };
    f.write_str(msg)
  }
}

impl std::error::Error for AllocatorError {}

impl Into<String> for AllocatorError {
  fn into(self) -> String {
    format!("{:?}", self)
  }
}

///
/// A generational allocator for resources
/// with a handle
#[derive(Debug)]
pub struct ResourceManager<T: Sized> {
  resource_index: SparseArrayAllocator<HandleIndex>,
  resources: SparseArrayAllocator<T>,

  generation_count: u32,
}

impl<T: Sized> ResourceManager<T> {
  pub fn with_capacity(capacity: usize) -> Self {
    let generation_count = 0;
    Self {
      resource_index: SparseArrayAllocator::with_capacity(capacity),
      resources: SparseArrayAllocator::with_capacity(capacity),
      generation_count,
    }
  }

  /// Inserts a resource into the manager
  ///
  /// # Arguments
  ///
  /// * `value`: the resource to be managed
  ///
  /// returns: Handle to access the resource
  ///
  ///
  /// # Examples
  ///
  /// ```
  ///
  /// ```
  pub fn insert(&mut self, value: T) -> Handle<T> {
    self.incr_generation();
    let value_index = self.resources.allocate(value);
    assert_handle_index_size(value_index);

    let value_handle = HandleIndex::new(value_index as u32, self.generation_count as _);
    let resource_index_index = self.resource_index.allocate(value_handle);
    assert_handle_index_size(resource_index_index);
    HandleIndex::new(resource_index_index as _, self.generation_count as _).into_typed()
  }

  pub fn get_ref(&self, handle: Handle<T>) -> Result<&T, AllocatorError> {
    match self.resource_index.get_ref(handle.index() as _) {
      None => Err(AllocatorError::NotFound),
      Some(handle_to_resource) => {
        if handle_to_resource.generation() != handle.generation() {
          return Err(AllocatorError::HandleFreed);
        }
        self
          .resources
          .get_ref(handle_to_resource.index() as _)
          .ok_or(AllocatorError::NotFound)
      }
    }
  }

  ///
  ///
  /// # Arguments
  ///
  /// * `handle`:
  ///
  /// returns: Result<&mut T, AllocatorError>
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::ResourceManager;
  /// let mut mgr: ResourceManager<i64> = ResourceManager::with_capacity(1);
  /// let handle = mgr.insert(0);
  /// {
  ///   let reference = mgr.mut_ref(handle).unwrap();
  ///   *reference = 1;
  /// }
  /// assert_eq!(mgr.get_ref(handle), Ok(&1));
  /// ```
  pub fn mut_ref(&mut self, handle: Handle<T>) -> Result<&mut T, AllocatorError> {
    match self.resource_index.get_ref(handle.index() as _) {
      None => Err(AllocatorError::NotFound),
      Some(handle_to_resource) => {
        if handle_to_resource.generation() != handle.generation() {
          return Err(AllocatorError::HandleFreed);
        }
        self
          .resources
          .mut_ref(handle_to_resource.index() as _)
          .ok_or(AllocatorError::NotFound)
      }
    }
  }

  /// Gets the most recent generation number
  ///
  ///
  /// returns: Handle the generation of the last allocated entry.
  /// If none have been inserted, return 0
  ///
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::ResourceManager;
  /// let mut mgr = ResourceManager::with_capacity(0);
  /// assert_eq!(mgr.generation_count(), 0);
  /// for i in 0..100{
  ///   mgr.insert(i);
  /// }
  /// let current_gen = mgr.generation_count();
  /// let handle = mgr.insert(200);
  /// assert_eq!(mgr.generation_count(), current_gen + 1);
  /// ```
  pub fn generation_count(&self) -> u32 {
    self.generation_count
  }

  ///
  ///
  /// # Arguments
  ///
  ///
  /// returns: The number of items currently managed
  ///
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::{ResourceManager};
  /// let mut al = ResourceManager::with_capacity(10);
  /// let mut handles = Vec::new();
  /// for i in 0..10 {
  ///   handles.push(al.insert(0));
  /// }
  /// assert_eq!(al.len(), 10);
  /// for i in 0..3 {
  ///   let handle = handles[i];
  ///   al.remove(handle).unwrap();
  /// }
  /// assert_eq!(al.len(), 7);
  /// ```
  pub fn len(&self) -> usize {
    self.resources.len()
  }

  /// Removes the resource managed by a given handle
  ///
  /// # Arguments
  ///
  /// * `handle`:
  ///
  /// returns: Result<T, AlreadyFreedError>
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::{ResourceManager};
  /// use sls_webgpu::renderer_common::sparse_array_allocator::AlreadyFreedError;
  /// let mut al = ResourceManager::with_capacity(10);
  /// let handle_a = al.insert(0);
  /// let handle_b = al.insert(1);
  /// assert_eq!(al.remove(handle_a), Ok(0));
  /// assert_eq!(al.len(), 1);
  /// assert_eq!(al.remove(handle_a), Err(AlreadyFreedError));
  ///
  /// ```
  pub fn remove(&mut self, handle: Handle<T>) -> Result<T, AlreadyFreedError> {
    let resource_handle = self.get_resource_handle(handle)?;
    let val = self.resources.free(resource_handle.index() as _)?;
    self.resource_index.free(handle.index() as _)?;
    Ok(val)
  }

  fn incr_generation(&mut self) {
    self.generation_count += 1;
    if self.generation_count > GENERATION_MAX_SIZE {
      self.generation_count = 1;
    }
  }

  ///
  /// Given the public index handle, returns the internal handle to the
  /// actual resource. If the handle's generation does not match, return
  /// an AlreadyFreedError
  fn get_resource_handle(&self, index_handle: Handle<T>) -> Result<HandleIndex, AlreadyFreedError> {
    let resource_handle = self
      .resource_index
      .get_ref(index_handle.index() as _)
      .ok_or(AlreadyFreedError)?;
    if resource_handle.generation() != index_handle.generation() {
      Err(AlreadyFreedError)
    } else {
      Ok(*resource_handle)
    }
  }
}

impl<T: Sized> Default for ResourceManager<T> {
  fn default() -> Self {
    Self::with_capacity(0)
  }
}

fn assert_handle_index_size(index: usize) {
  const INDEX_MAXSIZE: usize = HANDLE_INDEX_MASK as usize;
  if index >= INDEX_MAXSIZE {
    panic!("index {} cannot be larger than {}", index, INDEX_MAXSIZE);
  }
}
