use std::fmt::Formatter;
use std::option::Option::None;
use std::{error, fmt};

use legion::world::{ComponentError, EntityAccessError};

#[derive(Debug)]
pub enum Error {
  Create { reason: String },
  FromError(Box<dyn std::error::Error>),
  CreateObject { reason: String },
  Other { reason: String },
  LegionComponent(ComponentError),
  LegionEntityAccess(EntityAccessError),
}

impl Error {
  pub fn from_error(e: Box<dyn std::error::Error>) -> Self {
    Error::FromError(e)
  }
  pub fn from_other<S: AsRef<str>>(other: S) -> Self {
    Self::Other {
      reason: other.as_ref().to_string(),
    }
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    match self {
      Error::Create { reason } => write!(f, "creation error: {}", reason),
      Error::FromError(e) => write!(f, "error: {}", e),
      Error::CreateObject { reason } => write!(f, "error creating object: {}", reason),
      Error::Other { reason } => write!(f, "other error {}", reason),

      Error::LegionComponent(err) => {
        write!(f, "ComponentError: {:?}", err)
      }
      Error::LegionEntityAccess(err) => {
        write!(f, "EntityAccessError: {:?}", err)
      }
    }
  }
}

impl From<ComponentError> for Error {
  fn from(err: ComponentError) -> Self {
    Error::LegionComponent(err)
  }
}

impl From<EntityAccessError> for Error {
  fn from(err: EntityAccessError) -> Self {
    Error::LegionEntityAccess(err)
  }
}

impl error::Error for Error {
  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    match &self {
      Error::LegionEntityAccess(ref err) => Some(err),
      Error::LegionComponent(ref err) => Some(err),
      _ => None,
    }
  }
}
