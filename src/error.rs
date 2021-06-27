use raw_window_handle::HasRawWindowHandle;
use std::borrow::Cow;
use std::fmt::Formatter;
use std::rc::Rc;
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    Create { reason: String },
    FromError(Rc<dyn std::error::Error + 'static>),
    CreateObject { reason: String },
    Other { reason: String },
}

impl Error {
    pub fn from_error<E: error::Error + Sized + 'static>(e: E) -> Self {
        Error::FromError(Rc::new(e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Create { reason } => write!(f, "creation error: {}", reason),
            Error::FromError(e) => write!(f, "error: {}", e),
            Error::CreateObject { reason } => write!(f, "error creating object: {}", reason),
            Error::Other { reason } => write!(f, "other error {}", reason),
        }
    }
}

impl error::Error for Error {}
