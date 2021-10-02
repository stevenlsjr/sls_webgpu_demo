use sls_webgpu::util::anyhow_from_poisoned;
use std::sync::PoisonError;

#[test]
fn test_anyhow_from_poisoned() {
  let err = PoisonError::new(());
  let err_debug = format!("{:?}", err);
  let anyhow_err = anyhow_from_poisoned(err);
  let anyhow_debug = format!("{:?}", anyhow_err);
  assert!(
    anyhow_debug.contains(&err_debug),
    "'{}.debug(...)' should display '{}'",
    anyhow_debug,
    err_debug
  )
}
