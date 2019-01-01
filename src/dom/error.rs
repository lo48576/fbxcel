//! DOM error.

use std::error;
use std::fmt;

use crate::pull_parser::Error as ParserError;

/// Error on DOM load.
#[derive(Debug)]
pub enum LoadError {
    /// Bad parser.
    ///
    /// This error will be mainly caused by user logic error.
    BadParser,
    /// Parser error.
    Parser(ParserError),
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::BadParser => f.write_str("Bad parser is given"),
            LoadError::Parser(e) => write!(f, "Parser error: {}", e),
        }
    }
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            LoadError::Parser(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParserError> for LoadError {
    fn from(e: ParserError) -> Self {
        LoadError::Parser(e)
    }
}
