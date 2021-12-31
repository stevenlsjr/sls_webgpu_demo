use uuid::Uuid;

pub trait HasUuid {
  fn uuid(&self) -> Uuid;
}
