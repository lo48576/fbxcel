//! FBX DOM document loader.

use std::collections::HashMap;

use failure::format_err;
use log::{debug, trace, warn};

use crate::dom::error::{LoadError, LoadErrorKind, StructureError};
use crate::dom::v7400::document::ParsedData;
use crate::dom::v7400::object::connection::Connection;
use crate::dom::v7400::object::model::{ModelNodeData, ModelNodeId};
use crate::dom::v7400::object::scene::{SceneNodeData, SceneNodeId};
use crate::dom::v7400::object::{ObjectId, ObjectMeta, ObjectNodeId, ObjectsGraphBuilder};
use crate::dom::v7400::{Core, Document, NodeId};
use crate::pull_parser::v7400::Parser;
use crate::pull_parser::ParserSource;

/// DOM document loader config.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Loader;

impl Loader {
    /// Creates a new `Loader`.
    pub fn new() -> Self {
        Self::default()
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
    /// Objects graph builder.
    objects_graph: ObjectsGraphBuilder,
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
            self.objects_graph.build(),
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
        let objects_node_id = self
            .core
            .find_toplevel("Objects")
            .ok_or_else(|| StructureError::node_not_found("`Objects`").with_context_node(""))?;

        trace!("Loading `/Objects/*` under node_id={:?}", objects_node_id);

        let mut next_node_id = self.core.node(objects_node_id).first_child();
        while let Some(object_node_id) = next_node_id {
            trace!("Found object node: node_id={:?}", object_node_id);

            self.add_object(object_node_id)?;
            next_node_id = self.core.node(object_node_id).next_sibling();
        }

        // `/Documents/Document` nodes.
        let documents_node_id = self
            .core
            .find_toplevel("Documents")
            .ok_or_else(|| StructureError::node_not_found("`Documents`").with_context_node(""))?;
        trace!(
            "Loading `/Documents/Document` under node_id={:?}",
            documents_node_id
        );

        let document_sym = self.core.sym("Document");
        let mut next_node_id = self.core.node(documents_node_id).first_child();
        while let Some(document_node_id) = next_node_id {
            if self.core.node(document_node_id).data().name_sym() == document_sym {
                trace!("Found `Document` node: node_id={:?}", document_node_id);

                self.add_object(document_node_id)?;
            }
            next_node_id = self.core.node(document_node_id).next_sibling();
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
            ObjectMeta::from_attributes(attrs, strings)
                .map_err(|e| e.with_context_node((&self.core, node_id)))?
        };
        let obj_id = obj_meta.id();
        let node_id = ObjectNodeId::new(node_id);
        trace!("Interpreted object: id={:?}, meta={:?}", node_id, obj_meta);

        // Register to `object_ids`.
        match self.object_ids.entry(obj_id) {
            Entry::Occupied(entry) => {
                return Err(format_err!(
                    "Duplicate object ID: {:?} (nodes=({:?}, {:?}))",
                    obj_id,
                    entry.get(),
                    node_id
                )
                .context(LoadErrorKind::Value)
                .into());
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

        self.register_object_type_specific_data(node_id, obj_id)?;

        Ok(())
    }

    /// Parse object-type-specific data and store them in `self`.
    fn register_object_type_specific_data(
        &mut self,
        node_id: ObjectNodeId,
        obj_id: ObjectId,
    ) -> Result<(), LoadError> {
        let node_name = node_id.node(&self.core).name(&self.core);
        let meta = self
            .parsed_node_data
            .object_meta()
            .get(&node_id)
            .expect("Should never fail: object metadata should be registered");
        let (class, subclass) = (meta.class(&self.core), meta.subclass(&self.core));
        trace!(
            "Start registering object-type-specific data: node_id={:?}, obj_id={:?}, \
             node_name={:?}, class={:?}, subclass={:?}",
            node_id,
            obj_id,
            node_name,
            class,
            subclass
        );
        // Load object-type-specific data and add it to `parsed_node_data`.
        match node_name {
            "Document" => {
                if subclass != "Scene" {
                    return Err(format_err!(
                        "Unexpected object type for `Document` node: expected `Scene`, got {:?}, context_node={}",
                        subclass, self.core.path(node_id).debug_display()
                    )
                    .context(LoadErrorKind::Value).into());
                }

                let data = SceneNodeData::load(node_id, &self.core)?;
                trace!(
                    "Successfully interpreted `Document` node as scene data: data={:?}",
                    data
                );

                let scene_node_id = SceneNodeId::new(node_id);
                self.parsed_node_data
                    .scenes_mut()
                    .entry(scene_node_id)
                    .or_insert(data);
            }
            "Model" => {
                let data = ModelNodeData::load(node_id, &self.core)?;
                trace!(
                    "Successfully interpreted `Model` node data: data={:?}",
                    data
                );

                let model_node_id = ModelNodeId::new(node_id);
                self.parsed_node_data
                    .models_mut()
                    .entry(model_node_id)
                    .or_insert(data);
            }
            node_name => {
                warn!("Unsupported object type: node_name={:?}", node_name);
            }
        }

        trace!(
            "Successfully registered object-type-specific data: node_id={:?}, obj_id={:?}",
            node_id,
            obj_id
        );

        Ok(())
    }

    /// Load connetions.
    fn load_connections(&mut self) -> Result<(), LoadError> {
        trace!("Loading objects connections");

        // `/Connections/C` nodes.
        let connections_node_id = self
            .core
            .find_toplevel("Connections")
            .ok_or_else(|| StructureError::node_not_found("`Connections`").with_context_node(""))?;

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
            Connection::load_from_attributes(attrs, strings, conn_index)
                .map_err(|e| e.with_context_node((&self.core, node_id)))?
        };
        trace!(
            "Interpreted connection: node_id={:?}, conn={:?}",
            node_id,
            conn
        );

        if let Some(old_conn) =
            self.objects_graph
                .connection(conn.source_id(), conn.destination_id(), conn.label_sym())
        {
            return Err(format_err!(
                "Duplicate connection between objects: \
                 source={:?}, dest={:?}, preserved={:?}, ignored={:?}",
                conn.source_id(),
                conn.destination_id(),
                old_conn,
                conn,
            )
            .context(LoadErrorKind::Value)
            .into());
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
