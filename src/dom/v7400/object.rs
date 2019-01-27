//! Object.

use string_interner::StringInterner;

use crate::dom::v7400::{Core, Document, DowncastId, NodeId, StrSym};
use crate::dom::AccessError;
use crate::pull_parser::v7400::attribute::DirectAttributeValue;

use self::connection::ConnectionRef;
pub(crate) use self::graph::ObjectsGraph;
pub use self::scene::SceneNodeId;

pub mod connection;
mod graph;
pub mod scene;

/// Metadata of object node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectMeta {
    /// Object ID.
    id: ObjectId,
    /// Name (if exists).
    name: Option<String>,
    /// Class.
    class: StrSym,
    /// Subclass.
    subclass: StrSym,
}

impl ObjectMeta {
    /// Returns ID.
    pub fn id(&self) -> ObjectId {
        self.id
    }

    /// Returns object name.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|v| v.as_ref())
    }

    /// Returns object class.
    ///
    /// # Panics
    ///
    /// Panics if the data is stored in the given document.
    pub fn class<'a>(&self, core: &'a impl AsRef<Core>) -> &'a str {
        core.as_ref()
            .string(self.class)
            .expect("The `ObjectMeta` is not stored in the given document")
    }

    /// Returns object subclass.
    ///
    /// # Panics
    ///
    /// Panics if the data is stored in the given document.
    pub fn subclass<'a>(&self, core: &'a impl AsRef<Core>) -> &'a str {
        core.as_ref()
            .string(self.subclass)
            .expect("The `ObjectMeta` is not stored in the given document")
    }

    /// Returns the string symbol of the subclass.
    pub(crate) fn subclass_sym(&self) -> StrSym {
        self.subclass
    }

    /// Creates `ObjectMeta` from the given attributes.
    pub(crate) fn from_attributes(
        attrs: &[DirectAttributeValue],
        strings: &mut StringInterner<StrSym>,
    ) -> Result<Self, AccessError> {
        // Get ID.
        let id = match attrs.get(0) {
            Some(DirectAttributeValue::I64(v)) => ObjectId::new(*v),
            Some(_) => return Err(AccessError::UnexpectedAttributeType(Some(0))),
            None => return Err(AccessError::AttributeNotFound(Some(0))),
        };

        // Get name and class.
        let (name, class) = match attrs.get(1) {
            Some(DirectAttributeValue::String(name_class)) => {
                name_class.find("\u{0}\u{1}").map_or_else(
                    || (None, ""),
                    |sep_pos| {
                        (
                            Some(name_class[0..sep_pos].to_owned()),
                            &name_class[sep_pos + 2..],
                        )
                    },
                )
            }
            Some(_) => return Err(AccessError::UnexpectedAttributeType(Some(1))),
            None => return Err(AccessError::AttributeNotFound(Some(1))),
        };
        let class = strings.get_or_intern(class);

        // Get subclass.
        let subclass = match attrs.get(2) {
            Some(DirectAttributeValue::String(v)) => strings.get_or_intern(v.as_ref()),
            Some(_) => return Err(AccessError::UnexpectedAttributeType(Some(2))),
            None => return Err(AccessError::AttributeNotFound(Some(2))),
        };

        Ok(Self {
            id,
            name,
            class,
            subclass,
        })
    }
}

/// Object node ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectNodeId(NodeId);

impl ObjectNodeId {
    /// Creates a new `ObjectNodeId`.
    pub(crate) fn new(id: NodeId) -> Self {
        ObjectNodeId(id)
    }

    /// Returns the object meta data.
    ///
    /// # Panics
    ///
    /// Panics if the object node with the ID is stored in the given document.
    pub fn meta<'a>(&self, doc: &'a Document) -> &'a ObjectMeta {
        doc.parsed_node_data()
            .object_meta()
            .get(self)
            .expect("The object node with the `ObjectNodeId` is not stored in the given document")
    }

    /// Returns an iterator of the connections with source objects and
    /// properties.
    ///
    /// Note that this would not be ordered.
    /// To access them in correct order, sort by return value of
    /// [`ConnectionEdge::index()`].
    pub fn sources_undirected(self, doc: &Document) -> impl Iterator<Item = ConnectionRef<'_>> {
        self.meta(doc).id().sources_undirected(doc)
    }

    /// Returns an iterator of the connections with destination objects and
    /// properties.
    ///
    /// Note that this would not be ordered.
    /// To access them in correct order, sort by return value of
    /// [`ConnectionEdge::index()`].
    pub fn destinations_undirected(
        self,
        doc: &Document,
    ) -> impl Iterator<Item = ConnectionRef<'_>> {
        self.meta(doc).id().destinations_undirected(doc)
    }
}

impl From<ObjectNodeId> for NodeId {
    fn from(v: ObjectNodeId) -> Self {
        v.0
    }
}

impl DowncastId<ObjectNodeId> for NodeId {
    fn downcast(self, doc: &Document) -> Option<ObjectNodeId> {
        let maybe_invalid_id = ObjectNodeId::new(self);
        if doc
            .parsed_node_data()
            .object_meta()
            .contains_key(&maybe_invalid_id)
        {
            // Valid!
            Some(maybe_invalid_id)
        } else {
            // Invalid.
            None
        }
    }
}

impl DowncastId<ObjectNodeId> for ObjectId {
    fn downcast(self, doc: &Document) -> Option<ObjectNodeId> {
        doc.object_id_to_object_node_id(self)
    }
}

/// Object ID.
///
/// This is not object node ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(i64);

impl ObjectId {
    /// Creates a new `ObjectId`.
    pub(crate) fn new(v: i64) -> Self {
        ObjectId(v)
    }

    /// Returns an iterator of the connections with source objects and
    /// properties.
    ///
    /// Note that this would not be ordered.
    /// To access them in correct order, sort by return value of
    /// [`ConnectionEdge::index()`].
    pub fn sources_undirected(self, doc: &Document) -> impl Iterator<Item = ConnectionRef<'_>> {
        doc.objects_graph()
            .incoming_edges_unordered(self)
            .map(|(src, dest, edge)| ConnectionRef::new(src, dest, edge))
    }

    /// Returns an iterator of the connections with destination objects and
    /// properties.
    ///
    /// Note that this would not be ordered.
    /// To access them in correct order, sort by return value of
    /// [`ConnectionEdge::index()`].
    pub fn destinations_undirected(
        self,
        doc: &Document,
    ) -> impl Iterator<Item = ConnectionRef<'_>> {
        doc.objects_graph()
            .outgoing_edges_unordered(self)
            .map(|(src, dest, edge)| ConnectionRef::new(src, dest, edge))
    }
}
