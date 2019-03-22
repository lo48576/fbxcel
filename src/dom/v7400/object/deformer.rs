//! `Deformer` object.

use crate::dom::v7400::object::ObjectHandle;

pub use self::{blendshape::BlendShapeHandle, skin::SkinHandle};

mod blendshape;
mod skin;

define_typed_handle! {
    /// Typed deformer handle.
    TypedDeformerHandle(DeformerHandle) {
        /// BlendShape.
        ("Deformer", "BlendShape") => BlendShape(BlendShapeHandle),
        /// Skin.
        ("Deformer", "Skin") => Skin(SkinHandle),
    }
}

define_object_subtype! {
    /// `Deformer` node handle.
    DeformerHandle: ObjectHandle
}
