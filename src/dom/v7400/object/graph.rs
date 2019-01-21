//! Objects graph.

use petgraph::graph::DiGraph;
use petgraph::Direction;
use std::collections::HashMap;

use crate::dom::v7400::connection::{Connection, ConnectionEdge};
use crate::dom::v7400::object::ObjectId;

/// Internal representation of graph node index type.
type GraphNodeIndexInner = petgraph::graph::DefaultIx;

/// Graph node index type.
type GraphNodeIndex = petgraph::graph::NodeIndex<GraphNodeIndexInner>;

/// Objects graph.
#[derive(Default, Debug, Clone)]
pub struct ObjectsGraph {
    /// Graph structure.
    graph: DiGraph<ObjectId, ConnectionEdge, GraphNodeIndexInner>,
    /// Mapping from object ID to graph node index.
    obj_id_to_graph_node_index: HashMap<ObjectId, GraphNodeIndex>,
}

impl ObjectsGraph {
    /// Returns `GraphNodeIndex` corresponding to the given `ObjectId`.
    fn graph_node_index(&self, obj_id: ObjectId) -> Option<GraphNodeIndex> {
        self.obj_id_to_graph_node_index.get(&obj_id).cloned()
    }

    /// Creates a node if necessary and returns node index.
    fn add_or_get_graph_node_index(&mut self, obj_id: ObjectId) -> GraphNodeIndex {
        self.obj_id_to_graph_node_index
            .get(&obj_id)
            .cloned()
            .unwrap_or_else(|| self.graph.add_node(obj_id))
    }

    /// Returns `GraphNodeIndex` corresponding to the given `ObjectId`.
    ///
    /// # Panics
    ///
    /// Panics if the object ID is not added to the graph.
    fn object_id(&self, node_index: GraphNodeIndex) -> ObjectId {
        self.graph
            .node_weight(node_index)
            .cloned()
            .expect("The given object ID is not added to the graph")
    }

    /// Inserts or updates the given connection.
    ///
    /// This does not create duplicate edge.
    pub(crate) fn add_connection(&mut self, connection: Connection) {
        let source = self.add_or_get_graph_node_index(connection.source_id());
        let destination = self.add_or_get_graph_node_index(connection.destination_id());
        self.graph
            .update_edge(source, destination, *connection.edge());
    }

    /// Returns the weight of the edge if available.
    pub(crate) fn edge_weight(
        &self,
        source: ObjectId,
        destination: ObjectId,
    ) -> Option<&ConnectionEdge> {
        let source = self.graph_node_index(source)?;
        let destination = self.graph_node_index(destination)?;
        self.graph
            .find_edge(source, destination)
            .and_then(|edge| self.graph.edge_weight(edge))
    }

    /// Returns the iterator of outgoing edges.
    pub(crate) fn outgoing_edges(
        &self,
        source: ObjectId,
    ) -> impl Iterator<Item = (ObjectId, ObjectId, &ConnectionEdge)> {
        use petgraph::visit::EdgeRef;

        self.graph_node_index(source)
            .into_iter()
            .flat_map(move |source| {
                self.graph
                    .edges_directed(source, Direction::Outgoing)
                    .map(move |edge| {
                        let source = self.object_id(edge.source());
                        let destination = self.object_id(edge.target());
                        (source, destination, edge.weight())
                    })
            })
    }

    /// Returns the iterator of incoming edges.
    pub(crate) fn incoming_edges(
        &self,
        destination: ObjectId,
    ) -> impl Iterator<Item = (ObjectId, ObjectId, &ConnectionEdge)> {
        use petgraph::visit::EdgeRef;

        self.graph_node_index(destination)
            .into_iter()
            .flat_map(move |destination| {
                self.graph
                    .edges_directed(destination, Direction::Incoming)
                    .map(move |edge| {
                        let source = self.object_id(edge.source());
                        let destination = self.object_id(edge.target());
                        (source, destination, edge.weight())
                    })
            })
    }
}
