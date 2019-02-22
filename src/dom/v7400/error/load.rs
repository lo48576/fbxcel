//! FBX v7400 DOM load error.

use std::error;
use std::fmt;

use crate::pull_parser::Error as ParserError;

/// Error on DOM core load.
#[derive(Debug)]
pub enum CoreLoadError {
    /// Bad parser.
    ///
    /// This error will be mainly caused by user logic error.
    BadParser,
    /// Parser error.
    Parser(ParserError),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for CoreLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreLoadError::BadParser => f.write_str("Bad parser is given"),
            CoreLoadError::Parser(e) => write!(f, "Parser error: {}", e),
            CoreLoadError::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

impl error::Error for CoreLoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CoreLoadError::Parser(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ParserError> for CoreLoadError {
    fn from(e: ParserError) -> Self {
        CoreLoadError::Parser(e)
    }
}
