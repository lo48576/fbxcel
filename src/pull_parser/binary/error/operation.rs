//! Invalid operation.

use std::error;
use std::fmt;

use super::super::{FbxVersion, ParserVersion};

/// Invalid operation.
#[derive(Debug)]
pub enum OperationError {
    /// Attempt to create a parser with unsupported FBX version.
    UnsupportedFbxVersion(ParserVersion, FbxVersion),
}

impl error::Error for OperationError {}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OperationError::UnsupportedFbxVersion(parser, fbx) => write!(
                f,
                "Unsupported FBX version: parser={:?}, fbx={:?}",
                parser, fbx
            ),
        }
    }
}
