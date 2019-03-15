//! FBX DOM.

use crate::tree::v7400::Tree;

pub use self::loader::Loader;

mod loader;

/// FBX DOM.
#[derive(Debug, Clone)]
pub struct Document {
    /// FBX data tree.
    tree: Tree,
}

impl AsRef<Tree> for Document {
    fn as_ref(&self) -> &Tree {
        &self.tree
    }
}
