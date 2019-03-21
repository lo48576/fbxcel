//! `Geometry` object.

use crate::dom::v7400::object::ObjectHandle;

/// Typed geometry handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedGeometryHandle<'a> {
    /// Unoknwn.
    Unknown(GeometryHandle<'a>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'a> TypedGeometryHandle<'a> {
    /// Creates a new handle from the given object handle.
    pub(crate) fn new(obj: GeometryHandle<'a>) -> Self {
        match obj.subclass() {
            _ => TypedGeometryHandle::Unknown(obj),
        }
    }
}

impl<'a> std::ops::Deref for TypedGeometryHandle<'a> {
    type Target = GeometryHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedGeometryHandle::Unknown(o) => o,
            TypedGeometryHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}

define_object_subtype! {
    /// `Geometry` node handle.
    GeometryHandle: ObjectHandle
}
