//! FBX DOM document.

use std::collections::HashMap;

use crate::dom::v7400::object::{ObjectId, ObjectNodeId};
use crate::dom::v7400::{Core, Node, NodeId, StrSym};

pub use self::loader::Loader;

mod loader;

/// FBX DOM document.
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// DOM core.
    core: Core,
    /// Map from object ID to node ID.
    object_ids: HashMap<ObjectId, ObjectNodeId>,
}

impl Document {
    /// Creates a new `Document`.
    pub(crate) fn new(core: Core, object_ids: HashMap<ObjectId, ObjectNodeId>) -> Self {
        Self { core, object_ids }
    }

    /// Resolves the given interned string symbol into the corresponding string.
    ///
    /// Returns `None` if the given symbol is registered to the document.
    pub(crate) fn string(&self, sym: StrSym) -> Option<&str> {
        self.core.string(sym)
    }

    /// Returns the node from the node ID.
    ///
    /// # Panics
    ///
    /// Panics if the node with the given ID is not available.
    pub(crate) fn node(&self, id: NodeId) -> Node<'_> {
        self.core.node(id)
    }

    /// Returns the root node ID.
    pub fn root(&self) -> NodeId {
        self.core.root()
    }
}
