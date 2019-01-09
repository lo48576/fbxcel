//! DOM node.

use indextree;
use string_interner;

use crate::dom::v7400::Document;
use crate::pull_parser::v7400::attribute::DirectAttributeValue;

/// Symbol for interned string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrSym(usize);

impl string_interner::Symbol for StrSym {
    fn from_usize(v: usize) -> Self {
        StrSym(v)
    }

    fn to_usize(self) -> usize {
        self.0
    }
}

/// A trait for types convertible into `indextree::NodeId`.
///
/// This should be crate-local (should not exposed to crate users), so this is
/// not implemented using `Into` trait.
pub(crate) trait IntoRawNodeId: Copy + std::fmt::Debug {
    /// Returns raw node ID.
    fn raw_node_id(self) -> indextree::NodeId;
}

impl IntoRawNodeId for indextree::NodeId {
    fn raw_node_id(self) -> indextree::NodeId {
        self
    }
}

impl<T: Into<NodeId> + Copy + std::fmt::Debug> IntoRawNodeId for T {
    fn raw_node_id(self) -> indextree::NodeId {
        self.into().0
    }
}

/// FBX tree node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(indextree::NodeId);

impl NodeId {
    /// Creates a new `NodeId`.
    pub(crate) fn new(id: indextree::NodeId) -> Self {
        NodeId(id)
    }

    /// Returns the node from the node ID.
    ///
    /// # Panics
    ///
    /// Panics if the node with the id does not exist in the given document.
    pub fn node(self, doc: &Document) -> Node<'_> {
        doc.node(self)
    }
}

/// Node data (including related node ID info).
#[derive(Debug, Clone, PartialEq)]
pub struct Node<'a> {
    /// Node.
    node: &'a indextree::Node<NodeData>,
}

impl<'a> Node<'a> {
    /// Creates a new `Node`.
    pub(crate) fn new(node: &'a indextree::Node<NodeData>) -> Self {
        Self { node }
    }

    /// Returns the node data.
    pub(crate) fn data(&self) -> &'a NodeData {
        &self.node.data
    }

    /// Returns the node name.
    ///
    /// # Panics
    ///
    /// Panics if the node is not in the given document.
    pub fn name(&self, doc: &'a Document) -> &'a str {
        doc.string(self.data().name)
            .expect("The node is not registered in the document")
    }

    /// Returns the node attributes.
    pub fn attributes(&self) -> &'a [DirectAttributeValue] {
        &self.data().attributes
    }

    /// Returns the node ID of the parent node.
    pub fn parent(&self) -> Option<NodeId> {
        self.node.parent().map(NodeId::new)
    }

    /// Returns the node ID of the first child node.
    pub fn first_child(&self) -> Option<NodeId> {
        self.node.first_child().map(NodeId::new)
    }

    /// Returns the node ID of the last child node.
    pub fn last_child(&self) -> Option<NodeId> {
        self.node.last_child().map(NodeId::new)
    }

    /// Returns the node ID of the previous sibling node.
    pub fn previous_sibling(&self) -> Option<NodeId> {
        self.node.previous_sibling().map(NodeId::new)
    }

    /// Returns the node ID of the next sibling node.
    pub fn next_sibling(&self) -> Option<NodeId> {
        self.node.next_sibling().map(NodeId::new)
    }
}

/// Pure node data (without connections between related nodes).
#[derive(Debug, Clone, PartialEq)]
pub struct NodeData {
    /// Node name.
    name: StrSym,
    /// Node attributes.
    attributes: Vec<DirectAttributeValue>,
}

impl NodeData {
    /// Creates a new `NodeData`.
    pub(crate) fn new(name: StrSym, attributes: Vec<DirectAttributeValue>) -> Self {
        Self { name, attributes }
    }

    /// Returns node name symbol.
    pub(crate) fn name_sym(&self) -> StrSym {
        self.name
    }

    /// Returns node attributes.
    pub(crate) fn attributes(&self) -> &[DirectAttributeValue] {
        &self.attributes
    }
}
