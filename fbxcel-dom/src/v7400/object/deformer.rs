//! `Deformer` and `SubDeformer` object.

use crate::v7400::object::ObjectHandle;

pub use self::{
    blendshape::BlendShapeHandle, blendshapechannel::BlendShapeChannelHandle,
    cluster::ClusterHandle, skin::SkinHandle,
};

mod blendshape;
mod blendshapechannel;
mod cluster;
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

define_typed_handle! {
    /// Typed subdeformer handle.
    TypedSubDeformerHandle(SubDeformerHandle) {
        /// BlendShapeChannel.
        ("SubDeformer", "BlendShapeChannel") => BlendShapeChannel(BlendShapeChannelHandle),
        /// Cluster.
        ("SubDeformer", "Cluster") => Cluster(ClusterHandle),
    }
}

define_object_subtype! {
    /// `SubDeformer` node handle.
    SubDeformerHandle: ObjectHandle
}
