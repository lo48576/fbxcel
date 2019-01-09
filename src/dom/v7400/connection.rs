//! `Connections` and `C` node.

use crate::dom::v7400::object::ObjectId;
use crate::dom::v7400::StrSym;

/// Type of a connected node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectedNodeType {
    /// Object.
    Object,
    /// Property.
    Property,
}

/// Connection edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionEdge {
    /// Source node type.
    source_type: ConnectedNodeType,
    /// Destination node type.
    destination_type: ConnectedNodeType,
    /// Label.
    label: Option<StrSym>,
}

/// Connection data (provided by `C` node).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Connection {
    /// Edge data.
    edge: ConnectionEdge,
    /// Source object ID.
    source_id: ObjectId,
    /// Destination object ID.
    destination_id: ObjectId,
}
