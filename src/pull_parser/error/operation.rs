//! Invalid operation.

use std::{error, fmt};

use crate::{low::FbxVersion, pull_parser::ParserVersion};

/// Invalid operation.
#[derive(Debug)]
pub enum OperationError {
    /// Attempt to parse more data while the parsing is aborted.
    AlreadyAborted,
    /// Attempt to parse more data while the parsing is (successfully) finished.
    AlreadyFinished,
    /// Attempt to create a parser with unsupported FBX version.
    UnsupportedFbxVersion(ParserVersion, FbxVersion),
}

impl error::Error for OperationError {}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationError::AlreadyAborted => {
                write!(f, "Attempt to parse more data while the parsing is aborted")
            }
            OperationError::AlreadyFinished => write!(
                f,
                "Attempt to parse more data while the parsing is successfully finished"
            ),
            OperationError::UnsupportedFbxVersion(parser, fbx) => write!(
                f,
                "Unsupported FBX version: parser={:?}, fbx={:?}",
                parser, fbx
            ),
        }
    }
}
