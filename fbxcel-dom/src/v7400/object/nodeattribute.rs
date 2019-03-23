//! `NodeAttribute` object.

use crate::v7400::object::ObjectHandle;

pub use self::{
    camera::CameraHandle, light::LightHandle, limbnode::LimbNodeHandle, null::NullHandle,
};

mod camera;
mod light;
mod limbnode;
mod null;

define_typed_handle! {
    /// Typed node attribute handle.
    TypedNodeAttributeHandle(NodeAttributeHandle) {
        /// Mesh.
        ("NodeAttribute", "Camera") => Camera(CameraHandle),
        /// Light.
        ("NodeAttribute", "Light") => Light(LightHandle),
        /// LimbNode.
        ("NodeAttribute", "LimbNode") => LimbNode(LimbNodeHandle),
        /// Null.
        ("NodeAttribute", "Null") => Null(NullHandle),
    }
}

define_object_subtype! {
    /// `NodeAttribute` node handle.
    NodeAttributeHandle: ObjectHandle
}
