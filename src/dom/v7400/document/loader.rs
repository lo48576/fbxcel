//! FBX DOM document loader.

use indextree::Arena;
use log::error;
use string_interner::StringInterner;

use crate::dom::v7400::{Document, NodeData, NodeId, StrSym};
use crate::dom::LoadError;
use crate::pull_parser::v7400::attribute::visitor::DirectVisitor;
use crate::pull_parser::v7400::attribute::DirectAttributeValue;
use crate::pull_parser::v7400::{Attributes, Event, Parser, StartNode};
use crate::pull_parser::ParserSource;

/// DOM document loader.
#[derive(Debug, Clone)]
pub struct Loader {
    /// FBX node names.
    strings: StringInterner<StrSym>,
    /// FBX nodes.
    nodes: Arena<NodeData>,
    /// (Implicit) root node.
    root: NodeId,
    /// Strict mode flag.
    ///
    /// If this is `true`, non-critical errors should be `Err`.
    /// If `false`, non-critical errors are ignored.
    strict: bool,
}

impl Loader {
    /// Creates a new `Loader`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the strict mode flag.
    pub fn strict(self, v: bool) -> Self {
        Self { strict: v, ..self }
    }

    /// Loads the DOM document from the parser.
    pub fn load_document<R>(mut self, parser: &mut Parser<R>) -> Result<Document, LoadError>
    where
        R: ParserSource,
    {
        // Load basic tree.
        self.load_tree(parser)?;

        Ok(Document::new(self.strings, self.nodes, self.root))
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
            let attributes = Self::load_attributes(&mut start.attributes())?;

            NodeId::new(self.nodes.new_node(NodeData::new(name, attributes)))
        };

        // Set the parent.
        parent.raw().append(current.raw(), &mut self.nodes).expect(
            "Should never fail: The newly created node should always be successfully appended",
        );

        Ok(current)
    }

    /// Loads node attributes.
    fn load_attributes<R>(
        attrs_parser: &mut Attributes<'_, R>,
    ) -> Result<Vec<DirectAttributeValue>, LoadError>
    where
        R: ParserSource,
    {
        let mut attributes = Vec::with_capacity(attrs_parser.total_count() as usize);
        // TODO: Should `visit_next_buffered` be used here?
        while let Some(attr) = attrs_parser.visit_next(DirectVisitor)? {
            attributes.push(attr);
        }

        Ok(attributes)
    }
}

impl Default for Loader {
    fn default() -> Self {
        let mut strings = StringInterner::new();
        let mut nodes = Arena::new();
        let root =
            NodeId::new(nodes.new_node(NodeData::new(strings.get_or_intern(""), Vec::new())));

        Self {
            strings,
            nodes,
            root,
            strict: false,
        }
    }
}
