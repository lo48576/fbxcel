//! FBX DOM core loader.

use indextree::Arena;
use log::error;
use string_interner::StringInterner;

use crate::dom::v7400::core::Core;
use crate::dom::v7400::{IntoRawNodeId, NodeData, NodeId, StrSym};
use crate::dom::LoadError;
use crate::pull_parser::v7400::attribute::visitor::DirectVisitor;
use crate::pull_parser::v7400::{Event, Parser, StartNode};
use crate::pull_parser::ParserSource;

/// DOM core loader.
#[derive(Debug, Clone)]
pub struct CoreLoader {
    /// FBX node names.
    strings: StringInterner<StrSym>,
    /// FBX nodes.
    nodes: Arena<NodeData>,
    /// (Implicit) root node.
    root: NodeId,
}

impl CoreLoader {
    /// Creates a new `CoreLoader`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads the DOM core from the parser.
    pub fn load<R>(mut self, parser: &mut Parser<R>) -> Result<Core, LoadError>
    where
        R: ParserSource,
    {
        // Load basic tree.
        self.load_tree(parser)?;

        Ok(Core::new(self.strings, self.nodes, self.root))
    }

    /// Loads simple tree data.
    fn load_tree<R>(&mut self, parser: &mut Parser<R>) -> Result<(), LoadError>
    where
        R: ParserSource,
    {
        if parser.current_depth() != 0 {
            error!("The given parser should be brand-new, but it has already emitted some events");
            return Err(LoadError::BadParser);
        }

        let mut open_nodes = vec![self.root];
        loop {
            assert!(
                !open_nodes.is_empty(),
                "Open nodes stack should not be empty on loop start"
            );

            match parser.next_event()? {
                Event::StartNode(start) => {
                    let parent = open_nodes
                        .last_mut()
                        .expect("Should never fail: Open nodes stack should not be empty here");
                    let current = self.add_node(*parent, start)?;

                    // Update the open nodes stack.
                    open_nodes.push(current);
                }
                Event::EndNode => {
                    open_nodes
                        .pop()
                        .expect("Should never fail: Open nodes stack should not be empty here");
                }
                Event::EndFbx(_) => {
                    open_nodes
                        .pop()
                        .expect("Should never fail: Open nodes stack should not be empty here");
                    break;
                }
            }
        }
        assert!(
            open_nodes.is_empty(),
            "Should never fail: There should be no open nodes after `EndFbx` event is emitted"
        );

        Ok(())
    }

    /// Creates and adds a new node to the tree.
    fn add_node<R>(&mut self, parent: NodeId, start: StartNode<'_, R>) -> Result<NodeId, LoadError>
    where
        R: ParserSource,
    {
        // Create a new node.
        let current = {
            let name = self.strings.get_or_intern(start.name());
            let attributes = start
                .attributes()
                .into_iter(std::iter::repeat(DirectVisitor))
                .collect::<Result<Vec<_>, _>>()?;

            NodeId::new(self.nodes.new_node(NodeData::new(name, attributes)))
        };

        // Set the parent.
        parent
            .raw_node_id()
            .append(current.raw_node_id(), &mut self.nodes)
            .expect(
                "Should never fail: The newly created node should always be successfully appended",
            );

        Ok(current)
    }
}

impl Default for CoreLoader {
    fn default() -> Self {
        let mut strings = StringInterner::new();
        let mut nodes = Arena::new();
        let root =
            NodeId::new(nodes.new_node(NodeData::new(strings.get_or_intern(""), Vec::new())));

        Self {
            strings,
            nodes,
            root,
        }
    }
}
