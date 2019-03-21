//! `Model` object.

use crate::dom::v7400::object::ObjectHandle;

pub use self::{camera::CameraHandle, light::LightHandle, mesh::MeshHandle};

mod camera;
mod light;
mod mesh;

/// Typed model handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedModelHandle<'a> {
    /// Camera.
    Camera(CameraHandle<'a>),
    /// Light.
    Light(LightHandle<'a>),
    /// Mesh.
    Mesh(MeshHandle<'a>),
    /// Unoknwn.
    Unknown(ModelHandle<'a>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'a> TypedModelHandle<'a> {
    /// Creates a new handle from the given object handle.
    pub(crate) fn new(obj: ModelHandle<'a>) -> Self {
        match obj.subclass() {
            "Camera" => TypedModelHandle::Camera(CameraHandle::new(obj)),
            "Light" => TypedModelHandle::Light(LightHandle::new(obj)),
            "Mesh" => TypedModelHandle::Mesh(MeshHandle::new(obj)),
            _ => TypedModelHandle::Unknown(obj),
        }
    }
}

impl<'a> std::ops::Deref for TypedModelHandle<'a> {
    type Target = ModelHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedModelHandle::Camera(o) => &**o,
            TypedModelHandle::Light(o) => &**o,
            TypedModelHandle::Mesh(o) => &**o,
            TypedModelHandle::Unknown(o) => o,
            TypedModelHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

define_object_subtype! {
    /// `Model` node handle.
    ModelHandle: ObjectHandle
}
