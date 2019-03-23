//! FBX DOM load error.

use std::{error, fmt};

use fbxcel::tree::v7400::LoadError as TreeLoadError;

/// FBX DOM load error.
#[derive(Debug)]
pub struct LoadError(Box<dyn error::Error + Send + Sync + 'static>);

impl LoadError {
    /// Creates a new `LoadError`.
    pub(crate) fn new(e: impl Into<Box<dyn error::Error + Send + Sync + 'static>>) -> Self {
        Self(e.into())
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for LoadError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&*self.0)
    }
}

impl From<TreeLoadError> for LoadError {
    fn from(e: TreeLoadError) -> Self {
        Self::new(e)
    }
}

/// FBX DOM structure error.
#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum StructureError {
    /// Toplevel `Connections` node not found.
    MissingConnectionsNode,
    /// Toplevel `Documents` node not found.
    MissingDocumentsNode,
    /// Toplevel `Objects` node not found.
    MissingObjectsNode,
}

impl fmt::Display for StructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructureError::MissingConnectionsNode => {
                f.write_str("Toplevel `Connections` node not found")
            }
            StructureError::MissingDocumentsNode => {
                f.write_str("Toplevel `Documents` node not found")
            }
            StructureError::MissingObjectsNode => f.write_str("Toplevel `Objects` node not found"),
        }
    }
}

impl error::Error for StructureError {}

impl From<StructureError> for LoadError {
    fn from(e: StructureError) -> Self {
        Self::new(e)
    }
}
