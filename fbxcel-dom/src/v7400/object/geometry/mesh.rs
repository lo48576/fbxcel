//! `Geometry` object (mesh).

use crate::v7400::object::{deformer, geometry::GeometryHandle, model, TypedObjectHandle};

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

    /// Returns a child deformer skin if available.
    pub fn skins(&self) -> impl Iterator<Item = deformer::SkinHandle<'a>> {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::Skin(o)) => Some(o),
                _ => None,
            })
    }

    /// Returns a child deformer blendshapes if available.
    pub fn blendshapes(&self) -> impl Iterator<Item = deformer::BlendShapeHandle<'a>> {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::BlendShape(o)) => {
                    Some(o)
                }
                _ => None,
            })
    }
}
