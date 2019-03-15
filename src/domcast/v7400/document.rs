//! FBX DOM.

use crate::tree::v7400::Tree;

/// FBX DOM.
#[derive(Debug, Clone)]
pub struct Document {
    /// FBX data tree.
    tree: Tree,
}
