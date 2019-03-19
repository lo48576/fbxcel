//! FBX DOM.

use crate::{
    dom::v7400::{
        connection::ConnectionsCache,
        definition::DefinitionsCache,
        object::{scene::SceneHandle, ObjectsCache},
    },
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
    /// Object template definitions.
    definitions: DefinitionsCache,
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

    /// Returns a reference to the object template definitions.
    pub(crate) fn definitions(&self) -> &DefinitionsCache {
        &self.definitions
    }

    /// Returns `Document` object nodes, which have root object ID of scenes.
    pub fn scenes(&self) -> impl Iterator<Item = SceneHandle<'_>> {
        self.objects.document_nodes().iter().map(move |obj_id| {
            SceneHandle::new(obj_id.to_object_handle(self))
                .expect("Should never fail: Actually using `Document` objects")
        })
    }
}

impl AsRef<Tree> for Document {
    fn as_ref(&self) -> &Tree {
        &self.tree
    }
}
