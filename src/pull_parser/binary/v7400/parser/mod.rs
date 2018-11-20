//! Parser for BX 7.4 or later.

use std::io;

use super::super::reader::{PlainSource, SeekableSource};
use super::super::FbxHeader;
use super::error::{DataError, OperationError};
use super::{Event, FbxVersion, ParserSource, ParserSourceExt, ParserVersion, Result, StartNode};

use self::node::NodeHeader;

mod node;

/// Creates a new `Parser` from the given buffered reader.
///
/// Returns an error if the given FBX version in unsupported.
pub fn from_reader<R>(header: FbxHeader, reader: R) -> Result<Parser<PlainSource<R>>>
where
    R: io::Read,
{
    Parser::create(
        header.version(),
        PlainSource::with_offset(reader, header.len()),
    )
}

/// Creates a new `Parser` from the given seekable reader.
///
/// Returns an error if the given FBX version in unsupported.
pub fn from_seekable_reader<R>(header: FbxHeader, reader: R) -> Result<Parser<SeekableSource<R>>>
where
    R: io::Read + io::Seek,
{
    Parser::create(
        header.version(),
        SeekableSource::with_offset(reader, header.len()),
    )
}

/// Pull parser for FBX 7.4 binary or compatible later versions.
#[derive(Debug, Clone)]
pub struct Parser<R> {
    /// Parser state.
    state: State,
    /// Reader.
    reader: R,
}

impl<R: ParserSource> Parser<R> {
    /// Parser version.
    pub const PARSER_VERSION: ParserVersion = ParserVersion::V7400;

    /// Creates a new `Parser`.
    ///
    /// Returns an error if the given FBX version in unsupported.
    pub(crate) fn create(fbx_version: FbxVersion, reader: R) -> Result<Self> {
        if fbx_version.parser_version() != Some(Self::PARSER_VERSION) {
            return Err(
                OperationError::UnsupportedFbxVersion(Self::PARSER_VERSION, fbx_version).into(),
            );
        }

        Ok(Self {
            state: State::new(fbx_version),
            reader,
        })
    }

    /// Returns a mutable reference to the inner reader.
    pub(crate) fn reader(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Returns FBX version.
    pub fn fbx_version(&self) -> FbxVersion {
        self.state.fbx_version
    }

    /// Returns the name of the current node.
    ///
    /// # Panics
    ///
    /// This panics if there are no open nodes.
    pub fn current_node_name(&self) -> &str {
        self.state
            .current_node()
            .expect("Implicit top-level node has no name")
            .name
            .as_str()
    }

    /// Returns the number of attributes of the current node.
    pub(crate) fn current_attributes_count(&self) -> u64 {
        self.state
            .current_node()
            .expect("Implicit top-level node has no name")
            .attributes_count
    }

    /// Returns current node depth.
    ///
    /// Implicit root node is considered to be depth 0.
    pub fn current_depth(&self) -> usize {
        self.state.started_nodes.len()
    }

    /// Returns next event if successfully read.
    ///
    /// You should not call `next_event()` if a parser functionality has been
    /// already failed and returned error.
    /// If you call `next_event()` with failed parser, error created from
    /// [`OperationError::AlreadyAborted`] will be returned.
    pub fn next_event(&mut self) -> Result<Event<'_, R>> {
        let previous_depth = self.current_depth();

        // Precondition: Health should be `Health::Running`.
        match self.state.health() {
            Health::Running => {}
            Health::Finished => return Err(OperationError::AlreadyFinished.into()),
            Health::Aborted => return Err(OperationError::AlreadyAborted.into()),
        }

        // Update health.
        let event_kind = match self.next_event_impl() {
            Ok(v) => v,
            Err(e) => {
                self.state.health = Health::Aborted;
                return Err(e);
            }
        };
        if event_kind == EventKind::EndFbx {
            self.state.health = Health::Finished;
        }

        // Postcondition: Depth should be updated correctly.
        let current_depth = self.current_depth();
        match event_kind {
            EventKind::StartNode => {
                assert_eq!(
                    current_depth.wrapping_sub(previous_depth),
                    1,
                    "The depth should be incremented on `StartNode`"
                );
            }
            EventKind::EndNode => {
                assert_eq!(
                    previous_depth.wrapping_sub(current_depth),
                    1,
                    "The depth should be decremented on `EndNode`"
                );
            }
            EventKind::EndFbx => {
                assert_eq!(
                    previous_depth, 0,
                    "Depth should be 0 before parsing finishes"
                );
                assert_eq!(
                    current_depth, 0,
                    "Depth should be 0 after parsing is finished"
                );
            }
        }

        // Create the real result.
        Ok(match event_kind {
            EventKind::StartNode => Event::StartNode(StartNode::new(self)),
            EventKind::EndNode => Event::EndNode,
            EventKind::EndFbx => Event::EndFbx,
        })
    }

    /// Reads the next node header and changes the parser state (except for
    /// parser health).
    fn next_event_impl(&mut self) -> Result<EventKind> {
        assert_eq!(self.state.health(), Health::Running);

        // Skip unread attribute of previous node, if exists.
        self.skip_unread_attributes()?;

        let event_start_offset = self.reader().position();

        // Check if the current node ends here (without any marker).
        if self.state.current_node().map(|v| v.node_end_offset) == Some(event_start_offset) {
            // The current node implicitly ends here.
            self.state.started_nodes.pop();
            return Ok(EventKind::EndNode);
        }

        // Read node header.
        let node_header = NodeHeader::read_from_parser(self)?;

        let header_end_offset = self.reader().position();

        // Check if a node or a document ends here (with explicit marker).
        if node_header.is_node_end() {
            // The current node explicitly ends here.
            return match self.state.started_nodes.pop() {
                Some(closing) => {
                    if closing.node_end_offset != header_end_offset {
                        return Err(DataError::NodeLengthMismatch(
                            closing.node_end_offset,
                            header_end_offset,
                        )
                        .into());
                    }
                    Ok(EventKind::EndNode)
                }
                None => Ok(EventKind::EndFbx),
            };
        }

        // Read the node name.
        let name = {
            let mut vec = vec![0; node_header.bytelen_name as usize];
            self.reader.read_exact(&mut vec[..])?;
            String::from_utf8(vec).map_err(DataError::InvalidNodeNameEncoding)?
        };
        let current_offset = self.reader().position();
        let starting = StartedNode {
            node_start_offset: event_start_offset,
            node_end_offset: node_header.end_offset,
            attributes_count: node_header.num_attributes,
            attributes_end_offset: current_offset + node_header.bytelen_attributes,
            name,
        };

        // Update parser status.
        self.state.started_nodes.push(starting);
        Ok(EventKind::StartNode)
    }

    /// Skip unread attribute of the current node, if remains.
    fn skip_unread_attributes(&mut self) -> Result<()> {
        let attributes_end_offset = match self.state.current_node() {
            Some(v) => v.attributes_end_offset,
            None => return Ok(()),
        };
        if attributes_end_offset > self.reader().position() {
            // Skip if attributes remains (partially or entirely) unread.
            self.reader().skip_to(attributes_end_offset)?;
        }

        Ok(())
    }
}

/// Health of a parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Health {
    /// Ready or already started, but not yet finished, and no critical errors.
    Running,
    /// Successfully finished.
    Finished,
    /// Aborted due to critical error.
    Aborted,
}

/// Parser state.
///
/// This type contains parser state especially which are independent of parser
/// source type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    /// Target FBX version.
    fbx_version: FbxVersion,
    /// Health of the parser.
    health: Health,
    /// Started nodes stack.
    ///
    /// This stack should not have an entry for implicit root node.
    started_nodes: Vec<StartedNode>,
}

impl State {
    /// Creates a new `State` for the given FBX version.
    fn new(fbx_version: FbxVersion) -> Self {
        Self {
            fbx_version,
            health: Health::Running,
            started_nodes: Vec::new(),
        }
    }

    /// Returns health of the parser.
    fn health(&self) -> Health {
        self.health
    }

    /// Returns info about current node (except for implicit root node).
    fn current_node(&self) -> Option<&StartedNode> {
        self.started_nodes.last()
    }
}

/// Event kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum EventKind {
    /// Node start.
    StartNode,
    /// Node end.
    EndNode,
    /// FBX document end.
    EndFbx,
}

/// Information about started node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct StartedNode {
    /// Start offset of the node attribute.
    node_start_offset: u64,
    /// End offset of the node.
    ///
    /// "End offset" means a next byte of the last byte of the last node.
    node_end_offset: u64,
    /// Number of node attributes.
    attributes_count: u64,
    /// End offset of the previous attribute.
    ///
    /// "End offset" means a next byte of the last byte of the last attribute.
    attributes_end_offset: u64,
    /// Node name.
    name: String,
}
