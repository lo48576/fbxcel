//! `Deformer` object (blendshape).

use failure::{format_err, Error};

use crate::v7400::object::{
    deformer::{self, DeformerHandle},
    geometry, TypedObjectHandle,
};

define_object_subtype! {
    /// `Deformer` node handle (blendshape).
    BlendShapeHandle: DeformerHandle
}

impl<'a> BlendShapeHandle<'a> {
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
                    "Deformer blendshape object should have a parent geometry mesh: object={:?}",
                    self
                )
            })
    }

    /// Returns an iterator of child subdeformer blendshapechannels.
    pub fn blendshape_channels(
        &self,
    ) -> impl Iterator<Item = deformer::BlendShapeChannelHandle<'a>> + 'a {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::SubDeformer(
                    deformer::TypedSubDeformerHandle::BlendShapeChannel(o),
                ) => Some(o),
                _ => None,
            })
    }
}
