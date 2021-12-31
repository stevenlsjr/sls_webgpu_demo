pub fn anyhow_from_poisoned<T>(err: std::sync::PoisonError<T>) -> anyhow::Error {
  anyhow::anyhow!("Lock poisoned: {:?}", err)
}
