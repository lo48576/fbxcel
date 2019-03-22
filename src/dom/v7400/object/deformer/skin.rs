//! `Deformer` object (skin).

use failure::{format_err, Error};

use crate::dom::v7400::object::{deformer::DeformerHandle, geometry, TypedObjectHandle};

define_object_subtype! {
    /// `Deformer` node handle (skin).
    SkinHandle: DeformerHandle
}

impl<'a> SkinHandle<'a> {
    /// Returns the parant geometry mesh.
    pub fn mesh(&self) -> Result<geometry::MeshHandle<'a>, Error> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Geometry(geometry::TypedGeometryHandle::Mesh(o)) => Some(o),
                _ => None,
            })
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Deformer skin object should have a parent geometry mesh: object={:?}",
                    self
                )
            })
    }
}
