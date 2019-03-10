//! FBX DOM core.

use std::fmt;

use indextree::Arena;
use log::trace;
use string_interner::StringInterner;

use crate::dom::v7400::error::CoreLoadError;
use crate::dom::v7400::node::{IntoRawNodeId, Node, NodeData};
use crate::dom::v7400::{NodeId, StrSym};
use crate::pull_parser::v7400::Parser;
use crate::pull_parser::ParserSource;

use self::loader::CoreLoader;

mod loader;

/// FBX DOM core.
///
/// This manages basic tree structure and interned string tables.
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
    pub fn load<R>(parser: &mut Parser<R>) -> Result<Self, CoreLoadError>
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

    /// Interns the given string and returns the corresponding symbol.
    pub(crate) fn sym(&mut self, s: &str) -> StrSym {
        self.strings.get_or_intern(s)
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
        trace!("Looking for toplevel node with name {:?}", target_name);

        let result = self.children_by_name(self.root(), target_name).next();
        match result.as_ref() {
            Some(id) => trace!("Found toplevel node {:?}: id={:?}", target_name, id),
            None => trace!("Toplevel node {:?} not found", target_name),
        }

        result
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

    /// Returns the path of the node.
    pub(crate) fn path(&self, target: impl Into<NodeId>) -> NodePath<'_> {
        self.path_impl(target.into())
    }

    /// Internal implementation of `path()`.
    fn path_impl(&self, target: NodeId) -> NodePath<'_> {
        let mut revpath = Vec::new();
        let mut current = target;
        let root = self.root();
        while current != root {
            let node = self.node(current);
            let name_sym = node.data().name_sym();
            let (name_order, all_order) = current
                .raw_node_id()
                .preceding_siblings(&self.nodes)
                .skip(1)
                .fold((0, 0), |(mut name_order, all_order), sib| {
                    if self.node(sib).data().name_sym() == name_sym {
                        name_order += 1;
                    }
                    (name_order, all_order + 1)
                });
            revpath.push(PathComponent {
                name: node.name(self),
                name_order,
                all_order,
            });
            current = current
                .node(self)
                .parent()
                .expect("Should never fail: `current` must not be root node here");
        }
        revpath.reverse();
        NodePath {
            components: revpath,
        }
    }
}

impl AsRef<Core> for Core {
    fn as_ref(&self) -> &Self {
        self
    }
}

/// FBX node path.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct NodePath<'a> {
    /// Path components.
    components: Vec<PathComponent<'a>>,
}

impl NodePath<'_> {
    /// Returns an object that implements `Display` for printing `NodePath` with
    /// debug format.
    pub(crate) fn debug_display(&self) -> NodePathDebugDisplay<'_, '_> {
        NodePathDebugDisplay(self)
    }
}

/// FBX node path component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct PathComponent<'a> {
    /// Node name.
    name: &'a str,
    /// Index of the node among the nodes with the same name.
    name_order: usize,
    /// Index of the node among the all siblings.
    all_order: usize,
}

/// Adapter to use debug format for `Display`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NodePathDebugDisplay<'a, 'b>(&'a NodePath<'b>);

impl fmt::Display for NodePathDebugDisplay<'_, '_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
