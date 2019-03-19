//! Proxy to object properties.

use crate::dom::v7400::{
    object::{property::PropertiesNodeId, ObjectHandle},
    Document,
};

/// Proxy to object properties.
#[derive(Debug, Clone, Copy)]
pub struct ObjectProperties<'a> {
    /// Direct properties node ID.
    direct_props: Option<PropertiesNodeId>,
    /// Default properties node ID.
    default_props: Option<PropertiesNodeId>,
    /// Document.
    doc: &'a Document,
}

impl<'a> ObjectProperties<'a> {
    /// Creates a new `ObjectProperties` for the given object node and native
    /// type name.
    pub(crate) fn from_object(object: &ObjectHandle<'a>, native_type: &str) -> Self {
        let direct_props = object
            .node()
            .children_by_name("Properties70")
            .map(|node| PropertiesNodeId::new(node.node_id()))
            .next();
        let default_props = object
            .document()
            .definitions()
            .properties_node_id(object.node().name(), native_type);

        Self {
            direct_props,
            default_props,
            doc: object.document(),
        }
    }
}
