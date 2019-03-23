//! Object-related error.

use std::{error, fmt};

use fbxcel::{low::v7400::AttributeType, tree::v7400::NodeId};

use crate::v7400::{object::ObjectId, LoadError};

/// Object metadata load error.
#[derive(Debug, Clone)]
pub(crate) enum ObjectMetaError {
    /// Duplicate object ID.
    DuplicateObjectId(ObjectId, NodeId, NodeId),
    /// Object ID not found.
    MissingId(NodeId),
    /// Invalid ID value type.
    InvalidIdType(NodeId, AttributeType),
    /// Name and class not found.
    MissingNameClass(NodeId, ObjectId),
    /// Invalid name and class value type.
    InvalidNameClassType(NodeId, ObjectId, AttributeType),
    /// Subclass not found.
    MissingSubclass(NodeId, ObjectId),
    /// Invalid subclass value type.
    InvalidSubclassType(NodeId, ObjectId, AttributeType),
}

impl fmt::Display for ObjectMetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectMetaError::DuplicateObjectId(obj, node1, node2) => write!(
                f,
                "Duplicate object ID: object={:?}, node1={:?}, node2={:?}",
                obj, node1, node2
            ),
            ObjectMetaError::MissingId(node) => write!(f, "Object ID not found: node={:?}", node),
            ObjectMetaError::InvalidIdType(node, ty) => write!(
                f,
                "Invalid object ID value type for node={:?}: expected `i64`, got {:?}",
                node, ty
            ),
            ObjectMetaError::MissingNameClass(node, obj) => write!(
                f,
                "Object name and class not found: node={:?}, object={:?}",
                node, obj
            ),
            ObjectMetaError::InvalidNameClassType(node, obj, ty) => write!(
                f,
                "Invalid object name and class value type for node={:?}, obj={:?}: \
                 expected string, got {:?}",
                node, obj, ty
            ),
            ObjectMetaError::MissingSubclass(node, obj) => write!(
                f,
                "Object subclass not found: node={:?}, object={:?}",
                node, obj
            ),
            ObjectMetaError::InvalidSubclassType(node, obj, ty) => write!(
                f,
                "Invalid object subclass value type for node={:?}, obj={:?}: \
                 expected string, got {:?}",
                node, obj, ty
            ),
        }
    }
}

impl error::Error for ObjectMetaError {}

impl From<ObjectMetaError> for LoadError {
    fn from(e: ObjectMetaError) -> Self {
        Self::new(e)
    }
}
