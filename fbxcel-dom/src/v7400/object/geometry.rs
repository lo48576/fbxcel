//! `Geometry` object.

use crate::v7400::object::ObjectHandle;

pub use self::{mesh::MeshHandle, shape::ShapeHandle};

mod mesh;
mod shape;

define_typed_handle! {
    /// Typed geometry handle.
    TypedGeometryHandle(GeometryHandle) {
        /// Mesh.
        ("Geometry", "Mesh") => Mesh(MeshHandle),
        /// Shape.
        ("Geometry", "Shape") => Shape(ShapeHandle),
    }
}

define_object_subtype! {
    /// `Geometry` node handle.
    GeometryHandle: ObjectHandle
}
