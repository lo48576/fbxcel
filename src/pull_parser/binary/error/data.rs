//! Data error.
//!
//! This is mainly syntax and low-level structure error.

use std::error;
use std::fmt;
use std::string::FromUtf8Error;

/// Data error.
#[derive(Debug)]
pub enum DataError {
    /// Invalid node name encoding.
    ///
    /// This error indicates that the node name is non-valid UTF-8.
    InvalidNodeNameEncoding(FromUtf8Error),
    /// Node length mismatch.
    ///
    /// This error indicates that a node ends at the position which differs from
    /// the offset declared at the header.
    ///
    /// The former `u64` is expected position, the latter `u64` is the actual
    /// position.
    NodeLengthMismatch(u64, u64),
}

impl error::Error for DataError {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            DataError::InvalidNodeNameEncoding(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::InvalidNodeNameEncoding(e) => {
                write!(f, "Invalid node name encoding: {:?}", e)
            }
            DataError::NodeLengthMismatch(expected, got) => write!(
                f,
                "Node ends with unexpected position: expected {}, got {}",
                expected, got
            ),
        }
    }
}
