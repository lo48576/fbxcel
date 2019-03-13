//! Objects graph.

use std::collections::BTreeMap;

use crate::dom::v7400::object::connection::Connection;
use crate::dom::v7400::object::ObjectId;
use crate::dom::v7400::StrSym;

/// Objects graph.
#[derive(Default, Debug, Clone)]
pub struct ObjectsGraph {
    /// Edges.
    edges: Vec<Connection>,
    /// Edge indices sorted by source object ID.
    edge_indices_sorted_by_src: Vec<usize>,
    /// Edge indices sorted by destination object ID.
    edge_indices_sorted_by_dest: Vec<usize>,
}

impl ObjectsGraph {
    /// Returns the iterator of outgoing edges.
    ///
    /// Note that there are no guarantee about edges order.
    pub(crate) fn outgoing_edges_unordered(
        &self,
        source: ObjectId,
    ) -> impl Iterator<Item = &Connection> {
        assert_eq!(self.edges.len(), self.edge_indices_sorted_by_src.len());

        let start = self
            .edge_indices_sorted_by_src
            .binary_search_by(|&idx| self.edges[idx].source_id().cmp(&source))
            .unwrap_or_else(|_| self.edges.len());
        self.edge_indices_sorted_by_src[start..]
            .iter()
            .map(move |&edge_index| &self.edges[edge_index])
            .filter(move |edge| edge.source_id() == source)
            .fuse()
    }

    /// Returns the iterator of incoming edges.
    ///
    /// Note that there are no guarantee about edges order.
    pub(crate) fn incoming_edges_unordered(
        &self,
        destination: ObjectId,
    ) -> impl Iterator<Item = &Connection> {
        assert_eq!(self.edges.len(), self.edge_indices_sorted_by_dest.len());

        let start = self
            .edge_indices_sorted_by_dest
            .binary_search_by(|&idx| self.edges[idx].destination_id().cmp(&destination))
            .unwrap_or_else(|_| self.edges.len());
        self.edge_indices_sorted_by_src[start..]
            .iter()
            .map(move |&edge_index| &self.edges[edge_index])
            .filter(move |edge| edge.destination_id() == destination)
            .fuse()
    }
}

/// Objects graph.
#[derive(Default, Debug, Clone)]
pub(crate) struct ObjectsGraphBuilder {
    /// Edges.
    edges: Vec<Connection>,
    /// Edge indices sorted by source object ID.
    edge_indices_sorted_by_src: BTreeMap<ObjectId, Vec<usize>>,
    /// Edge indices sorted by destination object ID.
    edge_indices_sorted_by_dest: BTreeMap<ObjectId, Vec<usize>>,
}

impl ObjectsGraphBuilder {
    /// Builds an `ObjectsGraph`.
    pub(crate) fn build(self) -> ObjectsGraph {
        let edge_indices_sorted_by_src = self
            .edge_indices_sorted_by_src
            .into_iter()
            .flat_map(|(_, v)| v)
            .collect();
        let edge_indices_sorted_by_dest = self
            .edge_indices_sorted_by_dest
            .into_iter()
            .flat_map(|(_, v)| v)
            .collect();
        ObjectsGraph {
            edges: self.edges,
            edge_indices_sorted_by_src,
            edge_indices_sorted_by_dest,
        }
    }

    /// Inserts or updates the given connection.
    ///
    /// This does not create duplicate edge.
    pub(crate) fn add_connection(&mut self, connection: Connection) {
        let edge_index = self.edges.len();
        let src = connection.source_id();
        let dest = connection.destination_id();
        self.edges.push(connection);
        self.edge_indices_sorted_by_src
            .entry(src)
            .or_insert_with(Vec::new)
            .push(edge_index);
        self.edge_indices_sorted_by_dest
            .entry(dest)
            .or_insert_with(Vec::new)
            .push(edge_index);
    }

    /// Returns the connection if available.
    pub(crate) fn connection(
        &self,
        source: ObjectId,
        destination: ObjectId,
        label_sym: Option<StrSym>,
    ) -> Option<&Connection> {
        let entries = self.edge_indices_sorted_by_src.get(&source)?;
        entries
            .iter()
            .map(|&edge_index| &self.edges[edge_index])
            .find(|edge| {
                edge.destination_id() == destination && edge.edge().label_sym() == label_sym
            })
    }
}
