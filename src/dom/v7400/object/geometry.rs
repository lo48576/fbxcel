//! `Geometry` object.

use crate::dom::v7400::object::ObjectHandle;

define_typed_handle! {
    /// Typed geometry handle.
    TypedGeometryHandle(GeometryHandle) {
    }
}

define_object_subtype! {
    /// `Geometry` node handle.
    GeometryHandle: ObjectHandle
}
