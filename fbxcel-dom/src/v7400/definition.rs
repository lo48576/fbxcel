//! Object template definitions.

use std::collections::HashMap;

use failure::{format_err, Error};
use fbxcel::tree::v7400::{NodeHandle, Tree};
use log::warn;

use crate::v7400::{object::property::PropertiesNodeId, LoadError};

/// Object template definitions cache.
#[derive(Default, Debug, Clone)]
pub(crate) struct DefinitionsCache {
    /// Node IDs of property templates.
    ///
    /// Inner key is (maybe) the native node type in FBX SDK, such as
    /// `FbxCamera`, `FbxMesh`, and `FbxSurfacePhong`.
    templates: HashMap<String, HashMap<String, PropertiesNodeId>>,
}

impl DefinitionsCache {
    /// Returns the properties node ID if available.
    pub(crate) fn properties_node_id(
        &self,
        obj_node_name: &str,
        native_type: &str,
    ) -> Option<PropertiesNodeId> {
        self.templates.get(obj_node_name)?.get(native_type).cloned()
    }

    /// Creates a new `DefinitionsCache` from the given FBX data tree.
    pub(crate) fn from_tree(tree: &Tree) -> Result<Self, LoadError> {
        let mut this = Self::default();

        if let Some(definitions_node) = tree.root().children_by_name("Definitions").next() {
            this.load_definitions(definitions_node)?;
        }

        Ok(this)
    }

    /// Loads the given `Definitions` node.
    fn load_definitions(&mut self, node: NodeHandle<'_>) -> Result<(), LoadError> {
        for obj_type_node in node.children_by_name("ObjectType") {
            if let Err(e) = self.load_object_type(obj_type_node) {
                warn!(
                    "Ignoring error: Failed to get object node type from \
                     `ObjectType` node (node_id={:?}): {}",
                    obj_type_node.node_id(),
                    e
                );
            }
        }

        Ok(())
    }

    /// Loads the given `ObjectType` node.
    fn load_object_type(&mut self, node: NodeHandle<'_>) -> Result<(), Error> {
        let obj_type = node
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found"))?
            .get_string_or_type()
            .map_err(|ty| {
                format_err!("Expected string as the first attribute, but got {:?}", ty)
            })?;
        for property_template_node in node.children_by_name("PropertyTemplate") {
            if let Err(e) = self.load_property_template(property_template_node, obj_type) {
                warn!(
                    "Ignoring error: Failed to get object node type from \
                     `PropertyTemplate` node (node_id={:?}): {}",
                    property_template_node.node_id(),
                    e
                );
            }
        }

        Ok(())
    }

    /// Loads the given `PropertyTemplate` node.
    fn load_property_template(
        &mut self,
        node: NodeHandle<'_>,
        obj_type: &str,
    ) -> Result<(), Error> {
        let native_type = node
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("No attributes found"))?
            .get_string_or_type()
            .map_err(|ty| {
                format_err!("Expected string as the first attribute, but got {:?}", ty)
            })?;
        let properties_node = match node.children_by_name("Properties70").next() {
            Some(v) => v,
            None => return Ok(()),
        };
        self.templates
            .entry(obj_type.into())
            .or_insert_with(Default::default)
            .insert(
                native_type.into(),
                PropertiesNodeId::new(properties_node.node_id()),
            );
        Ok(())
    }
}
