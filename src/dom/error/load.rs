//! DOM load error.

use std::error;
use std::fmt;

use crate::dom::v7400::error::CoreLoadError;
use crate::dom::AccessError;
use crate::pull_parser::Error as ParserError;

/// Error on DOM load.
#[derive(Debug)]
pub enum LoadError {
    /// Node data access error.
    Access(AccessError),
    /// Bad parser.
    ///
    /// This error will be mainly caused by user logic error.
    BadParser,
    /// DOM core error.
    Core(CoreLoadError),
    /// Duplicate connection.
    ///
    /// The first is kind of ID, the second and the third is content of ID.
    ///
    /// Use `String` to make it version independent.
    DuplicateConnection(String, String, String),
    /// Duplicate ID.
    ///
    /// The first is kind of ID, the second is content of ID.
    ///
    /// Use `String` to make it version independent.
    DuplicateId(String, String),
    /// Parser error.
    Parser(ParserError),
    /// Unexpected object type.
    ///
    /// The first is expected type, the second is actual type.
    UnexpectedObjectType(String, String),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadError::Access(e) => write!(f, "Node data access error: {}", e),
            LoadError::BadParser => f.write_str("Bad parser is given"),
            LoadError::Core(e) => write!(f, "DOM core load error: {}", e),
            LoadError::DuplicateConnection(kind, from, to) => write!(
                f,
                "Duplicate Connection ({}): from {} to {}",
                kind, from, to
            ),
            LoadError::DuplicateId(kind, id) => write!(f, "Duplicate ID ({}): {}", kind, id),
            LoadError::Parser(e) => write!(f, "Parser error: {}", e),
            LoadError::UnexpectedObjectType(expected, got) => write!(
                f,
                "Unexpected object type: expected {}, got {}",
                expected, got
            ),
            LoadError::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            LoadError::Access(e) => Some(e),
            LoadError::Core(e) => Some(e),
            LoadError::Parser(e) => Some(e),
            _ => None,
        }
    }
}

impl From<AccessError> for LoadError {
    fn from(e: AccessError) -> Self {
        LoadError::Access(e)
    }
}

impl From<CoreLoadError> for LoadError {
    fn from(e: CoreLoadError) -> Self {
        LoadError::Core(e)
    }
}

impl From<ParserError> for LoadError {
    fn from(e: ParserError) -> Self {
        LoadError::Parser(e)
    }
}
