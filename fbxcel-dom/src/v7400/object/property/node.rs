//! Property node.

use failure::{format_err, Error};
use fbxcel::{
    low::v7400::AttributeValue,
    tree::v7400::{NodeHandle, NodeId},
};
use log::warn;

use crate::v7400::{object::property::LoadProperty, Document};

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

/// Node handle of a `P` node under `Properties70` node.
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

    /// Returns a node handle.
    pub(crate) fn node(&self) -> NodeHandle<'a> {
        self.node_id.to_handle(self.doc.tree())
    }

    /// Returns a node ID.
    pub fn node_id(&self) -> PropertyNodeId {
        self.node_id
    }

    /// Returns a reference to the document.
    pub fn document(&self) -> &'a Document {
        self.doc
    }

    /// Reads a value from the property handle if possible.
    pub fn load_value<V: LoadProperty<'a>>(&self, loader: V) -> Result<V::Value, V::Error> {
        loader.load(self)
    }

    /// Returns proprety name.
    pub fn name(&self) -> Result<&'a str, Error> {
        self.get_string_attr(0)
            .map_err(|e| format_err!("Failed to get property name: {}", e))
    }

    /// Returns proprety type name.
    pub fn data_type(&self) -> Result<&'a str, Error> {
        self.get_string_attr(1)
            .map_err(|e| format_err!("Failed to get property data type: {}", e))
    }

    /// Returns proprety label.
    pub fn label(&self) -> Result<&'a str, Error> {
        self.get_string_attr(2)
            .map_err(|e| format_err!("Failed to get property label: {}", e))
    }

    /// Returns property value part of node attributes.
    pub fn value_part(&self) -> &'a [AttributeValue] {
        self.node().attributes().get(4..).unwrap_or_else(|| {
            warn!(
                "Ignoring error: Not enough node attribute for proprerty node: \
                 node_id={:?}, num_attrs={}",
                self.node_id,
                self.node().attributes().len()
            );
            &[]
        })
    }

    /// For internal use: returns string attribute.
    fn get_string_attr(&self, index: usize) -> Result<&'a str, Error> {
        self.node()
            .attributes()
            .get(index)
            .ok_or_else(|| {
                format_err!(
                    "No properties found: node_id={:?}, attr_index={:?}",
                    self,
                    index
                )
            })?
            .get_string_or_type()
            .map_err(|ty| {
                format_err!(
                    "Expected string but got {:?}: node_id={:?}, attr_index={:?}",
                    ty,
                    self,
                    index
                )
            })
    }
}
