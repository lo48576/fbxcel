//! Node type.

use crate::tree::v7400::{DepthFirstTraverseSubtree, NodeHandle, Tree};

pub(crate) use self::{data::NodeData, name::NodeNameSym};

mod data;
pub(crate) mod handle;
mod name;

/// Node ID in FBX data tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(indextree::NodeId);

impl NodeId {
    /// Creates a new `NodeId`.
    #[inline]
    #[must_use]
    pub(crate) fn new(id: indextree::NodeId) -> Self {
        NodeId(id)
    }

    /// Returns the raw node ID used by internal tree implementation.
    #[inline]
    #[must_use]
    pub(crate) fn raw(self) -> indextree::NodeId {
        self.0
    }

    /// Creates a new `NodeHandle` to make accesible to the node in the tree.
    ///
    /// # Panics and safety
    ///
    /// This may panic if the given node ID is not used in the given tree.
    ///
    /// Even if creation of an invalid node ID does not panic, subsequent
    /// operations through `NodeHandle` object may panic if the given node ID is
    /// not used in the given tree.
    #[inline]
    #[must_use]
    pub fn to_handle(self, tree: &Tree) -> NodeHandle<'_> {
        NodeHandle::new(tree, self)
    }

    /// Returns the helper object to traverse the subtree (the node itself and its descendant).
    #[inline]
    #[must_use]
    pub fn traverse_depth_first(&self) -> DepthFirstTraverseSubtree {
        DepthFirstTraverseSubtree::with_root_id(*self)
    }
}
