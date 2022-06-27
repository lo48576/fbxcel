//! FBX data tree loader.

use indextree::Arena;
use log::{debug, error, trace};
use string_interner::{DefaultBackend, StringInterner};

use crate::{
    low::v7400::FbxFooter,
    pull_parser::{
        v7400::{attribute::loaders::DirectLoader, Event, Parser, StartNode},
        Error as ParserError, ParserSource,
    },
    tree::v7400::{LoadError, NodeData, NodeId, NodeNameSym, Tree},
};

/// FBX data tree loader.
#[derive(Debug, Clone)]
pub struct Loader {
    /// Tree data.
    arena: Arena<NodeData>,
    /// Node name interner.
    node_names: StringInterner<DefaultBackend<NodeNameSym>>,
    /// (Implicit) root node ID.
    root_id: NodeId,
}

impl Loader {
    /// Creates a new `Loader`.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a tree from the given parser, and returns the tree and FBX footer.
    ///
    /// The given parser should be brand-new, i.e. it should not have emited any
    /// events.
    /// This can be checked by [`Parser::is_used()`].
    /// If the given parser is already used, [`LoadError::BadParser`] error will
    /// be returned.
    ///
    /// If the tree is successfully read but FBX footer is not,
    /// `Ok(tree, Err(parser_error))` is returned.
    pub fn load<R: ParserSource>(
        mut self,
        parser: &mut Parser<R>,
    ) -> Result<(Tree, Result<Box<FbxFooter>, ParserError>), LoadError> {
        debug!("Loading FBX data tree from a parser");

        if parser.is_used() {
            error!("The given parser should be brand-new, but it has already emitted some events");
            return Err(LoadError::BadParser);
        }

        let mut open_nodes = vec![self.root_id];
        let footer = loop {
            trace!("Loading next parser event: open_nodes={:?}", open_nodes);
            assert!(
                !open_nodes.is_empty(),
                "Open nodes stack should not be empty on loop start"
            );
            match parser.next_event()? {
                Event::StartNode(start) => {
                    trace!("Got `Event::StartNode(name={:?})`", start.name());
                    let parent = open_nodes
                        .last_mut()
                        .expect("Should never fail: Open nodes stack should not be empty here");
                    let current = self.add_node(*parent, start)?;

                    // Update the open nodes stack.
                    open_nodes.push(current);
                }
                Event::EndNode => {
                    trace!("Got `Event::EndNode`");
                    open_nodes
                        .pop()
                        .expect("Should never fail: Open nodes stack should not be empty here");
                }
                Event::EndFbx(footer) => {
                    trace!("Got `Event::EndFbx(_)`");
                    open_nodes
                        .pop()
                        .expect("Should never fail: Open nodes stack should not be empty here");
                    break footer;
                }
            }
        };
        assert!(
            open_nodes.is_empty(),
            "Should never fail: There should be no open nodes after `EndFbx` event is emitted"
        );

        debug!("Successfully loaded FBX data tree");
        let tree = Tree::new(self.arena, self.node_names, self.root_id);
        Ok((tree, footer))
    }

    /// Creates and adds a new node to the tree.
    fn add_node<R: ParserSource>(
        &mut self,
        parent: NodeId,
        start: StartNode<'_, R>,
    ) -> Result<NodeId, LoadError> {
        trace!(
            "Adding a new child name={:?} to the parent {:?}",
            start.name(),
            parent
        );

        // Create a new node.
        let current = {
            let name_sym = self.node_names.get_or_intern(start.name());
            let attributes = start
                .attributes()
                .into_iter(std::iter::repeat(DirectLoader))
                .collect::<Result<Vec<_>, _>>()?;

            NodeId::new(self.arena.new_node(NodeData::new(name_sym, attributes)))
        };

        // Set the parent.
        parent.raw().append(current.raw(), &mut self.arena);

        trace!(
            "Successfully added a new child {:?} to the parent {:?}",
            current,
            parent
        );

        Ok(current)
    }
}

impl Default for Loader {
    fn default() -> Self {
        let mut arena = Arena::new();
        let mut node_names = StringInterner::new();
        let root_id = {
            // Use empty string as dummy node name.
            let empty_sym = node_names.get_or_intern("");
            NodeId::new(arena.new_node(NodeData::new(empty_sym, Vec::new())))
        };
        Self {
            arena,
            node_names,
            root_id,
        }
    }
}
