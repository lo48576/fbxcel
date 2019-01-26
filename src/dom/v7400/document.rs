//! FBX DOM document.

use std::collections::HashMap;

use crate::dom::v7400::object::{ObjectId, ObjectNodeId, ObjectsGraph};
use crate::dom::v7400::{Core, NodeId, ParsedData};

pub use self::loader::Loader;

mod loader;

/// FBX DOM document.
///
/// This manages not only tree structure, but also interpreted high-level
/// structures.
#[derive(Debug, Clone)]
pub struct Document {
    /// DOM core.
    core: Core,
    /// Map from object ID to node ID.
    object_ids: HashMap<ObjectId, ObjectNodeId>,
    /// Parsed node data.
    parsed_node_data: ParsedData,
    /// Objects graph.
    objects_graph: ObjectsGraph,
}

impl Document {
    /// Creates a new `Document`.
    pub(crate) fn new(
        core: Core,
        object_ids: HashMap<ObjectId, ObjectNodeId>,
        parsed_node_data: ParsedData,
        objects_graph: ObjectsGraph,
    ) -> Self {
        Self {
            core,
            object_ids,
            parsed_node_data,
            objects_graph,
        }
    }

    /// Returns the root node ID.
    pub fn root(&self) -> NodeId {
        self.core.root()
    }

    /// Returns the object node ID corresponding to the given object ID.
    pub(crate) fn object_id_to_object_node_id(&self, id: ObjectId) -> Option<ObjectNodeId> {
        self.object_ids.get(&id).cloned()
    }

    /// Returns the reference to the parsed node data.
    pub fn parsed_node_data(&self) -> &ParsedData {
        &self.parsed_node_data
    }

    /// Returns the reference to the objects graph.
    pub(crate) fn objects_graph(&self) -> &ObjectsGraph {
        &self.objects_graph
    }
}

impl AsRef<Core> for Document {
    fn as_ref(&self) -> &Core {
        &self.core
    }
}
