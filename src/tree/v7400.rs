//! FBX data tree for v7.4 or later.

use std::fmt;

use indextree::Arena;
use string_interner::{DefaultBackend, StringInterner};

use crate::low::v7400::AttributeValue;

use self::node::{NodeData, NodeNameSym};
pub use self::{
    error::LoadError,
    loader::Loader,
    node::{
        handle::{Children, ChildrenByName, NodeHandle},
        NodeId,
    },
};

mod macros;

mod error;
mod loader;
mod node;

/// FBX data tree.
#[derive(Debug, Clone, PartialEq)]
pub struct Tree {
    /// Tree data.
    arena: Arena<NodeData>,
    /// Node name interner.
    node_names: StringInterner<DefaultBackend<NodeNameSym>>,
    /// (Implicit) root node ID.
    root_id: NodeId,
}

impl Tree {
    /// Returns the root node.
    #[inline]
    #[must_use]
    pub fn root(&self) -> NodeHandle<'_> {
        NodeHandle::new(self, self.root_id)
    }

    /// Creates a new `Tree`.
    #[inline]
    #[must_use]
    fn new(
        arena: Arena<NodeData>,
        node_names: StringInterner<DefaultBackend<NodeNameSym>>,
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
    ///
    /// Panics if a node with the given node ID does not exist in the tree.
    #[must_use]
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
    #[must_use]
    pub(crate) fn resolve_node_name(&self, sym: NodeNameSym) -> &str {
        self.node_names
            .resolve(sym)
            .unwrap_or_else(|| panic!("Unresolvable node name symbol: {:?}", sym))
    }

    /// Returns node name symbol if available.
    #[must_use]
    pub(crate) fn node_name_sym(&self, name: &str) -> Option<NodeNameSym> {
        self.node_names.get(name)
    }

    /// Checks whether or not the given node ID is used in the tree.
    #[must_use]
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
        parent.raw().append(new_child, &mut self.arena);

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
        parent.raw().prepend(new_child, &mut self.arena);

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
        sibling.raw().insert_after(new_child, &mut self.arena);

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
        sibling.raw().insert_before(new_child, &mut self.arena);

        NodeId::new(new_child)
    }

    /// Creates a new node and inserts before the given sibling node.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    pub fn append_attribute(&mut self, node_id: NodeId, v: impl Into<AttributeValue>) {
        self.append_attribute_impl(node_id, v.into())
    }

    /// Internal implementation of `append_attribute`.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    fn append_attribute_impl(&mut self, node_id: NodeId, v: AttributeValue) {
        assert_ne!(node_id, self.root_id, "Root node should have no attributes");
        let node = self.arena.get_mut(node_id.raw()).expect("Invalid node ID");
        node.get_mut().append_attribute(v)
    }

    /// Returns a mutable reference to the node attribute at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    #[must_use]
    pub fn get_attribute_mut(&mut self, node_id: NodeId, i: usize) -> Option<&mut AttributeValue> {
        let node = self.arena.get_mut(node_id.raw()).expect("Invalid node ID");
        node.get_mut().get_attribute_mut(i)
    }

    /// Takes all attributes as a `Vec`.
    ///
    /// After calling this, the node will have no attributes (until other values are set).
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    pub fn take_attributes_vec(&mut self, node_id: NodeId) -> Vec<AttributeValue> {
        let node = self.arena.get_mut(node_id.raw()).expect("Invalid node ID");
        node.get_mut().replace_attributes(Default::default())
    }

    /// Sets the given `Vec` of attribute values as the node attributes.
    ///
    /// After calling this, the node will have only the given attributes.
    ///
    /// # Panics
    ///
    /// Panics if the given node ID is invalid (i.e. not used or root node).
    pub fn set_attributes_vec(&mut self, node_id: NodeId, new: Vec<AttributeValue>) {
        let node = self.arena.get_mut(node_id.raw()).expect("Invalid node ID");
        // Ignore the returned value.
        node.get_mut().replace_attributes(new);
    }

    /// Compares trees strictly.
    ///
    /// Returns `true` if the two trees are same.
    ///
    /// Note that `f32` and `f64` values are compared bitwise.
    ///
    /// Note that this method compares tree data, not internal states of the
    /// objects.
    #[inline]
    #[must_use]
    pub fn strict_eq(&self, other: &Self) -> bool {
        self.root().strict_eq(&other.root())
    }

    /// Pretty-print the tree for debugging purpose.
    ///
    /// Be careful, this output format may change in future.
    #[inline]
    #[must_use]
    pub fn debug_tree(&self) -> impl fmt::Debug + '_ {
        DebugTree { tree: self }
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

/// A simple wrapper for pretty-printing tree.
struct DebugTree<'a> {
    /// Tree.
    tree: &'a Tree,
}

impl fmt::Debug for DebugTree<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = DebugNodeHandle {
            node: self.tree.root(),
        };
        v.fmt(f)
    }
}

/// A simple wrapper for pretty-printing node.
struct DebugNodeHandle<'a> {
    /// Node.
    node: NodeHandle<'a>,
}

impl fmt::Debug for DebugNodeHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.node.name())
            .field("attributes", &self.node.attributes())
            .field("children", &DebugNodeHandleChildren { node: self.node })
            .finish()
    }
}

/// A simple wrapper for pretty-printing children.
struct DebugNodeHandleChildren<'a> {
    /// Parent node.
    node: NodeHandle<'a>,
}

impl fmt::Debug for DebugNodeHandleChildren<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(
                self.node
                    .children()
                    .map(|child| DebugNodeHandle { node: child }),
            )
            .finish()
    }
}

/// Event of depth-first traversal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DepthFirstTraversed {
    /// Opening of a node.
    Open(NodeId),
    /// Closing of a node.
    Close(NodeId),
}

impl DepthFirstTraversed {
    /// Returns the node ID.
    #[inline]
    #[must_use]
    pub fn node_id(self) -> NodeId {
        match self {
            Self::Open(id) => id,
            Self::Close(id) => id,
        }
    }

    /// Returns true if the event is node open.
    #[inline]
    #[must_use]
    pub fn is_open(self) -> bool {
        matches!(self, Self::Open(_))
    }

    /// Returns true if the event is node close.
    #[inline]
    #[must_use]
    pub fn is_close(self) -> bool {
        matches!(self, Self::Close(_))
    }

    /// Returns the opened node ID.
    #[inline]
    #[must_use]
    pub fn node_id_open(self) -> Option<NodeId> {
        match self {
            Self::Open(id) => Some(id),
            Self::Close(_) => None,
        }
    }

    /// Returns the closed node ID.
    #[inline]
    #[must_use]
    pub fn node_id_close(self) -> Option<NodeId> {
        match self {
            Self::Open(_) => None,
            Self::Close(id) => Some(id),
        }
    }

    /// Returns next (forward) event.
    ///
    /// Returns `None` for `Close(root_id)`.
    #[must_use]
    pub fn next(self, tree: &Tree) -> Option<Self> {
        let next = match self {
            Self::Open(current) => {
                // Dive into the first child if available, or otherwise leave the node.
                match current.to_handle(tree).first_child() {
                    Some(child) => Self::Open(child.node_id()),
                    None => Self::Close(current),
                }
            }
            Self::Close(current) => {
                // Dive into the next sibling if available, or leave the parent.
                let node = current.to_handle(tree);
                match node.next_sibling() {
                    Some(next_sib) => Self::Open(next_sib.node_id()),
                    None => Self::Close(node.parent()?.node_id()),
                }
            }
        };
        Some(next)
    }

    /// Returns previous (backward next) event.
    ///
    /// Note that this backward traversal returns `Clone` first, and `Open`
    /// later for every node.
    ///
    /// Returns `None` for `Open(root_id)`.
    #[must_use]
    pub fn prev(self, tree: &Tree) -> Option<Self> {
        let prev = match self {
            Self::Close(current) => {
                // Dive into the last child if available, or otherwise leave the node.
                match current.to_handle(tree).last_child() {
                    Some(child) => Self::Close(child.node_id()),
                    None => Self::Open(current),
                }
            }
            Self::Open(current) => {
                // Dive into the previous sibling if available, or leave the parent.
                let node = current.to_handle(tree);
                match node.previous_sibling() {
                    Some(prev_sib) => Self::Close(prev_sib.node_id()),
                    None => Self::Open(node.parent()?.node_id()),
                }
            }
        };
        Some(prev)
    }
}

/// A type to traverse a node and its descendants in depth-first order.
///
/// This type has two cursors, forward cursor and backward cursor.
/// In the initial state, forward cursor points to the opening of the root node,
/// and the backward cursor points to the ending of the root node.
///
/// Forward cursor advances forward, and backward cursor advances backward.
/// `next_forward` and `next_backward` methods returns the node ID the
/// corresponding cursor points to, and advances the cursor.
///
/// When the forward cursor goes after the backward cursor, then all events are
/// considered emitted.
#[derive(Debug, Clone, Copy)]
pub struct DepthFirstTraverseSubtree {
    /// Next (forward and backward) events to return.
    cursors: Option<(DepthFirstTraversed, DepthFirstTraversed)>,
}

impl DepthFirstTraverseSubtree {
    /// Creates a new object.
    #[inline]
    #[must_use]
    fn with_root_id(root: NodeId) -> Self {
        Self {
            cursors: Some((
                DepthFirstTraversed::Open(root),
                DepthFirstTraversed::Close(root),
            )),
        }
    }

    /// Returns the forward next traversal event and advances the forward cursor.
    #[inline]
    pub fn next_forward(&mut self, tree: &Tree) -> Option<DepthFirstTraversed> {
        let (forward, backward) = self.cursors?;
        if forward == backward {
            self.cursors = None;
        } else {
            let next_of_next = forward
                .next(tree)
                .expect("`forward` should point before `backward`");
            self.cursors = Some((next_of_next, backward));
        }
        Some(forward)
    }

    /// Returns the backward next traversal event and advances the backward cursor.
    #[inline]
    pub fn next_backward(&mut self, tree: &Tree) -> Option<DepthFirstTraversed> {
        let (forward, backward) = self.cursors?;
        if forward == backward {
            self.cursors = None;
        } else {
            let next_of_next = backward
                .prev(tree)
                .expect("`forward` should point before `backward`");
            self.cursors = Some((forward, next_of_next));
        }
        Some(backward)
    }

    /// Returns the forward next traversal event without advancing the cursor.
    #[inline]
    #[must_use]
    pub fn peek_forward(&self) -> Option<DepthFirstTraversed> {
        self.cursors.map(|(forward, _backward)| forward)
    }

    /// Returns the backward next traversal event without advancing the cursor.
    #[inline]
    #[must_use]
    pub fn peek_backward(&self) -> Option<DepthFirstTraversed> {
        self.cursors.map(|(_forward, backward)| backward)
    }

    /// Returns the forward next `Open` traversal event and advances the forward cursor.
    ///
    /// This makes it easy to forward-traverse the subtree in preorder.
    pub fn next_open_forward(&mut self, tree: &Tree) -> Option<NodeId> {
        loop {
            let next = self.next_forward(tree)?;
            if let DepthFirstTraversed::Open(id) = next {
                return Some(id);
            }
        }
    }

    /// Returns the forward next `Close` traversal event and advances the forward cursor.
    ///
    /// This makes it easy to forward-traverse the subtree in postorder.
    pub fn next_close_forward(&mut self, tree: &Tree) -> Option<NodeId> {
        loop {
            let next = self.next_forward(tree)?;
            if let DepthFirstTraversed::Close(id) = next {
                return Some(id);
            }
        }
    }

    /// Returns the backward next `Open` traversal event and advances the backward cursor.
    ///
    /// This makes it easy to backward-traverse the subtree in postorder.
    #[must_use]
    pub fn next_open_backward(&mut self, tree: &Tree) -> Option<NodeId> {
        loop {
            let next = self.next_backward(tree)?;
            if let DepthFirstTraversed::Open(id) = next {
                return Some(id);
            }
        }
    }

    /// Returns the backward next `Close` traversal event and advances the backward cursor.
    ///
    /// This makes it easy to backward-traverse the subtree in preorder.
    #[must_use]
    pub fn next_close_backward(&mut self, tree: &Tree) -> Option<NodeId> {
        loop {
            let next = self.next_backward(tree)?;
            if let DepthFirstTraversed::Close(id) = next {
                return Some(id);
            }
        }
    }
}
