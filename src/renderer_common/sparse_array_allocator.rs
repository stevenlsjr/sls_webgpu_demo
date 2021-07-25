use std::collections::LinkedList;
use std::fmt::{Display, Formatter};

///
/// Structure comprising of a sparse
/// array and linked list, which records
/// freed locations in the array.
/// When allocating a new element,
/// if the free_list is empty, push
/// a new item to the values vector.
/// If it is not, pop the first index
/// in the free list, and allocate the
/// location of the popped index
#[derive(Debug, Default, Clone)]
pub struct SparseArrayAllocator<T: Sized> {
  pub(crate) values: Vec<Option<T>>,
  pub(crate) free_list: LinkedList<usize>,
}

impl<T: Sized> SparseArrayAllocator<T> {
  pub fn new() -> Self {
    Self::with_capacity(0)
  }

  pub fn with_capacity(capacity: usize) -> Self {
    let values = Vec::with_capacity(capacity);
    let free_list = LinkedList::new();
    Self {
      values,
      free_list,
    }
  }


  ///
  ///
  /// # Arguments
  ///
  /// * `val`:
  ///
  /// returns: usize
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::SparseArrayAllocator;
  /// let mut allocator = SparseArrayAllocator::new();
  /// let index = allocator.allocate(1);
  /// assert_eq!(allocator.get_ref(index), Some(& 1))
  /// ```
  pub fn allocate(&mut self, val: T) -> usize {
    let index = match self.free_list.pop_front() {
      None => {
        self.values.push(None);
        self.values.len() - 1
      }
      Some(i) => { i }
    };
    self.values[index] = Some(val);
    index
  }

  ///
  ///
  /// # Arguments
  ///
  /// * `index`:
  ///
  /// returns: ()
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::SparseArrayAllocator;
  /// use sls_webgpu::renderer_common::sparse_array_allocator::AlreadyFreedError;
  /// let mut allocator = SparseArrayAllocator::new();
  /// let index = allocator.allocate(1);
  /// assert_eq!(allocator.free(index), Ok(1));
  /// assert_eq!(allocator.free(index), Err(AlreadyFreedError), "double-free results in an error return");
  /// assert_eq!(allocator.get_ref(index), None);
  /// ```
  pub fn free(&mut self, index: usize) -> Result<T, AlreadyFreedError> {
    let val = std::mem::replace(&mut self.values[index], None);
    match val {
      Some(val) => {
        self.free_list.push_back(index);
        Ok(val)
      }
      None => Err(AlreadyFreedError)
    }
  }


  ///
  ///
  /// # Arguments
  ///
  /// * `index`: index handle for the item to retrieve
  ///
  /// returns: Option<&T> Some reference at index if it is allocated
  ///
  /// # Examples
  ///
  /// ```
  ///
  /// ```
  pub fn get_ref(&self, index: usize) -> Option<&T> {
    if self.values.len() <= index {
      return None;
    }
    self.values[index].as_ref()
  }

  ///
  ///
  /// # Arguments
  ///
  /// * `index`: index handle for the item to retrieve
  ///
  /// returns: Option<&mut T> Some mutable reference at index if it is allocated
  ///
  /// # Examples
  ///
  /// ```
  ///
  /// ```
  pub fn mut_ref(&mut self, index: usize) -> Option<&mut T> {
    if self.values.len() <= index {
      return None;
    }
    self.values[index].as_mut()
  }


  ///
  ///
  /// # Arguments
  ///
  ///
  /// returns: number of active items in the sparse list
  ///
  /// # Efficiency:
  /// O(N) for number of freed items in the list
  ///
  /// # Examples
  ///
  /// ```
  /// use sls_webgpu::renderer_common::allocator::SparseArrayAllocator;
  /// use sls_webgpu::renderer_common::sparse_array_allocator::AlreadyFreedError;
  /// let mut allocator = SparseArrayAllocator::new();
  ///
  /// for i in 0..10 {
  ///   let index = allocator.allocate(0);
  ///   if i < 3 {
  ///     allocator.free(index).unwrap();
  ///   }
  /// }
  /// assert_eq!(allocator.len(), 7)
  /// ```
  pub fn len(&self) -> usize {
    // self.free_list.l
    self.values.len() - self.free_list.len()
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct AlreadyFreedError;

impl Display for AlreadyFreedError {
  fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
    f.write_str("Index Already Freed")
  }
}

impl std::error::Error for AlreadyFreedError {}