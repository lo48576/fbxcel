//! Property node.

use crate::{dom::v7400::Document, tree::v7400::NodeId};

/// Node ID of a `P` node under `Properties70` node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertyNodeId(NodeId);

impl PropertyNodeId {
    /// Creates a new `PropertyNodeId`.
    pub(crate) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl std::ops::Deref for PropertyNodeId {
    type Target = NodeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PropertyNodeId> for NodeId {
    fn from(v: PropertyNodeId) -> Self {
        v.0
    }
}

/// Node handle of a `Properties70` node.
#[derive(Debug, Clone, Copy)]
pub struct PropertyHandle<'a> {
    /// Node ID.
    node_id: PropertyNodeId,
    /// Document.
    doc: &'a Document,
}

impl<'a> PropertyHandle<'a> {
    /// Creates a new `PropertyNodeId`.
    pub(crate) fn new(node_id: PropertyNodeId, doc: &'a Document) -> Self {
        Self { node_id, doc }
    }
}
