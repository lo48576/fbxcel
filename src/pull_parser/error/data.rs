//! Data error.
//!
//! This is mainly syntax and low-level structure error.

use std::error;
use std::fmt;
use std::string::FromUtf8Error;

/// Data error.
#[derive(Debug)]
pub enum DataError {
    /// Data with broken compression.
    BrokenCompression(Compression, Box<dyn std::error::Error + Send + Sync>),
    /// FBX footer is broken.
    ///
    /// Detail is not available because the footer may contain variable length
    /// field, and it is hard to identify what is actually broken.
    BrokenFbxFooter,
    /// Got an unknown array attribute encoding.
    InvalidArrayAttributeEncoding(u32),
    /// Invalid node attribute type code.
    ///
    /// The `u8` is the code the parser got.
    InvalidAttributeTypeCode(u8),
    /// Invalid node name encoding.
    ///
    /// This error indicates that the node name is non-valid UTF-8.
    InvalidNodeNameEncoding(FromUtf8Error),
    /// Node attribute error.
    ///
    /// This error indicates that some error happened while reading node
    /// attributes.
    NodeAttributeError,
    /// Node length mismatch.
    ///
    /// This error indicates that a node ends at the position which differs from
    /// the offset declared at the header.
    ///
    /// The former `u64` is expected position, the latter `Option<u64>` is the
    /// actual position the node ends.
    /// If the error is detected before the node actually ends, the actual
    /// position will be `None`.
    NodeLengthMismatch(u64, Option<u64>),
    /// Unexpected attribute value or type.
    ///
    /// The former is the expected, the latter is a description of the actual value.
    UnexpectedAttribute(String, String),
}

impl error::Error for DataError {
    fn cause(&self) -> Option<&error::Error> {
        match self {
            DataError::BrokenCompression(_, e) => Some(e.as_ref()),
            DataError::InvalidNodeNameEncoding(e) => Some(e),
            _ => None,
        }
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::BrokenFbxFooter => write!(f, "FBX footer is broken"),
            DataError::BrokenCompression(codec, e) => write!(
                f,
                "Data with broken compression (codec={:?}): {:?}",
                codec, e
            ),
            DataError::InvalidArrayAttributeEncoding(encoding) => {
                write!(f, "Unknown array attribute encoding: got {:?}", encoding)
            }
            DataError::InvalidAttributeTypeCode(code) => {
                write!(f, "Invalid node attribute type code: {:?}", code)
            }
            DataError::InvalidNodeNameEncoding(e) => {
                write!(f, "Invalid node name encoding: {:?}", e)
            }
            DataError::NodeAttributeError => {
                write!(f, "Some error occured while reading node attributes")
            }
            DataError::NodeLengthMismatch(expected, got) => write!(
                f,
                "Node ends with unexpected position: expected {}, got {:?}",
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

/// Compression format or algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Compression {
    /// ZLIB compression.
    Zlib,
}
