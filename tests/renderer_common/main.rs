mod gltf_loader;

use sls_webgpu::renderer_common::handle::Handle;

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
  for (i, &(index, generation, handle)) in cases.iter().enumerate() {
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
  use sls_webgpu::renderer_common::{
    allocator::SparseArrayAllocator, sparse_array_allocator::AlreadyFreedError,
  };

  #[test]
  fn allocate() {
    let mut al: SparseArrayAllocator<i32> = SparseArrayAllocator::with_capacity(10);
    for i in 0..10 {
      let index = al.allocate(i);
      assert_eq!(al.get_ref(index), Some(&i));
    }
  }

  #[test]
  fn deallocate() {
    let mut al: SparseArrayAllocator<i32> = SparseArrayAllocator::with_capacity(10);
    for i in 0..10 {
      al.allocate(i);
    }
    al.free(9).unwrap();
    let next_index = al.allocate(0);
    assert_eq!(
      next_index, 9,
      "allocation {} should be the free index least recently freed",
      next_index
    );
    assert_eq!(al.free(9), Ok(0));
    assert_eq!(al.free(9), Err(AlreadyFreedError));
  }

  #[test]
  fn get_ref() {
    let mut cases = Vec::new();
    let mut al = SparseArrayAllocator::with_capacity(100);
    for _ in 0..100 {
      let value = rand::random::<f32>();
      let handle = al.allocate(value);
      cases.push((handle, value));
    }

    for (i, &(handle, value)) in cases.iter().enumerate() {
      let actual = al.get_ref(handle);
      assert_eq!(
        actual,
        Some(&value),
        "failed to get ref for case {}, handle={}",
        i,
        handle
      );
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
      cases.push((handle, value));
    }

    for (i, &(handle, value)) in cases.iter().enumerate() {
      let actual = al.mut_ref(handle);
      assert_eq!(
        actual,
        Some(&mut value.clone()),
        "failed to get ref for case {}, handle={}",
        i,
        handle
      );
      if let Some(m) = actual {
        *m = 10f32;
      }
      assert_eq!(
        al.get_ref(handle),
        Some(&10f32),
        "failed to get ref for case {}, handle={}",
        i,
        handle
      );
    }
    assert_eq!(al.mut_ref(1000), None);
  }

  #[test]
  fn iter() {
    use sls_webgpu::renderer_common::allocator::SparseArrayAllocator;

    let mut allocator: SparseArrayAllocator<i32> = SparseArrayAllocator::new();
    for i in 0..6 {
      allocator.allocate(i);
    }
    for i in 0..3 {
      allocator.free(i).unwrap();
    }
    let mut iter = allocator.iter().cloned();
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(4));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
  }

  #[test]
  fn iter_mut() {
    use sls_webgpu::renderer_common::allocator::SparseArrayAllocator;

    let mut allocator: SparseArrayAllocator<i32> = SparseArrayAllocator::new();
    for i in 0..6 {
      allocator.allocate(i);
    }
    for i in 0..3 {
      allocator.free(i).unwrap();
    }
    let mut iter = allocator.iter_mut();
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), Some(&mut 5));
    assert_eq!(iter.next(), None);
  }
}

mod resource_manager {
  #[test]
  fn test_stuff() {}
}
