//! Parser for FBX 7.4 or later.

use std::{fmt, io};

use crate::{
    low::{
        v7400::{FbxFooter, NodeHeader},
        FbxHeader, FbxVersion,
    },
    pull_parser::{
        error::{DataError, OperationError},
        reader::{PlainSource, SeekableSource},
        v7400::{Event, FromParser, StartNode},
        Error, ParserSource, ParserVersion, Result, SyntacticPosition, Warning,
    },
};

/// Warning handler type.
type WarningHandler = Box<dyn FnMut(Warning, &SyntacticPosition) -> Result<()>>;

/// Creates a new [`Parser`] from the given reader.
///
/// Returns an error if the given FBX version in unsupported.
#[inline]
pub fn from_reader<R>(header: FbxHeader, reader: R) -> Result<Parser<PlainSource<R>>>
where
    R: io::Read,
{
    Parser::create(
        header.version(),
        PlainSource::with_offset(reader, header.len()),
    )
}

/// Creates a new [`Parser`] from the given seekable reader.
///
/// Returns an error if the given FBX version in unsupported.
#[inline]
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
pub struct Parser<R> {
    /// Parser state.
    state: State,
    /// Reader.
    reader: R,
    /// Warning handler.
    warning_handler: Option<WarningHandler>,
}

impl<R: ParserSource> Parser<R> {
    /// Parser version.
    pub const PARSER_VERSION: ParserVersion = ParserVersion::V7400;

    /// Creates a new `Parser`.
    ///
    /// Returns an error if the given FBX version in unsupported.
    pub(crate) fn create(fbx_version: FbxVersion, reader: R) -> Result<Self> {
        if ParserVersion::from_fbx_version(fbx_version) != Some(Self::PARSER_VERSION) {
            return Err(
                OperationError::UnsupportedFbxVersion(Self::PARSER_VERSION, fbx_version).into(),
            );
        }

        Ok(Self {
            state: State::new(fbx_version),
            reader,
            warning_handler: None,
        })
    }

    /// Sets the warning handler.
    ///
    /// The warning handler will receive warnings and their [syntactic
    /// position]s each time the warnings happen.
    ///
    /// If the handler returned `Ok(())`, the warning is considered non-critical
    /// and parsing can be continued.
    /// If the handler returned `Err(_)`, the warning is considered critical,
    /// and the parsing cannot be continued.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fbxcel::low::FbxHeader;
    /// # let reader = std::io::empty();
    /// # let header: FbxHeader = unimplemented!();
    /// let mut parser = fbxcel::pull_parser::v7400::from_reader(header, reader)
    ///     .expect("Failed to create parser");
    /// parser.set_warning_handler(|warning, pos| {
    ///     // Print warning.
    ///     eprintln!("WARNING: {} (pos={:?})", warning, pos);
    ///     // To ignore the warning and continue processing, return `Ok(())`.
    ///     // To treat the given warning as a critical error, return
    ///     // `Err(warning.into())`.
    ///     Ok(())
    /// });
    /// ```
    ///
    /// [syntactic position]: `SyntacticPosition`
    #[inline]
    pub fn set_warning_handler<F>(&mut self, warning_handler: F)
    where
        F: 'static + FnMut(Warning, &SyntacticPosition) -> Result<()>,
    {
        self.warning_handler = Some(Box::new(warning_handler));
    }

    /// Returns a mutable reference to the inner reader.
    #[inline]
    #[must_use]
    pub(crate) fn reader(&mut self) -> &mut R {
        &mut self.reader
    }

    /// Returns FBX version.
    #[inline]
    #[must_use]
    pub fn fbx_version(&self) -> FbxVersion {
        self.state.fbx_version
    }

    /// Returns the name of the current node.
    ///
    /// # Panics
    ///
    /// This panics if there are no open nodes.
    #[inline]
    #[must_use]
    pub fn current_node_name(&self) -> &str {
        self.state
            .current_node()
            .expect("Implicit top-level node has no name")
            .name
            .as_str()
    }

    /// Returns the number of attributes of the current node.
    #[inline]
    #[must_use]
    pub(crate) fn current_attributes_count(&self) -> u64 {
        self.state
            .current_node()
            .expect("Implicit top-level node has no name")
            .attributes_count
    }

    /// Returns current node depth.
    ///
    /// Implicit root node is considered to be depth 0.
    #[inline]
    #[must_use]
    pub fn current_depth(&self) -> usize {
        self.state.started_nodes.len()
    }

    /// Returns `true` if the parser can continue parsing, `false` otherwise.
    pub(crate) fn ensure_continuable(&self) -> Result<()> {
        match self.state.health() {
            Health::Running => Ok(()),
            Health::Finished => Err(OperationError::AlreadyFinished.into()),
            Health::Aborted(err_pos) => Err(Error::with_position(
                OperationError::AlreadyAborted.into(),
                err_pos.clone(),
            )),
        }
    }

    /// Reads the given type from the underlying reader.
    #[inline]
    pub(crate) fn parse<T: FromParser>(&mut self) -> Result<T> {
        T::read_from_parser(self)
    }

    /// Passes the given warning to the warning handler.
    pub(crate) fn warn(&mut self, warning: Warning, pos: SyntacticPosition) -> Result<()> {
        match self.warning_handler {
            Some(ref mut handler) => match handler(warning, &pos) {
                Ok(()) => Ok(()),
                Err(e) => Err(e.and_position(pos)),
            },
            None => Ok(()),
        }
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
        self.ensure_continuable()?;

        // Update health.
        let event_kind = match self.next_event_impl() {
            Ok(v) => v,
            Err(e) => {
                let err_pos = self.position();
                self.set_aborted(err_pos.clone());
                return Err(e.and_position(err_pos));
            }
        };
        if event_kind == EventKind::EndFbx {
            self.state.health = Health::Finished;
        }

        // Update the last event kind.
        self.state.last_event_kind = Some(event_kind);

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

        // Postcondition: The last event kind should be memorized correctly.
        assert_eq!(
            self.state.last_event_kind(),
            Some(event_kind),
            "The last event kind should be memorized correctly"
        );

        // Create the real result.
        Ok(match event_kind {
            EventKind::StartNode => Event::StartNode(StartNode::new(self)),
            EventKind::EndNode => Event::EndNode,
            EventKind::EndFbx => {
                let footer_res = FbxFooter::read_from_parser(self).map(Box::new);
                Event::EndFbx(footer_res)
            }
        })
    }

    /// Reads the next node header and changes the parser state (except for
    /// parser health and the last event kind).
    fn next_event_impl(&mut self) -> Result<EventKind> {
        assert_eq!(self.state.health(), &Health::Running);
        assert_ne!(self.state.last_event_kind(), Some(EventKind::EndFbx));

        // Skip unread attribute of previous node, if exists.
        self.skip_unread_attributes()?;

        let event_start_offset = self.reader().position();

        // Check if the current node ends here (without any marker).
        // A node end marker (all-zero node header, which indicates end of the
        // current node) is omitted if and only if:
        //
        // * the node has no children nodes, and
        // * the node has one or more attributes.
        //
        // Note that the check can be skipped for the implicit root node,
        // It has always a node end marker at the ending (because it has no
        // attributes).
        if let Some(current_node) = self.state.current_node() {
            if current_node.node_end_offset < event_start_offset {
                // The current node has already been ended.
                return Err(
                    DataError::NodeLengthMismatch(current_node.node_end_offset, None).into(),
                );
            }
            if current_node.node_end_offset == event_start_offset {
                // `last_event_kind() == Some(EventKind::EndNode)` means that
                // some node ends right before the event currently reading.
                let has_children = self.state.last_event_kind() == Some(EventKind::EndNode);
                let has_attributes = current_node.attributes_count != 0;

                if has_children || !has_attributes {
                    // It's odd, the current node should have a node end marker
                    // at the ending, but `node_end_offset` data tells that the
                    // node ends without node end marker.
                    self.warn(Warning::MissingNodeEndMarker, self.position())?;
                }
                self.state.started_nodes.pop();
                return Ok(EventKind::EndNode);
            }
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
                            Some(header_end_offset),
                        )
                        .into());
                    }
                    if closing.attributes_count != 0 && closing.known_children_count == 0 {
                        // It's odd, the current node should not have a node end
                        // marker at the ending, but found.
                        self.warn(Warning::ExtraNodeEndMarker, self.position())?;
                    }
                    Ok(EventKind::EndNode)
                }
                None => Ok(EventKind::EndFbx),
            };
        }

        if node_header.bytelen_name == 0 {
            let mut pos = self.position();
            // Need to modify position, because the currently reading node is
            // not reflected to the parser.
            pos.byte_pos = self.reader().position();
            pos.component_byte_pos = event_start_offset;
            let local_node_index = self
                .state
                .current_node()
                .map_or(self.state.known_toplevel_nodes_count, |v| {
                    v.known_children_count
                });
            pos.node_path.push((local_node_index, String::new()));

            self.warn(Warning::EmptyNodeName, pos)?;
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
            known_children_count: 0,
        };

        // Update parser status.
        match self.state.started_nodes.last_mut() {
            Some(parent) => parent.known_children_count += 1,
            None => self.state.known_toplevel_nodes_count += 1,
        }
        self.state.started_nodes.push(starting);
        Ok(EventKind::StartNode)
    }

    /// Skips unread attribute of the current node, if remains.
    ///
    /// If there are no unread attributes, this method simply do nothing.
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

    /// Sets the parser to aborted state.
    #[inline]
    pub(crate) fn set_aborted(&mut self, pos: SyntacticPosition) {
        self.state.health = Health::Aborted(pos);
    }

    /// Ignores events until the current node closes.
    ///
    /// This discards parser events until the [`EndNode`] event for the current
    /// node is read.
    /// The last [`EndNode`] (for the current node) is also discarded.
    ///
    /// This method seeks to the node end position without any additional
    /// parsing, since the parser already knows the node end position.
    /// Because of this, some errors can be overlooked, or detected at the
    /// different position from the true error position.
    ///
    /// To detect errors correctly, you should use [`next_event`][`Self::next_event`] manually.
    /// See an example to how to do this.
    ///
    /// # Panics
    ///
    /// Panics if there are no open nodes, i.e. when [`current_depth()`][`Self::current_depth`]
    /// returns 0.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fbxcel::low::FbxHeader;
    /// # let reader = std::io::empty();
    /// # let header: FbxHeader = unimplemented!();
    /// let mut parser = fbxcel::pull_parser::v7400::from_reader(header, reader)
    ///     .expect("Failed to create parser");
    /// // Do something here.
    /// // Something done.
    /// let depth = parser.current_depth();
    /// if depth > 0 {
    ///     parser.skip_current_node().expect("Failed to skip current node");
    ///     assert_eq!(parser.current_depth(), depth - 1);
    /// }
    /// ```
    ///
    /// `parser.skip_current_node()` is almost same as the code below, except
    /// for error handling.
    ///
    /// ```no_run
    /// # use fbxcel::pull_parser::{v7400::{Parser, Event}, ParserSource, Result};
    /// fn skip_current_node<R: ParserSource>(parser: &mut Parser<R>) -> Result<()> {
    ///     loop {
    ///         match parser.next_event()? {
    ///             Event::StartNode(_) => skip_current_node(parser)?,
    ///             Event::EndNode => return Ok(()),
    ///             Event::EndFbx(_) => panic!("Attempt to skip implicit top-level node"),
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// [`EndNode`]: `Event::EndNode`
    pub fn skip_current_node(&mut self) -> Result<()> {
        let end_pos = self
            .state
            .started_nodes
            .pop()
            .expect("Attempt to skip implicit top-level node")
            .node_end_offset;
        self.state.last_event_kind = Some(EventKind::EndNode);
        self.reader.skip_to(end_pos)?;

        Ok(())
    }
    /// Returns the syntactic position of the current node.
    ///
    /// Note that this allocates memory.
    #[inline]
    pub fn position(&self) -> SyntacticPosition {
        let byte_pos = self.reader.position();
        if self.state.current_node().is_none() {
            // Reading implicit root node.
            return SyntacticPosition {
                byte_pos,
                component_byte_pos: 0,
                node_path: Vec::new(),
                attribute_index: None,
            };
        }

        let toplevel_index = self
            .state
            .known_toplevel_nodes_count
            .checked_sub(1)
            .expect("Should never fail: implicit root node should have some children here");
        // For now, use 0 for start offset of implicit root node.
        // This behaviour may change in future.
        let node_start_pos = self.state.current_node().map_or(0, |v| v.node_start_offset);
        // Use not `checked_sub` but `saturating_sub` here, because
        // `Iterator::zip` might read extra elements which can be used as
        // result.
        let trailing_indices = self
            .state
            .started_nodes
            .iter()
            .map(|v| v.known_children_count.saturating_sub(1));
        let node_indices = std::iter::once(toplevel_index).chain(trailing_indices);
        let node_names = self.state.started_nodes.iter().map(|v| v.name.clone());
        let node_path = node_indices.zip(node_names).collect();

        SyntacticPosition {
            byte_pos,
            component_byte_pos: node_start_pos,
            node_path,
            attribute_index: None,
        }
    }

    /// Returns whether the parser is already used or brand-new.
    ///
    /// Returns `true` if the parser emitted some events in the past, returns
    /// `false` if the parser have not emitted any events.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use fbxcel::low::FbxHeader;
    /// # let reader = std::io::empty();
    /// # let header: FbxHeader = unimplemented!();
    /// let mut parser = fbxcel::pull_parser::v7400::from_reader(header, reader)
    ///     .expect("Failed to create parser");
    /// assert!(!parser.is_used());
    /// parser.set_warning_handler(|warning, pos| {
    ///     // Print warning.
    ///     eprintln!("WARNING: {} (pos={:?})", warning, pos);
    ///     // To ignore the warning and continue processing, return `Ok(())`.
    ///     // To treat the given warning as a critical error, return
    ///     // `Err(warning.into())`.
    ///     Ok(())
    /// });
    /// assert!(!parser.is_used(), "Parser emitted no events yet");
    /// let _ = parser.next_event();
    /// assert!(parser.is_used(), "Parser emitted an event");
    /// ```
    #[inline]
    #[must_use]
    pub fn is_used(&self) -> bool {
        self.state.last_event_kind.is_some()
    }
}

impl<R: fmt::Debug> fmt::Debug for Parser<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parser")
            .field("state", &self.state)
            .field("reader", &self.reader)
            .field(
                "warning_handler",
                &self.warning_handler.as_ref().map(|v| v as *const _),
            )
            .finish()
    }
}

/// Health of a parser.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Health {
    /// Ready or already started, but not yet finished, and no critical errors.
    Running,
    /// Successfully finished.
    Finished,
    /// Aborted due to critical error.
    Aborted(SyntacticPosition),
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
    /// Last event kind.
    last_event_kind: Option<EventKind>,
    /// Number of known top-level nodes.
    ///
    /// This is here because [`StartedNode`] is not used for implicit root node.
    known_toplevel_nodes_count: usize,
}

impl State {
    /// Creates a new `State` for the given FBX version.
    #[inline]
    #[must_use]
    fn new(fbx_version: FbxVersion) -> Self {
        Self {
            fbx_version,
            health: Health::Running,
            started_nodes: Vec::new(),
            last_event_kind: None,
            known_toplevel_nodes_count: 0,
        }
    }

    /// Returns health of the parser.
    #[inline]
    #[must_use]
    fn health(&self) -> &Health {
        &self.health
    }

    /// Returns info about current node (except for implicit root node).
    #[inline]
    #[must_use]
    fn current_node(&self) -> Option<&StartedNode> {
        self.started_nodes.last()
    }

    /// Returns the last event kind.
    #[inline]
    #[must_use]
    fn last_event_kind(&self) -> Option<EventKind> {
        self.last_event_kind
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
    /// Number of known children.
    known_children_count: usize,
}
