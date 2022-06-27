//! Error types.

use std::{error, fmt};

use crate::pull_parser::Error as ParserError;

/// FBX data tree load error.
#[derive(Debug)]
#[non_exhaustive]
pub enum LoadError {
    /// Bad parser.
    ///
    /// This error will be mainly caused by user logic error.
    BadParser,
    /// Parser error.
    Parser(ParserError),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::BadParser => f.write_str("Attempt to use a bad parser"),
            LoadError::Parser(e) => write!(f, "Parser error: {}", e),
        }
    }
}

impl error::Error for LoadError {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            LoadError::Parser(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParserError> for LoadError {
    #[inline]
    fn from(e: ParserError) -> Self {
        LoadError::Parser(e)
    }
}
