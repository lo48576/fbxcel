//! Properties set object.

use failure::format_err;
use fbxcel::tree::v7400::{NodeHandle, NodeId};
use log::warn;

use crate::v7400::{
    object::{
        property::{PropertyHandle, PropertyNodeId},
        ObjectHandle,
    },
    Document,
};

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

/// Node handle of a `Properties70` node.
#[derive(Debug, Clone, Copy)]
pub struct PropertiesHandle<'a> {
    /// Node ID.
    node_id: PropertiesNodeId,
    /// Document.
    doc: &'a Document,
}

impl<'a> PropertiesHandle<'a> {
    /// Creates a new `PropertiesNodeId`.
    pub(crate) fn new(node_id: PropertiesNodeId, doc: &'a Document) -> Self {
        Self { node_id, doc }
    }

    /// Creates a new `ObjectProperties` for the given object node and native
    /// type name.
    pub(crate) fn from_object(object: &ObjectHandle<'a>) -> Option<Self> {
        let node_id = object
            .node()
            .children_by_name("Properties70")
            .map(|node| PropertiesNodeId::new(node.node_id()))
            .next()?
            .into();
        Some(Self {
            node_id: PropertiesNodeId::new(node_id),
            doc: object.document(),
        })
    }

    /// Returns a node handle for the properties node.
    pub(crate) fn node(&self) -> NodeHandle<'a> {
        self.node_id.to_handle(self.doc.tree())
    }

    /// Returns a node handle of the property node with the given name.
    pub fn get_property(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.node()
            .children_by_name("P")
            .map(|node| {
                node.attributes()
                    .get(0)
                    .ok_or_else(|| format_err!("No attributes found"))?
                    .get_string_or_type()
                    .map_err(|ty| {
                        format_err!(
                            "Expected string as property name (first attribute), but got {:?}",
                            ty
                        )
                    })
                    .map(|attr| (PropertyNodeId::new(node.node_id()), attr))
            })
            .filter_map(|res| match res {
                Ok((node, attr)) => Some((node, attr)),
                Err(e) => {
                    warn!(
                        "Ignoring error for `P` node (node_id={:?}): {}",
                        self.node_id, e
                    );
                    None
                }
            })
            .find(move |&(_node, v)| v == name)
            .map(|(node_id, _v)| PropertyHandle::new(node_id, self.doc))
    }
}
