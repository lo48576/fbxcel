//! Objects-related stuff.

use crate::{domcast::v7400::Document, tree::v7400::NodeId};

pub(crate) use self::{
    cache::ObjectsCache,
    meta::{ObjectClassSym, ObjectMeta},
};

mod cache;
mod meta;

/// Node ID of a object node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectNodeId(NodeId);

impl ObjectNodeId {
    /// Creates a new `ObjectNodeId`.
    pub(crate) fn new(node_id: NodeId) -> Self {
        Self(node_id)
    }

    /// Creates a new `ObjectHandle`.
    pub fn to_object_handle(self, doc: &Document) -> ObjectHandle<'_> {
        ObjectHandle::from_object_node_id(self, doc)
    }
}

impl From<ObjectNodeId> for NodeId {
    fn from(v: ObjectNodeId) -> Self {
        v.0
    }
}

/// Object ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(i64);

impl ObjectId {
    /// Creates a new `ObjectId`.
    fn new(id: i64) -> Self {
        Self(id)
    }

    /// Creates a new `ObjectHandle`.
    pub fn to_object_handle(self, doc: &Document) -> ObjectHandle<'_> {
        ObjectHandle::from_object_id(self, doc)
    }
}

/// Object handle.
#[derive(Debug, Clone, Copy)]
pub struct ObjectHandle<'a> {
    /// Node ID.
    node_id: ObjectNodeId,
    /// Object metadata.
    object_meta: &'a ObjectMeta,
    /// Document.
    doc: &'a Document,
}

impl<'a> ObjectHandle<'a> {
    /// Creates a new `ObjectHandle` from the given object node ID.
    ///
    /// # Panics
    ///
    /// This may panic if the object node with the given ID does not exist in
    /// the given document.
    fn from_object_node_id(node_id: ObjectNodeId, doc: &'a Document) -> Self {
        let object_meta = doc
            .objects()
            .meta_from_node_id(node_id)
            .unwrap_or_else(|| panic!("No corresponding object metadata: node_id={:?}", node_id));
        Self {
            node_id,
            object_meta,
            doc,
        }
    }

    /// Creates a new `ObjectHandle` from the given object node ID.
    ///
    /// # Panics
    ///
    /// This may panic if the object node with the given ID does not exist in
    /// the given document.
    fn from_object_id(obj_id: ObjectId, doc: &'a Document) -> Self {
        let node_id = doc
            .objects()
            .node_id(obj_id)
            .unwrap_or_else(|| panic!("No corresponding object node: object_id={:?}", obj_id));
        let object_meta = doc
            .objects()
            .meta_from_node_id(node_id)
            .expect("Should never fail: object cache should be consistent");
        assert_eq!(obj_id, object_meta.object_id(), "Object ID mismatch");
        Self {
            node_id,
            object_meta,
            doc,
        }
    }

    /// Returns object node ID.
    pub fn object_node_id(&self) -> ObjectNodeId {
        self.node_id
    }

    /// Returns object ID.
    pub fn object_id(&self) -> ObjectId {
        self.object_meta.object_id()
    }

    /// Returns object name.
    pub fn name(&self) -> Option<&'a str> {
        self.object_meta.name()
    }

    /// Returns object class.
    pub fn class(&self) -> &'a str {
        self.doc
            .objects()
            .resolve_class_string(self.object_meta.class_sym())
    }

    /// Returns object subclass.
    pub fn subclass(&self) -> &'a str {
        self.doc
            .objects()
            .resolve_class_string(self.object_meta.subclass_sym())
    }
}
