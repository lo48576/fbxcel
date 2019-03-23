//! Connection-related error.

use std::{error, fmt};

use fbxcel::{low::v7400::AttributeType, tree::v7400::NodeId};

use crate::v7400::{connection::ConnectionIndex, object::ObjectId, LoadError};

/// Object metadata load error.
#[derive(Debug, Clone)]
pub(crate) enum ConnectionError {
    /// Duplicate object ID.
    DuplicateConnection(
        ObjectId,
        ObjectId,
        Option<String>,
        NodeId,
        ConnectionIndex,
        NodeId,
        ConnectionIndex,
    ),
    /// Node types not found.
    MissingNodeTypes(NodeId, ConnectionIndex),
    /// Invalid type value of node types.
    InvalidNodeTypesType(NodeId, ConnectionIndex, AttributeType),
    /// Invalid value of node types.
    InvalidNodeTypesValue(NodeId, ConnectionIndex, String),
    /// Source object ID not found.
    MissingSourceId(NodeId, ConnectionIndex),
    /// Invalid source ID value type.
    InvalidSourceIdType(NodeId, ConnectionIndex, AttributeType),
    /// Destination object ID not found.
    MissingDestinationId(NodeId, ConnectionIndex),
    /// Invalid source ID value type.
    InvalidDestinationIdType(NodeId, ConnectionIndex, AttributeType),
    /// Invalid connection label value type.
    InvalidLabelType(NodeId, ConnectionIndex, AttributeType),
}

impl fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionError::DuplicateConnection(
                source,
                dest,
                label,
                conn_node1,
                conn_index1,
                conn_node2,
                conn_index2,
            ) => write!(
                f,
                "Duplicate connections: source={:?}, dest={:?}, label={:?} \
                 node1={:?}, index1={:?}, node2={:?}, index2={:?}",
                source, dest, label, conn_node1, conn_index1, conn_node2, conn_index2
            ),
            ConnectionError::MissingNodeTypes(node, conn_index) => write!(
                f,
                "Connection node types not found: node={:?}, conn_index={:?}",
                node, conn_index
            ),
            ConnectionError::InvalidNodeTypesType(node, conn_index, ty) => write!(
                f,
                "Invalid node types value type for node={:?}, conn_index={:?}: \
                 expected string, got {:?}",
                node, conn_index, ty
            ),
            ConnectionError::InvalidNodeTypesValue(node, conn_index, val) => write!(
                f,
                "Invalid node types value for node={:?}, conn_index={:?}: got {:?}",
                node, conn_index, val
            ),
            ConnectionError::MissingSourceId(node, conn_index) => write!(
                f,
                "Connection source object ID not found: node={:?}, conn_index={:?}",
                node, conn_index
            ),
            ConnectionError::InvalidSourceIdType(node, conn_index, ty) => write!(
                f,
                "Invalid source object ID value type for node={:?}, conn_index={:?}: \
                 expected `i64`, got {:?}",
                node, conn_index, ty
            ),
            ConnectionError::MissingDestinationId(node, conn_index) => write!(
                f,
                "Connection destination object ID not found: node={:?}, conn_index={:?}",
                node, conn_index
            ),
            ConnectionError::InvalidDestinationIdType(node, conn_index, ty) => write!(
                f,
                "Invalid destination object ID value type for node={:?}, conn_index={:?}: \
                 expected `i64`, got {:?}",
                node, conn_index, ty
            ),
            ConnectionError::InvalidLabelType(node, conn_index, ty) => write!(
                f,
                "Invalid connection label value type for node={:?}, conn_index={:?}: \
                 expected string, got {:?}",
                node, conn_index, ty
            ),
        }
    }
}

impl error::Error for ConnectionError {}

impl From<ConnectionError> for LoadError {
    fn from(e: ConnectionError) -> Self {
        Self::new(e)
    }
}
