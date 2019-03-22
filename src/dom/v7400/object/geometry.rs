//! `Geometry` object.

use crate::dom::v7400::object::ObjectHandle;

pub use self::mesh::MeshHandle;

mod mesh;

define_typed_handle! {
    /// Typed geometry handle.
    TypedGeometryHandle(GeometryHandle) {
        /// Mesh.
        ("Geometry", "Mesh") => Mesh(MeshHandle),
    }
}

define_object_subtype! {
    /// `Geometry` node handle.
    GeometryHandle: ObjectHandle
}
