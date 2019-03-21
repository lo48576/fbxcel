//! `Model` object.

use crate::dom::v7400::object::ObjectHandle;

pub use self::mesh::MeshHandle;

mod mesh;

/// Typed model handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedModelHandle<'a> {
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
            "Mesh" => TypedModelHandle::Mesh(MeshHandle::new(obj)),
            _ => TypedModelHandle::Unknown(obj),
        }
    }
}

impl<'a> std::ops::Deref for TypedModelHandle<'a> {
    type Target = ModelHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedModelHandle::Mesh(o) => &**o,
            TypedModelHandle::Unknown(o) => o,
            TypedModelHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

/// `Model` node handle.
#[derive(Debug, Clone, Copy)]
pub struct ModelHandle<'a> {
    /// Object handle.
    object: ObjectHandle<'a>,
}

impl<'a> ModelHandle<'a> {
    /// Creates a new handle.
    pub(crate) fn new(object: ObjectHandle<'a>) -> Self {
        Self { object }
    }
}

impl<'a> std::ops::Deref for ModelHandle<'a> {
    type Target = ObjectHandle<'a>;

    fn deref(&self) -> &Self::Target {
        &self.object
    }
}
