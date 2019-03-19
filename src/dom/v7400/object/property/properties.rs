//! Properties set object.

use crate::tree::v7400::NodeId;

/// Node ID of a `Properties70` node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PropertiesNodeId(NodeId);

impl PropertiesNodeId {
    /// Creates a new `PropertiesNodeId`.
    pub(crate) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl std::ops::Deref for PropertiesNodeId {
    type Target = NodeId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PropertiesNodeId> for NodeId {
    fn from(v: PropertiesNodeId) -> Self {
        v.0
    }
}
