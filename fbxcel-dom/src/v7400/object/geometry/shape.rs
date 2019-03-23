//! `Geometry` object (shape).

use failure::{format_err, Error};

use crate::v7400::object::{deformer, geometry::GeometryHandle, TypedObjectHandle};

define_object_subtype! {
    /// `Geometry` node handle (shape).
    ShapeHandle: GeometryHandle
}

impl<'a> ShapeHandle<'a> {
    /// Returns the parant subdeformer blendshapechannel.
    pub fn blendshape_channel(&self) -> Result<deformer::BlendShapeChannelHandle<'a>, Error> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::SubDeformer(
                    deformer::TypedSubDeformerHandle::BlendShapeChannel(o),
                ) => Some(o),
                _ => None,
            })
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Geometry shape object should have a \
                     parent subdeformer blendshapechannel: object={:?}",
                    self
                )
            })
    }
}
