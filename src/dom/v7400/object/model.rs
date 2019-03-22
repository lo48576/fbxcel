//! `Model` object.

use crate::dom::v7400::object::ObjectHandle;

pub use self::{camera::CameraHandle, light::LightHandle, mesh::MeshHandle};

mod camera;
mod light;
mod mesh;

define_typed_handle! {
    /// Typed model handle.
    TypedModelHandle(ModelHandle) {
        /// Camera.
        ("Model", "Camera") => Camera(CameraHandle),
        /// Light.
        ("Model", "Light") => Light(LightHandle),
        /// Mesh.
        ("Model", "Mesh") => Mesh(MeshHandle),
    }
}

define_object_subtype! {
    /// `Model` node handle.
    ModelHandle: ObjectHandle
}
