//! FBX DOM.

use crate::{
    domcast::v7400::{connection::ConnectionsCache, object::ObjectsCache},
    tree::v7400::Tree,
};

pub use self::loader::Loader;

mod loader;

/// FBX DOM.
#[derive(Debug, Clone)]
pub struct Document {
    /// FBX data tree.
    tree: Tree,
    /// Objects cache.
    objects: ObjectsCache,
    /// Objects connection cache.
    connections: ConnectionsCache,
}

impl Document {
    /// Returns a reference to the tree.
    pub fn tree(&self) -> &Tree {
        &self.tree
    }

    /// Returns a reference to the objects cache.
    pub(crate) fn objects(&self) -> &ObjectsCache {
        &self.objects
    }

    /// Returns a reference to the connections cache.
    pub(crate) fn connections(&self) -> &ConnectionsCache {
        &self.connections
    }
}

impl AsRef<Tree> for Document {
    fn as_ref(&self) -> &Tree {
        &self.tree
    }
}
