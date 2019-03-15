//! Node-local data.

use crate::{pull_parser::v7400::attribute::DirectAttributeValue, tree::v7400::node::NodeNameSym};

/// Node-local data in FBX data tree.
///
/// This does not manages relations among nodes (including parent-child
/// relatinos).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NodeData {
    /// Node name.
    name_sym: NodeNameSym,
    /// Node attributes.
    attributes: Vec<DirectAttributeValue>,
}

impl NodeData {
    /// Returns the node name symbol.
    pub(crate) fn name_sym(&self) -> NodeNameSym {
        self.name_sym
    }

    /// Returns the reference to the attributes.
    pub(crate) fn attributes(&self) -> &[DirectAttributeValue] {
        &self.attributes
    }

    /// Creates a new `NodeData`.
    pub(crate) fn new(name_sym: NodeNameSym, attributes: Vec<DirectAttributeValue>) -> Self {
        Self {
            name_sym,
            attributes,
        }
    }
}
