//! Objects graph.

use petgraph::graph::DiGraph;
use petgraph::Direction;
use std::collections::HashMap;

use crate::dom::v7400::object::connection::{Connection, ConnectionEdge};
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
    ///
    /// Note that there are no guarantee about edges order.
    // NOTE: `petgraph-0.4.13` guarantees that `Graph::neigbors_directed()` [1]
    // sees edges in reverse order of edges addition.
    // However, it is not explicitly specified that returned iterator from
    // `Graph::edges_directed()` [2] has specific ordering.
    //
    // [1]: https://docs.rs/petgraph/0.4.13/petgraph/graph/struct.Graph.html#method.neighbors_directed
    // [2]: https://docs.rs/petgraph/0.4.13/petgraph/graph/struct.Graph.html#method.edges_directed
    pub(crate) fn outgoing_edges_unordered(
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
    ///
    /// Note that there are no guarantee about edges order.
    // NOTE: `petgraph-0.4.13` guarantees that `Graph::neigbors_directed()` [1]
    // sees edges in reverse order of edges addition.
    // However, it is not explicitly specified that returned iterator from
    // `Graph::edges_directed()` [2] has specific ordering.
    //
    // [1]: https://docs.rs/petgraph/0.4.13/petgraph/graph/struct.Graph.html#method.neighbors_directed
    // [2]: https://docs.rs/petgraph/0.4.13/petgraph/graph/struct.Graph.html#method.edges_directed
    pub(crate) fn incoming_edges_unordered(
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
