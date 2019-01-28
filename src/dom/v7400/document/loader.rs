//! FBX DOM document loader.

use std::collections::HashMap;

use log::{debug, trace, warn};

use crate::dom::v7400::document::ParsedData;
use crate::dom::v7400::object::connection::Connection;
use crate::dom::v7400::object::scene::SceneNodeData;
use crate::dom::v7400::object::{ObjectId, ObjectMeta, ObjectNodeId, ObjectsGraph, SceneNodeId};
use crate::dom::v7400::{Core, Document, NodeId};
use crate::dom::{AccessError, LoadError};
use crate::pull_parser::v7400::Parser;
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

/// DOM document loader config.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loader {
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
        Self { strict: v }
    }

    /// Loads the DOM document from the parser.
    pub fn load_document<R>(self, parser: &mut Parser<R>) -> Result<Document, LoadError>
    where
        R: ParserSource,
    {
        LoaderImpl::new(parser, self)?.load_document()
    }
}

/// DOM document loader.
#[derive(Debug, Clone)]
struct LoaderImpl {
    /// Loader config.
    config: Loader,
    /// DOM core.
    core: Core,
    /// Map from object ID to node ID.
    object_ids: HashMap<ObjectId, ObjectNodeId>,
    /// Parsed node data.
    parsed_node_data: ParsedData,
    /// Objects graph.
    objects_graph: ObjectsGraph,
}

impl LoaderImpl {
    /// Creates a new loader from the given config and parser.
    fn new<R>(parser: &mut Parser<R>, config: Loader) -> Result<Self, LoadError>
    where
        R: ParserSource,
    {
        Ok(Self {
            config,
            core: Core::load(parser)?,
            object_ids: Default::default(),
            parsed_node_data: Default::default(),
            objects_graph: Default::default(),
        })
    }

    /// Returns the strict flag.
    #[inline]
    fn is_strict(&self) -> bool {
        self.config.strict
    }

    /// Loads the DOM document from the parser.
    fn load_document(mut self) -> Result<Document, LoadError> {
        debug!("Loading v7400 DOM from parser: config={:?}", self.config);

        // Load objects.
        self.load_objects()?;

        // Load object connections.
        self.load_connections()?;

        debug!("successfully loaded v7400 DOM");

        Ok(Document::new(
            self.core,
            self.object_ids,
            self.parsed_node_data,
            self.objects_graph,
        ))
    }

    /// Loads objects.
    fn load_objects(&mut self) -> Result<(), LoadError> {
        trace!("Loading objects");

        assert!(
            self.object_ids.is_empty(),
            "Attempt to initialize `self.object_ids` which has been already initialized"
        );

        // Cannot use `indextree::NodeId::children()`, because it borrows arena.

        // `/Objects/*` nodes.
        if let Some(objects_node_id) = self.core.find_toplevel("Objects") {
            trace!("Loading `/Objects/*` under node_id={:?}", objects_node_id);

            let mut next_node_id = self.core.node(objects_node_id).first_child();
            while let Some(object_node_id) = next_node_id {
                trace!("Found object node: node_id={:?}", object_node_id);

                self.add_object(object_node_id)?;
                next_node_id = self.core.node(object_node_id).next_sibling();
            }
        } else {
            warn_noncritical!(self.is_strict(), "`Objects` node not found");
            bail_if_strict!(
                self.is_strict(),
                AccessError::NodeNotFound("`Objects`".to_owned()),
                return Ok(())
            );
        }

        // `/Documents/Document` nodes.
        if let Some(documents_node_id) = self.core.find_toplevel("Documents") {
            trace!(
                "Loading `/Documents/Document` under node_id={:?}",
                documents_node_id
            );

            let document_sym = self.core.sym("Document");
            let scene_sym = self.core.sym("Scene");
            let mut next_node_id = self.core.node(documents_node_id).first_child();
            while let Some(document_node_id) = next_node_id {
                if self.core.node(document_node_id).data().name_sym() == document_sym {
                    trace!("Found `Document` node: node_id={:?}", document_node_id);

                    self.add_object(document_node_id)?;

                    trace!("Interpreting document (scene) data");
                    let object_node_id = ObjectNodeId::new(document_node_id);
                    let node_meta = self
                        .parsed_node_data
                        .object_meta()
                        .get(&object_node_id)
                        .expect("Should never fail: `add_object()` should have added the entry");
                    if node_meta.subclass_sym() == scene_sym {
                        // Add scene data to `parsed_node_data`.
                        match SceneNodeData::load(object_node_id, &self.core) {
                            Ok(data) => {
                                trace!("Successfully interpreted `Document` node as scene data: data={:?}", data);

                                let scene_node_id = SceneNodeId::new(object_node_id);
                                self.parsed_node_data
                                    .scenes_mut()
                                    .entry(scene_node_id)
                                    .or_insert(data);
                            }
                            Err(e) => {
                                warn_noncritical!(
                                    self.is_strict(),
                                    "Failed to load scene object node data from `Document` node"
                                );
                                bail_if_strict!(self.is_strict(), e, return Ok(()));
                            }
                        }
                    } else {
                        warn_noncritical!(
                            self.is_strict(),
                            "`Document` node does not have `Scene` subclass"
                        );
                        bail_if_strict!(
                            self.is_strict(),
                            LoadError::UnexpectedObjectType(
                                "`Scene`".into(),
                                self.core
                                    .string(node_meta.subclass_sym())
                                    .expect(
                                        "Should never fail: subclass string should be \
                                         registered by `add_object()`"
                                    )
                                    .into(),
                            ),
                            return Ok(())
                        );
                    }
                }
                next_node_id = self.core.node(document_node_id).next_sibling();
            }
        } else {
            warn_noncritical!(self.is_strict(), "`Documents` node not found");
            bail_if_strict!(
                self.is_strict(),
                AccessError::NodeNotFound("`Documents`".to_owned()),
                return Ok(())
            );
        }

        trace!("Successfully loaded objects");

        Ok(())
    }

    /// Registers object node.
    fn add_object(&mut self, node_id: NodeId) -> Result<(), LoadError> {
        use std::collections::hash_map::Entry;

        trace!("Loading object: node_id={:?}", node_id);

        let obj_meta = {
            let (node, strings) = self.core.node_and_strings(node_id);
            let attrs = node.data().attributes();
            match ObjectMeta::from_attributes(attrs, strings) {
                Ok(v) => v,
                Err(e) => {
                    warn_noncritical!(self.is_strict(), "Object load error: {}", e);
                    bail_if_strict!(self.is_strict(), e, return Ok(()));
                }
            }
        };
        let obj_id = obj_meta.id();
        let node_id = ObjectNodeId::new(node_id);
        trace!("Interpreted object: id={:?}, meta={:?}", node_id, obj_meta);

        // Register to `object_ids`.
        match self.object_ids.entry(obj_id) {
            Entry::Occupied(entry) => {
                warn_noncritical!(
                    self.config.strict,
                    "Duplicate object ID: nodes with ID {:?} and {:?} have same object ID {:?}",
                    entry.get(),
                    node_id,
                    obj_id
                );
                bail_if_strict!(
                    self.is_strict(),
                    LoadError::DuplicateId("object".to_owned(), format!("{:?}", obj_id)),
                    return Ok(())
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(node_id);
            }
        }

        let meta_dup = self
            .parsed_node_data
            .object_meta_mut()
            .insert(node_id, obj_meta)
            .is_some();
        assert!(!meta_dup);

        trace!(
            "Successfully loaded object: node_id={:?}, obj_id={:?}",
            node_id,
            obj_id
        );

        Ok(())
    }

    /// Load connetions.
    fn load_connections(&mut self) -> Result<(), LoadError> {
        trace!("Loading objects connections");

        // `/Connections/C` nodes.
        if let Some(connections_node_id) = self.core.find_toplevel("Connections") {
            trace!(
                "Loading `/Connections/C` nodes under {:?}",
                connections_node_id
            );

            let c_sym = self.core.sym("C");
            let mut next_node_id = self.core.node(connections_node_id).first_child();
            let mut conn_index = 0;
            while let Some(connection_node_id) = next_node_id {
                trace!("Found `C` node: node_id={:?}", connection_node_id);
                if self.core.node(connection_node_id).data().name_sym() == c_sym {
                    self.add_connection(connection_node_id, conn_index)?;
                }
                next_node_id = self.core.node(connection_node_id).next_sibling();
                conn_index = conn_index.checked_add(1).expect("Too many connections");
            }
        } else {
            warn_noncritical!(self.is_strict(), "`Connections` node not found");
            bail_if_strict!(
                self.is_strict(),
                AccessError::NodeNotFound("`Connections`".to_owned()),
                return Ok(())
            );
        }

        trace!("Successfully loaded objects connections");

        Ok(())
    }

    /// Registers object connection.
    fn add_connection(&mut self, node_id: NodeId, conn_index: usize) -> Result<(), LoadError> {
        trace!(
            "Adding a connection: node_id={:?}, conn_index={:?}",
            node_id,
            conn_index
        );

        let conn = {
            let (node, strings) = self.core.node_and_strings(node_id);
            let attrs = node.data().attributes();
            Connection::load_from_attributes(attrs, strings, conn_index)?
        };
        trace!(
            "Interpreted connection: node_id={:?}, conn={:?}",
            node_id,
            conn
        );

        if let Some(old_conn) = self
            .objects_graph
            .edge_weight(conn.source_id(), conn.destination_id())
        {
            warn_noncritical!(
                self.is_strict(),
                "Duplicate object connections: found more than two objects connections \
                 from {:?} to {:?} edge={:?}, ignored={:?}",
                conn.source_id(),
                conn.destination_id(),
                old_conn,
                conn.edge()
            );
            bail_if_strict!(
                self.is_strict(),
                LoadError::DuplicateConnection(
                    "objects".to_owned(),
                    format!("{:?}", conn.source_id()),
                    format!("{:?}", conn.destination_id())
                ),
                return Ok(())
            );
        }
        self.objects_graph.add_connection(conn);

        trace!(
            "Successfully added the connection: node_id={:?}, conn_index={:?}",
            node_id,
            conn_index
        );

        Ok(())
    }
}
