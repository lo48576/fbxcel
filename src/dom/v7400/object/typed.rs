//! Node types.

use crate::dom::v7400::object::{geometry, material, model, ObjectHandle};

/// Typed object handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedObjectHandle<'a> {
    /// Geometry.
    Geometry(geometry::TypedGeometryHandle<'a>),
    /// Material.
    Material(material::MaterialHandle<'a>),
    /// Model.
    Model(model::TypedModelHandle<'a>),
    /// Unknown.
    Unknown(ObjectHandle<'a>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'a> TypedObjectHandle<'a> {
    /// Creates a new handle from the given object handle.
    pub(crate) fn new(obj: ObjectHandle<'a>) -> Self {
        match obj.node().name() {
            "Geometry" => TypedObjectHandle::Geometry(geometry::TypedGeometryHandle::new(
                geometry::GeometryHandle::new(obj),
            )),
            "Material" => TypedObjectHandle::Material(material::MaterialHandle::new(obj)),
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
            TypedObjectHandle::Geometry(o) => &**o,
            TypedObjectHandle::Material(o) => &**o,
            TypedObjectHandle::Model(o) => &**o,
            TypedObjectHandle::Unknown(o) => o,
            TypedObjectHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}
