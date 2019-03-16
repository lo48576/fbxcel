//! Objects-related stuff.

use crate::tree::v7400::NodeId;

pub(crate) use self::{
    cache::ObjectsCache,
    meta::{ObjectClassSym, ObjectMeta},
};

mod cache;
mod meta;

/// Node ID of a object node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectNodeId(NodeId);

impl ObjectNodeId {
    /// Creates a new `ObjectNodeId`.
    pub(crate) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }
}

impl From<ObjectNodeId> for NodeId {
    fn from(v: ObjectNodeId) -> Self {
        v.0
    }
}

/// Object ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(i64);

impl ObjectId {
    /// Creates a new `ObjectId`.
    fn new(id: i64) -> Self {
        Self(id)
    }
}
