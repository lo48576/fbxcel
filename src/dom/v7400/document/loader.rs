//! FBX DOM document loader.

use std::collections::HashMap;

use indextree::Arena;
use log::{error, warn};
use string_interner::StringInterner;

use crate::dom::v7400::object::{ObjectId, ObjectMeta, ObjectNodeId};
use crate::dom::v7400::{Document, NodeData, NodeId, StrSym};
use crate::dom::{AccessError, LoadError};
use crate::pull_parser::v7400::attribute::visitor::DirectVisitor;
use crate::pull_parser::v7400::{Event, Parser, StartNode};
use crate::pull_parser::ParserSource;

macro_rules! bail_if_strict {
    ($is_strict:expr, $err:expr, $loose:expr) => {
        if $is_strict {
            return Err($err.into());
        } else {
            $loose
        }
    };
    ($is_strict:expr, $err:expr) => {
        if $is_strict {
            return Err($err.into());
        }
    };
}

macro_rules! warn_noncritical {
    ($strict:expr, $format:expr) => {
        warn!(concat!("Noncritical DOM load error [strict={}] ", $format), $strict)
    };
    ($strict:expr, $format:expr, $($args:tt)*) => {
        warn!(
            concat!("Noncritical DOM load error [strict={}] ", $format),
            $strict, $($args)*
        )
    };
}

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
    /// Map from object ID to node ID.
    object_ids: HashMap<ObjectId, ObjectNodeId>,
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

        // Load objects.
        self.load_objects()?;

        Ok(Document::new(
            self.strings,
            self.nodes,
            self.root,
            self.object_ids,
        ))
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
        parent.raw().append(current.raw(), &mut self.nodes).expect(
            "Should never fail: The newly created node should always be successfully appended",
        );

        Ok(current)
    }

    /// Finds a toplevel node by the name.
    fn find_toplevel(&self, target_name: &str) -> Option<NodeId> {
        let target_sym = self.strings.get(target_name)?;
        for toplevel_id in self.root.raw().children(&self.nodes) {
            let toplevel = self
                .nodes
                .get(toplevel_id)
                .expect("Should never fail: node should exist");
            if toplevel.data.name_sym() == target_sym {
                return Some(NodeId::new(toplevel_id));
            }
        }
        None
    }

    /// Loads objects.
    fn load_objects(&mut self) -> Result<(), LoadError> {
        assert!(
            self.object_ids.is_empty(),
            "Attempt to initialize `self.object_ids` which has been already initialized"
        );

        // Cannot use `indextree::NodeId::children()`, because it borrows arena.

        // `/Objects/*` nodes.
        if let Some(objects_node_id) = self.find_toplevel("Objects") {
            let mut next_node_id = self
                .nodes
                .get(objects_node_id.raw())
                .expect("Should never fail: node should exist")
                .first_child()
                .map(NodeId::new);
            while let Some(object_node_id) = next_node_id {
                self.add_object(object_node_id)?;
                next_node_id = self
                    .nodes
                    .get(object_node_id.raw())
                    .expect("Should never fail: node should exist")
                    .next_sibling()
                    .map(NodeId::new);
            }
        } else {
            warn_noncritical!(self.strict, "`Objects` node not found");
            bail_if_strict!(
                self.strict,
                AccessError::NodeNotFound("`Objects`".to_owned()),
                return Ok(())
            );
        }

        // `/Documents/Document` nodes.
        if let Some(documents_node_id) = self.find_toplevel("Documents") {
            let document_sym = self.strings.get("Document");
            let mut next_node_id = self
                .nodes
                .get(documents_node_id.raw())
                .expect("Should never fail: node should exist")
                .first_child()
                .map(NodeId::new);
            while let Some(document_node_id) = next_node_id {
                if Some(
                    self.nodes
                        .get(document_node_id.raw())
                        .expect("Should never fail: node should exist")
                        .data
                        .name_sym(),
                ) == document_sym
                {
                    self.add_object(document_node_id)?;
                }
                next_node_id = self
                    .nodes
                    .get(document_node_id.raw())
                    .expect("Should never fail: node should exist")
                    .next_sibling()
                    .map(NodeId::new);
            }
        } else {
            warn_noncritical!(self.strict, "`Documents` node not found");
            bail_if_strict!(
                self.strict,
                AccessError::NodeNotFound("`Documents`".to_owned()),
                return Ok(())
            );
        }

        Ok(())
    }

    /// Register object node.
    fn add_object(&mut self, node_id: NodeId) -> Result<(), LoadError> {
        use std::collections::hash_map::Entry;

        let node = self
            .nodes
            .get(node_id.raw())
            .expect("Should never fail: node should exist");
        let obj_meta = match ObjectMeta::from_attributes(node.data.attributes(), &mut self.strings)
        {
            Ok(v) => v,
            Err(e) => {
                warn_noncritical!(self.strict, "Object load error: {}", e);
                bail_if_strict!(self.strict, e, return Ok(()));
            }
        };
        let obj_id = obj_meta.id();
        let node_id = ObjectNodeId::new(node_id);

        // Register to `object_ids`.
        match self.object_ids.entry(obj_id) {
            Entry::Occupied(entry) => {
                warn_noncritical!(
                    self.strict,
                    "Duplicate object ID: nodes with ID {:?} and {:?} have same object ID {:?}",
                    entry.get(),
                    node_id,
                    obj_id
                );
                bail_if_strict!(
                    self.strict,
                    LoadError::DuplicateId("object".to_owned(), format!("{:?}", obj_id)),
                    return Ok(())
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(node_id);
            }
        }

        Ok(())
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
            object_ids: HashMap::new(),
        }
    }
}
