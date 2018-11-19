//! Data error.
//!
//! This is mainly syntax and low-level structure error.

use std::error;
use std::fmt;
use std::string::FromUtf8Error;

/// Data error.
#[derive(Debug)]
pub enum DataError {
    /// Invalid node attribute type code.
    ///
    /// The `u8` is the code the parser got.
    InvalidAttributeTypeCode(u8),
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
    /// Unexpected attribute value or type.
    ///
    /// The former is the expected, the latter is a description of the actual value.
    UnexpectedAttribute(String, String),
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
            DataError::InvalidAttributeTypeCode(code) => {
                write!(f, "Invalid node attribute type code: {:?}", code)
            }
            DataError::InvalidNodeNameEncoding(e) => {
                write!(f, "Invalid node name encoding: {:?}", e)
            }
            DataError::NodeLengthMismatch(expected, got) => write!(
                f,
                "Node ends with unexpected position: expected {}, got {}",
                expected, got
            ),
            DataError::UnexpectedAttribute(expected, got) => write!(
                f,
                "Unexpected attribute value or type: expected {}, got {}",
                expected, got
            ),
        }
    }
}
