//! Object.

use log::trace;
use string_interner::StringInterner;

use crate::dom::error::StructureError;
use crate::dom::v7400::{Core, Document, DowncastId, NodeId, StrSym, ValidateId};
use crate::pull_parser::v7400::attribute::DirectAttributeValue;

use self::connection::Connection;
pub(crate) use self::graph::{ObjectsGraph, ObjectsGraphBuilder};
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
    ) -> Result<Self, StructureError> {
        trace!("Loading object metadata");

        // Get ID.
        let id = match attrs.get(0) {
            Some(DirectAttributeValue::I64(v)) => ObjectId::new(*v),
            Some(v) => {
                return Err(StructureError::unexpected_attribute_type(
                    Some(0),
                    "`i64`",
                    format!("{:?}", v.type_()),
                ));
            }
            None => return Err(StructureError::attribute_not_found(Some(0))),
        };
        trace!("Got object id: {:?}", id);

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
            Some(v) => {
                return Err(StructureError::unexpected_attribute_type(
                    Some(1),
                    "string",
                    format!("{:?}", v.type_()),
                ));
            }
            None => return Err(StructureError::attribute_not_found(Some(1))),
        };
        trace!("Got name and class: name={:?}, class={:?}", name, class);
        let class = strings.get_or_intern(class);

        // Get subclass.
        let subclass = match attrs.get(2) {
            Some(DirectAttributeValue::String(v)) => {
                trace!("Got subclass: {:?}", v);
                strings.get_or_intern(v.as_ref())
            }
            Some(v) => {
                return Err(StructureError::unexpected_attribute_type(
                    Some(2),
                    "string",
                    format!("{:?}", v.type_()),
                ));
            }
            None => return Err(StructureError::attribute_not_found(Some(2))),
        };

        trace!("Successfully loaded object metadata");

        Ok(Self {
            id,
            name,
            class,
            subclass,
        })
    }
}

define_node_id_type! {
    /// Object node ID.
    ObjectNodeId {
        ancestors { NodeId }
    }
}

impl ValidateId for ObjectNodeId {
    fn validate_id(self, doc: &Document) -> bool {
        doc.parsed_node_data().object_meta().contains_key(&self)
    }
}

impl ObjectNodeId {
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
    pub fn sources_undirected(self, doc: &Document) -> impl Iterator<Item = &Connection> {
        self.meta(doc).id().sources_undirected(doc)
    }

    /// Returns an iterator of the connections with destination objects and
    /// properties.
    ///
    /// Note that this would not be ordered.
    /// To access them in correct order, sort by return value of
    /// [`ConnectionEdge::index()`].
    pub fn destinations_undirected(self, doc: &Document) -> impl Iterator<Item = &Connection> {
        self.meta(doc).id().destinations_undirected(doc)
    }
}

impl DowncastId<ObjectNodeId> for ObjectId {
    fn downcast(self, doc: &Document) -> Option<ObjectNodeId> {
        trace!("Trying to downcast {:?} to `ObjectNodeId`", self);

        let result = doc.object_id_to_object_node_id(self);
        match result {
            Some(id) => trace!("Successfully downcasted {:?} to {:?}", self, id),
            None => trace!(
                "Downcast failed: {:?} is not convertible to `ObjectNodeId`",
                self
            ),
        }
        result
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
    pub fn sources_undirected(self, doc: &Document) -> impl Iterator<Item = &Connection> {
        doc.objects_graph().incoming_edges_unordered(self)
    }

    /// Returns an iterator of the connections with destination objects and
    /// properties.
    ///
    /// Note that this would not be ordered.
    /// To access them in correct order, sort by return value of
    /// [`ConnectionEdge::index()`].
    pub fn destinations_undirected(self, doc: &Document) -> impl Iterator<Item = &Connection> {
        doc.objects_graph().outgoing_edges_unordered(self)
    }
}
