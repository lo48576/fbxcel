//! FBX data tree for v7.4 or later.

use indextree::Arena;
use string_interner::StringInterner;

use crate::low::v7400::AttributeValue;

use self::node::{NodeData, NodeNameSym};
pub use self::{
    error::LoadError,
    loader::Loader,
    node::{NodeHandle, NodeId},
};

mod error;
mod loader;
mod node;

/// FBX data tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    /// Tree data.
    arena: Arena<NodeData>,
    /// Node name interner.
    node_names: StringInterner<NodeNameSym>,
    /// (Implicit) root node ID.
    root_id: NodeId,
}

impl Tree {
    /// Returns the root node.
    pub fn root(&self) -> NodeHandle<'_> {
        NodeHandle::new(&self, self.root_id)
    }

    /// Creates a new `Tree`.
    fn new(
        arena: Arena<NodeData>,
        node_names: StringInterner<NodeNameSym>,
        root_id: NodeId,
    ) -> Self {
        Self {
            arena,
            node_names,
            root_id,
        }
    }

    /// Returns internally managed node data.
    ///
    /// # Panics
    /// Panics if a node with the given node ID does not exist in the tree.
    pub(crate) fn node(&self, node_id: NodeId) -> &indextree::Node<NodeData> {
        self.arena.get(node_id.raw()).unwrap_or_else(|| {
            panic!(
                "The given node ID is not used in the tree: node_id={:?}",
                node_id
            )
        })
    }

    /// Returns the string corresponding to the node name symbol.
    ///
    /// # Panics
    ///
    /// Panics if the given symbol is not used in the tree.
    pub(crate) fn resolve_node_name(&self, sym: NodeNameSym) -> &str {
        self.node_names
            .resolve(sym)
            .unwrap_or_else(|| panic!("Unresolvable node name symbol: {:?}", sym))
    }

    /// Returns node name symbol if available.
    pub(crate) fn node_name_sym(&self, name: &str) -> Option<NodeNameSym> {
        self.node_names.get(name)
    }

    /// Checks whether or not the given node ID is used in the tree.
    pub(crate) fn contains_node(&self, node_id: NodeId) -> bool {
        self.arena.get(node_id.raw()).is_some()
    }

    /// Creates a new node and appends to the given parent node.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is not used in the tree.
    pub fn append_new(&mut self, parent: NodeId, name: &str) -> NodeId {
        let name_sym = self.node_names.get_or_intern(name);
        let new_child = self.arena.new_node(NodeData::new(name_sym, Vec::new()));
        parent
            .raw()
            .append(new_child, &mut self.arena)
            .expect("Should never fail");

        NodeId::new(new_child)
    }

    /// Creates a new node and prepends to the given parent node.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is not used in the tree.
    pub fn prepend_new(&mut self, parent: NodeId, name: &str) -> NodeId {
        let name_sym = self.node_names.get_or_intern(name);
        let new_child = self.arena.new_node(NodeData::new(name_sym, Vec::new()));
        parent
            .raw()
            .prepend(new_child, &mut self.arena)
            .expect("Should never fail");

        NodeId::new(new_child)
    }

    /// Creates a new node and inserts after the given sibling node.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    pub fn insert_new_after(&mut self, sibling: NodeId, name: &str) -> NodeId {
        assert_ne!(sibling, self.root_id, "Root node should have no siblings");
        let name_sym = self.node_names.get_or_intern(name);
        let new_child = self.arena.new_node(NodeData::new(name_sym, Vec::new()));
        sibling
            .raw()
            .insert_after(new_child, &mut self.arena)
            .expect("Should never fail");

        NodeId::new(new_child)
    }

    /// Creates a new node and inserts before the given sibling node.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    pub fn insert_new_before(&mut self, sibling: NodeId, name: &str) -> NodeId {
        assert_ne!(sibling, self.root_id, "Root node should have no siblings");
        let name_sym = self.node_names.get_or_intern(name);
        let new_child = self.arena.new_node(NodeData::new(name_sym, Vec::new()));
        sibling
            .raw()
            .insert_before(new_child, &mut self.arena)
            .expect("Should never fail");

        NodeId::new(new_child)
    }

    /// Creates a new node and inserts before the given sibling node.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    pub fn append_attribute(&mut self, node_id: NodeId, v: AttributeValue) {
        assert_ne!(node_id, self.root_id, "Root node should have no attributes");
        let node = self.arena.get_mut(node_id.raw()).expect("Invalid node ID");
        node.data.append_attribute(v)
    }
}

impl Default for Tree {
    fn default() -> Self {
        let mut arena = Arena::new();
        let mut node_names = StringInterner::new();
        let root_id =
            NodeId::new(arena.new_node(NodeData::new(node_names.get_or_intern(""), Vec::new())));

        Self {
            arena,
            node_names,
            root_id,
        }
    }
}
