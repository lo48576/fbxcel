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
}
