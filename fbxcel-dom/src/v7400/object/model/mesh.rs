//! `Model` object (mesh).

use failure::{format_err, Error};

use crate::v7400::object::{geometry, material, model::ModelHandle, TypedObjectHandle};

define_object_subtype! {
    /// `Model` node handle (mesh).
    MeshHandle: ModelHandle
}

impl<'a> MeshHandle<'a> {
    /// Returns object handle of child geometry object.
    pub fn geometry(&self) -> Result<geometry::MeshHandle<'a>, Error> {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Geometry(geometry::TypedGeometryHandle::Mesh(o)) => Some(o),
                _ => None,
            })
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Model mesh should have a child geometry mesh, but not found: object={:?}",
                    self
                )
            })
    }

    /// Returns an iterator of child material objects.
    pub fn materials(&self) -> impl Iterator<Item = material::MaterialHandle<'a>> + 'a {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Material(o) => Some(o),
                _ => None,
            })
    }
}
