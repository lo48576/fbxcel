//! Proxy to object properties.

use crate::v7400::{
    object::{
        property::{PropertiesHandle, PropertiesNodeId, PropertyHandle},
        ObjectHandle,
    },
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
            .definitions_cache()
            .properties_node_id(object.node().name(), native_type);

        Self {
            direct_props,
            default_props,
            doc: object.document(),
        }
    }

    /// Returns property handle if found.
    pub fn get_property(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.get_direct_property(name)
            .or_else(|| self.get_default_property(name))
    }

    /// Returns property handle of the direct property if found.
    pub(crate) fn get_direct_property(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.direct_props
            .map(|id| PropertiesHandle::new(id, self.doc))
            .and_then(|props| props.get_property(name))
    }

    /// Returns property handle of the default property if found.
    pub(crate) fn get_default_property(&self, name: &str) -> Option<PropertyHandle<'a>> {
        self.default_props
            .map(|id| PropertiesHandle::new(id, self.doc))
            .and_then(|props| props.get_property(name))
    }
}
