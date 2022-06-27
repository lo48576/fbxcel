//! Node handle.

use std::fmt;

use crate::{
    low::v7400::AttributeValue,
    tree::v7400::{NodeData, NodeId, NodeNameSym, Tree},
};

/// Node handle.
#[derive(Debug, Clone, Copy)]
pub struct NodeHandle<'a> {
    /// The tree the node belongs to.
    tree: &'a Tree,
    /// Node ID.
    node_id: NodeId,
}

impl<'a> NodeHandle<'a> {
    /// Creates a new `NodeHandle`.
    ///
    /// # Panics and safety
    ///
    /// This may panic if the given node ID is not used in the given tree.
    ///
    /// Even if `new()` does not panic, subsequent operations through
    /// `NodeHandle` object may panic if the given node ID is not used in the
    /// given tree.
    #[inline]
    #[must_use]
    pub(crate) fn new(tree: &'a Tree, node_id: NodeId) -> Self {
        assert!(
            tree.contains_node(node_id),
            "The node ID is not used in the given tree: node_id={:?}",
            node_id
        );

        Self { tree, node_id }
    }

    /// Returns a reference to the tree.
    #[inline]
    #[must_use]
    pub fn tree(&self) -> &'a Tree {
        self.tree
    }

    /// Returns the node ID.
    #[inline]
    #[must_use]
    pub fn node_id(&self) -> NodeId {
        self.node_id
    }

    /// Returns the internally managed node data.
    #[inline]
    #[must_use]
    pub(crate) fn node(&self) -> &'a indextree::Node<NodeData> {
        self.tree.node(self.node_id)
    }

    /// Returns the node name symbol.
    #[inline]
    #[must_use]
    pub(crate) fn name_sym(&self) -> NodeNameSym {
        self.node().get().name_sym()
    }

    /// Returns the node name.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &'a str {
        self.tree.resolve_node_name(self.name_sym())
    }

    /// Returns the node attributes.
    #[inline]
    #[must_use]
    pub fn attributes(&self) -> &'a [AttributeValue] {
        self.node().get().attributes()
    }

    /// Returns an iterator of children with the given name.
    #[inline]
    #[must_use]
    pub fn children(&self) -> Children<'a> {
        Children {
            tree: self.tree,
            iter: self.node_id.raw().children(&self.tree.arena),
        }
    }

    /// Returns an iterator of children with the given name.
    #[inline]
    #[must_use]
    pub fn children_by_name(&self, name: &str) -> ChildrenByName<'a> {
        ChildrenByName {
            name_sym: self.tree.node_name_sym(name),
            children_iter: self.children(),
        }
    }

    /// Returns the first child with the given name.
    #[inline]
    #[must_use]
    pub fn first_child_by_name(&self, name: &str) -> Option<Self> {
        self.children_by_name(name).next()
    }

    /// Compares nodes strictly.
    ///
    /// Returns `true` if the two trees are same.
    ///
    /// Note that `f32` and `f64` values are compared bitwise.
    ///
    /// Note that this method compares tree data, not internal states of the
    /// trees.
    #[inline]
    #[must_use]
    pub fn strict_eq(&self, other: &Self) -> bool {
        nodes_strict_eq(*self, *other)
    }
}

/// Implement accessors to neighbor nodes.
macro_rules! impl_related_node_accessor {
    (
        $(
            $(#[$meta:meta])*
            $accessor:ident;
        )*
    ) => {
        impl<'a> NodeHandle<'a> {
            $(
                impl_related_node_accessor! { @single, $(#[$meta])* $accessor; }
            )*
        }
    };
    (@single, $(#[$meta:meta])* $accessor:ident;) => {
        $(#[$meta])*
        #[must_use]
        pub fn $accessor(&self) -> Option<NodeHandle<'a>> {
            self.node()
                .$accessor()
                .map(|id| NodeId::new(id).to_handle(&self.tree))
        }
    };
}

impl_related_node_accessor! {
    /// Returns parent node handle if available.
    parent;
    /// Returns first child node handle if available.
    first_child;
    /// Returns last child node handle if available.
    last_child;
    /// Returns previous sibling node handle if available.
    previous_sibling;
    /// Returns next sibling node handle if available.
    next_sibling;
}

/// Compares nodes strictly.
#[must_use]
fn nodes_strict_eq(left: NodeHandle<'_>, right: NodeHandle<'_>) -> bool {
    // Compare name.
    if left.name() != right.name() {
        return false;
    }
    // Compare attributes.
    {
        let left = left.attributes();
        let right = right.attributes();
        if left.len() != right.len() {
            return false;
        }
        if !left.iter().zip(right).all(|(l, r)| l.strict_eq(r)) {
            return false;
        }
    }
    // Compare children.
    {
        let mut left = left.children();
        let mut right = right.children();
        loop {
            match (left.next(), right.next()) {
                (Some(l), Some(r)) => {
                    if !nodes_strict_eq(l, r) {
                        return false;
                    }
                }
                (None, None) => break,
                _ => return false,
            }
        }
    }
    true
}

/// An iterator of children of a node.
#[derive(Clone)]
pub struct Children<'a> {
    /// Tree.
    tree: &'a Tree,
    /// Raw node children iterator.
    iter: indextree::Children<'a, NodeData>,
}

impl<'a> Iterator for Children<'a> {
    type Item = NodeHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let child_id = self.iter.next()?;
        Some(NodeId::new(child_id).to_handle(self.tree))
    }
}

impl std::iter::FusedIterator for Children<'_> {}

impl<'a> fmt::Debug for Children<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Children").finish()
    }
}

/// An iterator of children of a node, with a specific name.
#[derive(Clone)]
pub struct ChildrenByName<'a> {
    /// Name symbol.
    name_sym: Option<NodeNameSym>,
    /// Children node iterator.
    children_iter: Children<'a>,
}

impl<'a> Iterator for ChildrenByName<'a> {
    type Item = NodeHandle<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let name_sym = self.name_sym?;
        self.children_iter
            .find(|child| child.name_sym() == name_sym)
    }
}

impl std::iter::FusedIterator for ChildrenByName<'_> {}

impl<'a> fmt::Debug for ChildrenByName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ChildrenByName")
            .field("name_sym", &self.name_sym)
            .finish()
    }
}
