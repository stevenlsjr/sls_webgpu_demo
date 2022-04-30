use sls_webgpu::renderer_common::handle::*;

#[test]
fn test_any_handle() {
  #[derive(Debug, PartialEq)]
  struct TestStruct;
  let handle = HandleIndex(1).into_typed::<TestStruct>();
  let any_handle = AnyHandle::from_handle(handle);
  let expected = Handle::<TestStruct>::from_index(HandleIndex(1));
  assert_eq!(any_handle.downcast::<TestStruct>(), Some(expected));
  assert_eq!(any_handle.downcast::<u32>(), None);
}
