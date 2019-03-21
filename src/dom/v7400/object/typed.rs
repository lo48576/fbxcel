//! Node types.

use crate::dom::v7400::object::ObjectHandle;

/// Typed object handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedObjectHandle<'a> {
    /// Unoknwn.
    Unknown(ObjectHandle<'a>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'a> TypedObjectHandle<'a> {
    /// Creates a new handle from the given object handle.
    pub(crate) fn new(obj: ObjectHandle<'a>) -> Self {
        match obj.node().name() {
            _ => TypedObjectHandle::Unknown(obj),
        }
    }
}

impl<'a> std::ops::Deref for TypedObjectHandle<'a> {
    type Target = ObjectHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedObjectHandle::Unknown(o) => o,
            TypedObjectHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}
