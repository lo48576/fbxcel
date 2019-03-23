//! `SubDeformer` object (blendshapechannel).

use failure::{format_err, Error};

use crate::v7400::object::{
    deformer::{self, SubDeformerHandle},
    geometry, TypedObjectHandle,
};

define_object_subtype! {
    /// `SubDeformer` node handle (blendshapechannel).
    BlendShapeChannelHandle: SubDeformerHandle
}

impl<'a> BlendShapeChannelHandle<'a> {
    /// Returns the parant deformer blendshape.
    pub fn blendshape(&self) -> Result<deformer::BlendShapeHandle<'a>, Error> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::BlendShape(o)) => {
                    Some(o)
                }
                _ => None,
            })
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Subdeformer blendshapechannel object should have a \
                     parent deformer blendshape: object={:?}",
                    self
                )
            })
    }

    /// Returns an iterator of child geometry shapes.
    pub fn shapes(&self) -> impl Iterator<Item = geometry::ShapeHandle<'a>> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Geometry(geometry::TypedGeometryHandle::Shape(o)) => Some(o),
                _ => None,
            })
    }
}
