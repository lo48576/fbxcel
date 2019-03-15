//! FBX data tree for v7.4 or later.

use indextree::Arena;
use string_interner::StringInterner;

use self::node::{NodeData, NodeNameSym};
pub use self::node::{NodeHandle, NodeId};

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

    /// Checks whether or not the given node ID is used in the tree.
    pub(crate) fn contains_node(&self, node_id: NodeId) -> bool {
        self.arena.get(node_id.raw()).is_some()
    }
}
