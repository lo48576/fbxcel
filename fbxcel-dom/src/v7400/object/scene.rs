//! `Document` node.

use failure::{format_err, Error};

use crate::v7400::object::{ObjectHandle, ObjectId};

/// `Document` node (`Scene` object) handle.
///
/// `Document` node (`Scene` object) contains root object ID of a scene.
#[derive(Debug, Clone, Copy)]
pub struct SceneHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> SceneHandle<'a> {
    /// Creates a new `SceneHandle` if the given object is `Document` node.
    pub(crate) fn new(object: ObjectHandle<'a>) -> Option<Self> {
        let is_document_node = object
            .document()
            .objects_cache()
            .document_nodes()
            .contains(&object.object_node_id());
        if !is_document_node {
            return None;
        }
        Some(Self { object })
    }

    /// Returns the root object ID of the scene.
    pub fn root_object_id(&self) -> Result<ObjectId, Error> {
        self.object
            .node()
            .children_by_name("RootNode")
            .next()
            .ok_or_else(|| format_err!("`RootNode` not found for scene object node"))?
            .attributes()
            .get(0)
            .ok_or_else(|| format_err!("Attributes not found for `RootNode`"))?
            .get_i64_or_type()
            .map(ObjectId::new)
            .map_err(|ty| {
                format_err!(
                    "Unexpected attribute type for `RootNode`: expected `i64` but got {:?}",
                    ty
                )
            })
    }

    /// Returns the root object of the scene.
    ///
    /// Note that this returns `Err(_)` if the object has no corresponding node.
    /// This can happen for valid FBX data.
    pub fn root_object(&self) -> Result<ObjectHandle<'_>, Error> {
        self.root_object_id()?
            .to_object_handle(self.object.document())
            .ok_or_else(|| {
                format_err!(
                    "Root object of the scene has no corresponding node: object_id={:?}",
                    self.object.object_id()
                )
            })
    }
}

impl<'a> std::ops::Deref for SceneHandle<'a> {
    type Target = ObjectHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
