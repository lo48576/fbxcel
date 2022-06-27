//! Error and result types for `tree::any` module.

use std::{error, fmt};

use crate::{pull_parser, tree};

/// AnyTree load result.
pub type Result<T> = std::result::Result<T, Error>;

/// Error.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Parser creation error.
    ParserCreation(pull_parser::any::Error),
    /// Parser error.
    Parser(pull_parser::Error),
    /// Tree load error.
    Tree(Box<dyn error::Error + Send + Sync + 'static>),
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ParserCreation(e) => Some(e),
            Error::Parser(e) => Some(e),
            Error::Tree(e) => Some(&**e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ParserCreation(e) => write!(f, "Failed to create a parser: {}", e),
            Error::Parser(e) => write!(f, "Parser error: {}", e),
            Error::Tree(e) => write!(f, "Tree load error: {}", e),
        }
    }
}

impl From<pull_parser::any::Error> for Error {
    #[inline]
    fn from(e: pull_parser::any::Error) -> Self {
        Error::ParserCreation(e)
    }
}

impl From<pull_parser::Error> for Error {
    #[inline]
    fn from(e: pull_parser::Error) -> Self {
        Error::Parser(e)
    }
}

impl From<tree::v7400::LoadError> for Error {
    #[inline]
    fn from(e: tree::v7400::LoadError) -> Self {
        Error::Tree(e.into())
    }
}
