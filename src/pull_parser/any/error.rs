//! Error and result types for `pull_parser::any` module.

use std::{error, fmt};

use crate::low::{FbxVersion, HeaderError};

/// AnyTree load result.
pub type Result<T> = std::result::Result<T, Error>;

/// Error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Header error.
    Header(HeaderError),
    /// Unsupported version.
    UnsupportedVersion(FbxVersion),
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Header(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Header(e) => write!(f, "FBX header error: {}", e),
            Error::UnsupportedVersion(ver) => write!(f, "Unsupported FBX version: {:?}", ver),
        }
    }
}

impl From<HeaderError> for Error {
    #[inline]
    fn from(e: HeaderError) -> Self {
        Error::Header(e)
    }
}
