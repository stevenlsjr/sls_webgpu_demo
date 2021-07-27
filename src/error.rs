use thiserror::Error as ThisError;

use legion::world::{ComponentError, EntityAccessError};

#[derive(Debug, ThisError)]
pub enum Error {
  #[error("problem creating Context: {reason}")]
  Create { reason: String },
  #[error("caused by error {0:?}")]
  FromError(#[from] Box<dyn std::error::Error + Send + Sync>),
  #[error("creating resource object {reason}")]
  CreateObject { reason: String },
  #[error("miscellaneous: {reason}")]
  Other { reason: String },
  #[error("legion component: {0:?}")]
  LegionComponent(#[from] ComponentError),
  #[error("legion entity access: {0:?}")]
  LegionEntityAccess(#[from] EntityAccessError),
}

impl Error {
  pub fn from_error(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
    Error::FromError(e)
  }
  pub fn from_other<S: AsRef<str>>(other: S) -> Self {
    Self::Other {
      reason: other.as_ref().to_string(),
    }
  }
}

//
// impl From<ComponentError> for Error {
//   fn from(err: ComponentError) -> Self {
//     Error::LegionComponent(err)
//   }
// }
//
// impl From<EntityAccessError> for Error {
//   fn from(err: EntityAccessError) -> Self {
//     Error::LegionEntityAccess(err)
//   }
// }
