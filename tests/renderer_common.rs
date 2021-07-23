use sls_webgpu::renderer_common::allocator::{EngineResource, Handle};

struct Suite {}
fn setup() -> Suite {
  Suite {}
}

struct FakeTexture {
  handle: Handle,
  name: String,
}
impl EngineResource for FakeTexture {
  #[inline]
  fn handle(&self) -> Handle {
    self.handle
  }

  #[inline]
  fn set_handle(&mut self, handle: Handle) {
    self.handle = handle;
  }
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
  use super::*;
  use sls_webgpu::renderer_common::allocator::SparseResourceAllocator;

  #[test]
  fn insert() {
    let al = SparseResourceAllocator::new(100);
    let entity = al.insert(1);
    assert_eq!(al.get(entity), Ok(1));
  }
}
