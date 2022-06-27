//! Binary writer error.

use std::{error, fmt, io};

use crate::low::FbxVersion;

/// Write result.
pub type Result<T> = std::result::Result<T, Error>;

/// Write error.
#[derive(Debug)]
pub enum Error {
    /// Node attribute is too long.
    AttributeTooLong(usize),
    /// Compression error.
    Compression(CompressionError),
    /// File is too large.
    FileTooLarge(u64),
    /// I/O error.
    Io(io::Error),
    /// There are no nodes to close.
    NoNodesToClose,
    /// Node name is too long.
    NodeNameTooLong(usize),
    /// Too many array attribute elements.
    TooManyArrayAttributeElements(usize),
    /// Too many attributes.
    TooManyAttributes(usize),
    /// There remains unclosed nodes.
    UnclosedNode(usize),
    /// Unsupported FBX version.
    UnsupportedFbxVersion(FbxVersion),
    /// User-defined error.
    UserDefined(Box<dyn std::error::Error + 'static>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Compression(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::UserDefined(e) => Some(&**e),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::AttributeTooLong(v) => write!(f, "Node attribute is too long: {} bytes", v),
            Error::Compression(e) => write!(f, "Compression error: {}", e),
            Error::FileTooLarge(v) => write!(f, "File is too large: {} bytes", v),
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::NoNodesToClose => write!(f, "There are no nodes to close"),
            Error::NodeNameTooLong(v) => write!(f, "Node name is too long: {} bytes", v),
            Error::TooManyArrayAttributeElements(v) => write!(
                f,
                "Too many array elements for a single node attribute: count={}",
                v
            ),
            Error::TooManyAttributes(v) => write!(f, "Too many attributes: count={}", v),
            Error::UnclosedNode(v) => write!(f, "There remains unclosed nodes: depth={}", v),
            Error::UnsupportedFbxVersion(v) => write!(f, "Unsupported FBX version: {:?}", v),
            Error::UserDefined(e) => write!(f, "User-defined error: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    #[inline]
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<CompressionError> for Error {
    #[inline]
    fn from(e: CompressionError) -> Self {
        Error::Compression(e)
    }
}

/// Compression error.
#[derive(Debug)]
pub enum CompressionError {
    /// Zlib error.
    Zlib(io::Error),
}

impl error::Error for CompressionError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CompressionError::Zlib(e) => Some(e),
        }
    }
}

impl fmt::Display for CompressionError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompressionError::Zlib(e) => write!(f, "Zlib compression error: {}", e),
        }
    }
}
