//! Parser event.

use crate::low::v7400::FbxFooter;

use super::{Attributes, Parser, ParserSource, Result};

/// Parser event.
#[derive(Debug)]
pub enum Event<'a, R: 'a> {
    /// Start of a node.
    StartNode(StartNode<'a, R>),
    /// End of a node.
    EndNode,
    /// End of an FBX document.
    EndFbx(Result<Box<FbxFooter>>),
}

/// Node start event.
#[derive(Debug)]
pub struct StartNode<'a, R> {
    /// Parser (used as a token).
    parser: &'a mut Parser<R>,
}

impl<'a, R: 'a + ParserSource> StartNode<'a, R> {
    /// Creates a new `StartNode`.
    pub(crate) fn new(parser: &'a mut Parser<R>) -> Self {
        Self { parser }
    }

    /// Returns the node name.
    pub fn name(&self) -> &str {
        self.parser.current_node_name()
    }

    /// Returns node attributes reader.
    pub fn attributes(self) -> Attributes<'a, R> {
        Attributes::from_parser(self.parser)
    }
}
