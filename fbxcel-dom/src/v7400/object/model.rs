//! `Model` object.

use crate::v7400::object::{ObjectHandle, TypedObjectHandle};

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

impl<'a> ModelHandle<'a> {
    /// Returns the parent model if available.
    pub fn parent_model(&self) -> Option<TypedModelHandle<'a>> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(o) => Some(o),
                _ => None,
            })
            .next()
    }

    /// Returns an iterator of the child models.
    pub fn child_models(&self) -> impl Iterator<Item = TypedModelHandle<'a>> + 'a {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(o) => Some(o),
                _ => None,
            })
    }
}
