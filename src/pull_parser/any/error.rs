//! Error and result types for `pull_parser::any` module.

use std::{error, fmt};

use crate::low::{FbxVersion, HeaderError};

/// AnyTree load result.
pub type Result<T> = std::result::Result<T, Error>;

/// Error.
#[derive(Debug)]
pub enum Error {
    /// Header error.
    Header(HeaderError),
    /// Unsupported version.
    UnsupportedVersion(FbxVersion),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Header(e) => Some(e),
            Error::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Header(e) => write!(f, "FBX header error: {}", e),
            Error::UnsupportedVersion(ver) => write!(f, "Unsupported FBX version: {:?}", ver),
            Error::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

impl From<HeaderError> for Error {
    fn from(e: HeaderError) -> Self {
        Error::Header(e)
    }
}
