use sls_webgpu::renderer_common::allocator::{ Handle};

struct Suite {}
fn setup() -> Suite {
  Suite {}
}

struct FakeTexture {
  handle: Handle,
  name: String,
}

#[test]
fn test_handle() {
  let cases = &[
    (1, 4, Handle::new(1, 4)),
    (100, 2222, Handle::new(100, 2222)),
    (36277, 3143, Handle::new(36277, 3143)),
    (4917, 381, Handle::new(4917, 381)),
  ];
  for (i, &(index, generation, handle)) in cases.into_iter().enumerate() {
    assert_eq!(index, handle.index(), "index() failed on case {}", i);
    assert_eq!(
      generation,
      handle.generation(),
      "generation() failed on case {}",
      i
    );
  }
}

mod test_allocator {
  // use super::*;
  use sls_webgpu::renderer_common::allocator::{ResourceManager, SparseArrayAllocator};
  use sls_webgpu::renderer_common::sparse_array_allocator::AlreadyFreedError;

  #[test]
  fn allocate() {
    let mut al: SparseArrayAllocator<i32> = SparseArrayAllocator::with_capacity(10);
    for i in 0..10 {
      let index = al.allocate(i);
      assert_eq!(al.get_ref(index), Some(&i));
    }
  }

  #[test]
  fn deallocate(){
    let mut al: SparseArrayAllocator<i32> = SparseArrayAllocator::with_capacity(10);
    for i in 0..10 {
      al.allocate(i);
    }
    al.free(9).unwrap();
    let next_index = al.allocate(0);
    assert_eq!(next_index, 9, "allocation {} should be the free index least recently freed", next_index);
    assert_eq!(al.free(9), Ok(0));
    assert_eq!( al.free(9), Err(AlreadyFreedError));
  }

  #[test]
  fn get_ref() {
    let mut cases = Vec::new();
    let mut al = SparseArrayAllocator::with_capacity(100);
    for _ in 0..100 {
      let value = rand::random::<f32>();
      let handle = al.allocate(value);
      cases.push((handle,value));
    }

    for (i, &(handle, value)) in cases.iter().enumerate() {
      let actual = al.get_ref(handle);
      assert_eq!(actual, Some(&value), "failed to get ref for case {}, handle={}", i, handle);
    }
    assert_eq!(al.get_ref(1000), None);
  }

  #[test]
  fn mut_ref() {
    let mut cases = Vec::new();
    let mut al = SparseArrayAllocator::with_capacity(100);
    for _ in 0..100 {
      let value = rand::random::<f32>();
      let handle = al.allocate(value);
      cases.push((handle,value));
    }

    for (i, &(handle, value)) in cases.iter().enumerate() {
      let actual = al.mut_ref(handle);
      assert_eq!(actual, Some(&mut value.clone()), "failed to get ref for case {}, handle={}", i, handle);
      if let Some( m) = actual {
        *m = 10f32;
      }
      assert_eq!(al.get_ref(handle), Some(& 10f32), "failed to get ref for case {}, handle={}", i, handle);

    }
    assert_eq!(al.mut_ref(1000), None);
  }
}

mod resource_manager {
  #[test]
  fn test_stuff(){}
}