//! FBX DOM load error.

use std::{error, fmt};

use crate::tree::v7400::LoadError as TreeLoadError;

/// FBX DOM load error.
#[derive(Debug)]
pub struct LoadError(LoadErrorImpl);

impl LoadError {
    /// Creates a new `LoadError`.
    fn new(e: impl Into<LoadErrorImpl>) -> Self {
        Self(e.into())
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.0 {
            LoadErrorImpl::Tree(ref e) => Some(e),
        }
    }
}

impl From<TreeLoadError> for LoadError {
    fn from(e: TreeLoadError) -> Self {
        Self::new(e)
    }
}

/// Internal implementation of `LoadError`.
#[derive(Debug)]
enum LoadErrorImpl {
    /// Tree load error.
    Tree(TreeLoadError),
}

impl fmt::Display for LoadErrorImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoadErrorImpl::Tree(e) => write!(f, "Tree load error: {}", e),
        }
    }
}

impl From<TreeLoadError> for LoadErrorImpl {
    fn from(e: TreeLoadError) -> Self {
        LoadErrorImpl::Tree(e)
    }
}
