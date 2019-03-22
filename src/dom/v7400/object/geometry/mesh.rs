//! `Geometry` object (mesh).

use crate::dom::v7400::object::{geometry::GeometryHandle, model, TypedObjectHandle};

define_object_subtype! {
    /// `Geometry` node handle (mesh).
    MeshHandle: GeometryHandle
}

impl<'a> MeshHandle<'a> {
    /// Returns an iterator of parent model objects.
    pub fn models(&self) -> impl Iterator<Item = model::MeshHandle<'a>> + 'a {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(model::TypedModelHandle::Mesh(o)) => Some(o),
                _ => None,
            })
    }
}
