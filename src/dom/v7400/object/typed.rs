//! Node types.

use crate::dom::v7400::object::{model, ObjectHandle};

/// Typed object handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedObjectHandle<'a> {
    /// Model.
    Model(model::TypedModelHandle<'a>),
    /// Unoknwn.
    Unknown(ObjectHandle<'a>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'a> TypedObjectHandle<'a> {
    /// Creates a new handle from the given object handle.
    pub(crate) fn new(obj: ObjectHandle<'a>) -> Self {
        match obj.node().name() {
            "Model" => {
                TypedObjectHandle::Model(model::TypedModelHandle::new(model::ModelHandle::new(obj)))
            }
            _ => TypedObjectHandle::Unknown(obj),
        }
    }
}

impl<'a> std::ops::Deref for TypedObjectHandle<'a> {
    type Target = ObjectHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedObjectHandle::Model(o) => &**o,
            TypedObjectHandle::Unknown(o) => o,
            TypedObjectHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}
