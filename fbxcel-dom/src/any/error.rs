//! Error and result types for `tree::any` module.

use std::{error, fmt};

use fbxcel::tree;

/// AnyTree load result.
pub type Result<T> = std::result::Result<T, Error>;

/// Error.
#[derive(Debug)]
pub enum Error {
    /// Tree load error.
    Tree(tree::any::Error),
    /// DOM load error.
    Dom(Box<dyn error::Error + Send + Sync + 'static>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Tree(e) => Some(e),
            Error::Dom(e) => Some(&**e),
            Error::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Tree(e) => write!(f, "Tree load error: {}", e),
            Error::Dom(e) => write!(f, "DOM document load error: {}", e),
            Error::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

impl From<tree::any::Error> for Error {
    fn from(e: tree::any::Error) -> Self {
        Error::Tree(e)
    }
}

impl From<crate::v7400::LoadError> for Error {
    fn from(e: crate::v7400::LoadError) -> Self {
        Error::Dom(e.into())
    }
}
