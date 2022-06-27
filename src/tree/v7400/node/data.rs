//! Node-local data.

use crate::{low::v7400::AttributeValue, tree::v7400::node::NodeNameSym};

/// Node-local data in FBX data tree.
///
/// This does not manages relations among nodes (including parent-child
/// relatinos).
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NodeData {
    /// Node name.
    name_sym: NodeNameSym,
    /// Node attributes.
    attributes: Vec<AttributeValue>,
}

impl NodeData {
    /// Returns the node name symbol.
    #[inline]
    #[must_use]
    pub(crate) fn name_sym(&self) -> NodeNameSym {
        self.name_sym
    }

    /// Returns the reference to the attributes.
    #[inline]
    #[must_use]
    pub(crate) fn attributes(&self) -> &[AttributeValue] {
        &self.attributes
    }

    /// Appends the given value to the attributes.
    #[inline]
    pub(crate) fn append_attribute(&mut self, v: AttributeValue) {
        self.attributes.push(v)
    }

    /// Appends the given value to the attributes.
    #[inline]
    pub(crate) fn get_attribute_mut(&mut self, i: usize) -> Option<&mut AttributeValue> {
        self.attributes.get_mut(i)
    }

    /// Replaces all attributes by the given one, and returns the old.
    #[inline]
    pub(crate) fn replace_attributes(&mut self, new: Vec<AttributeValue>) -> Vec<AttributeValue> {
        std::mem::replace(&mut self.attributes, new)
    }

    /// Creates a new `NodeData`.
    #[inline]
    #[must_use]
    pub(crate) fn new(name_sym: NodeNameSym, attributes: Vec<AttributeValue>) -> Self {
        Self {
            name_sym,
            attributes,
        }
    }
}
