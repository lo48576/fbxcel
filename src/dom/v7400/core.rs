//! FBX DOM core.

use indextree::Arena;
use string_interner::StringInterner;

use crate::dom::v7400::node::{IntoRawNodeId, Node, NodeData};
use crate::dom::v7400::{NodeId, StrSym};
use crate::dom::LoadError;
use crate::pull_parser::v7400::Parser;
use crate::pull_parser::ParserSource;

use self::loader::CoreLoader;

mod loader;

/// FBX DOM core.
#[derive(Debug, Clone, PartialEq)]
pub struct Core {
    /// FBX node names.
    strings: StringInterner<StrSym>,
    /// FBX nodes.
    nodes: Arena<NodeData>,
    /// (Implicit) root node.
    root: NodeId,
}

impl Core {
    /// Creates a new `Core`.
    pub(crate) fn new(
        strings: StringInterner<StrSym>,
        nodes: Arena<NodeData>,
        root: NodeId,
    ) -> Self {
        Self {
            strings,
            nodes,
            root,
        }
    }

    /// Loads the DOM core data from the given parser.
    pub fn load<R>(parser: &mut Parser<R>) -> Result<Self, LoadError>
    where
        R: ParserSource,
    {
        CoreLoader::new().load(parser)
    }

    /// Resolves the given interned string symbol into the corresponding string.
    ///
    /// Returns `None` if the given symbol is registered to the document.
    pub(crate) fn string(&self, sym: StrSym) -> Option<&str> {
        self.strings.resolve(sym)
    }

    /// Returns string symbol if available.
    pub(crate) fn sym_opt(&self, s: &str) -> Option<StrSym> {
        self.strings.get(s)
    }

    /// Returns the node from the node ID.
    ///
    /// # Panics
    ///
    /// Panics if the node with the given ID is not available.
    pub(crate) fn node(&self, id: impl IntoRawNodeId) -> Node<'_> {
        match self.nodes.get(id.raw_node_id()) {
            Some(v) => Node::new(v),
            None => panic!("Node should exist but not found: id={:?}", id),
        }
    }

    /// Returns the reference to the nodes arena.
    pub(crate) fn nodes(&self) -> &Arena<NodeData> {
        &self.nodes
    }

    /// Returns the immutable reference to the node and mutable reference to the
    /// string interner.
    pub(crate) fn node_and_strings(
        &mut self,
        id: impl IntoRawNodeId,
    ) -> (Node<'_>, &mut StringInterner<StrSym>) {
        let node = match self.nodes.get(id.raw_node_id()) {
            Some(v) => Node::new(v),
            None => panic!("Node should exist but not found: id={:?}", id),
        };
        (node, &mut self.strings)
    }

    /// Returns the root node ID.
    pub fn root(&self) -> NodeId {
        self.root
    }

    /// Finds a toplevel node by the name.
    pub(crate) fn find_toplevel(&self, target_name: &str) -> Option<NodeId> {
        self.children_by_name(self.root(), target_name).next()
    }

    /// Returns an iterator of childrens with the given name.
    pub(crate) fn children_by_name<'a>(
        &'a self,
        parent: NodeId,
        target_name: &str,
    ) -> impl Iterator<Item = NodeId> + 'a {
        // By using `flat_map` first, the iterator can return `None` before
        // traversing the tree if `target_name` is not registered.
        self.sym_opt(target_name)
            .into_iter()
            .flat_map(move |target_sym| {
                parent
                    .raw_node_id()
                    .children(&self.nodes())
                    .filter(move |&child_id| self.node(child_id).data().name_sym() == target_sym)
                    .map(NodeId::new)
            })
    }
}

impl AsRef<Core> for Core {
    fn as_ref(&self) -> &Self {
        self
    }
}
