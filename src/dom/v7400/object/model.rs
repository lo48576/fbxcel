//! `Model` object.

use crate::dom::v7400::object::ObjectHandle;

pub use self::{
    camera::CameraHandle, light::LightHandle, limbnode::LimbNodeHandle, mesh::MeshHandle,
    null::NullHandle,
};

mod camera;
mod light;
mod limbnode;
mod mesh;
mod null;

define_typed_handle! {
    /// Typed model handle.
    TypedModelHandle(ModelHandle) {
        /// Camera.
        ("Model", "Camera") => Camera(CameraHandle),
        /// Light.
        ("Model", "Light") => Light(LightHandle),
        /// LimbNode.
        ("Model", "LimbNode") => LimbNode(LimbNodeHandle),
        /// Mesh.
        ("Model", "Mesh") => Mesh(MeshHandle),
        /// Null.
        ("Model", "Null") => Null(NullHandle),
    }
}

define_object_subtype! {
    /// `Model` node handle.
    ModelHandle: ObjectHandle
}
