//! `Material` object.

use crate::dom::v7400::object::{model, ObjectHandle, TypedObjectHandle};

define_object_subtype! {
    /// `Material` node handle.
    MaterialHandle: ObjectHandle
}

impl<'a> MaterialHandle<'a> {
    /// Returns an iterator of parent model mesh objects.
    pub fn meshes(&self) -> impl Iterator<Item = model::MeshHandle<'a>> + 'a {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(model::TypedModelHandle::Mesh(o)) => Some(o),
                _ => None,
            })
    }
}
