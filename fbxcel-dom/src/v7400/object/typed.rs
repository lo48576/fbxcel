//! Node types.

use crate::v7400::object::{
    deformer, geometry, material, model, nodeattribute, texture, video, ObjectHandle,
};

/// Typed object handle.
#[derive(Debug, Clone, Copy)]
pub enum TypedObjectHandle<'a> {
    /// Deformer.
    Deformer(deformer::TypedDeformerHandle<'a>),
    /// Geometry.
    Geometry(geometry::TypedGeometryHandle<'a>),
    /// Material.
    Material(material::MaterialHandle<'a>),
    /// Model.
    Model(model::TypedModelHandle<'a>),
    /// NodeAttribute.
    NodeAttribute(nodeattribute::TypedNodeAttributeHandle<'a>),
    /// SubDeformer.
    SubDeformer(deformer::TypedSubDeformerHandle<'a>),
    /// Texture.
    Texture(texture::TextureHandle<'a>),
    /// Model.
    Video(video::TypedVideoHandle<'a>),
    /// Unknown.
    Unknown(ObjectHandle<'a>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl<'a> TypedObjectHandle<'a> {
    /// Creates a new handle from the given object handle.
    pub(crate) fn new(obj: ObjectHandle<'a>) -> Self {
        match obj.node().name() {
            "Deformer" => match obj.class() {
                "Deformer" => TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::new(
                    deformer::DeformerHandle::new(obj),
                )),
                "SubDeformer" => TypedObjectHandle::SubDeformer(
                    deformer::TypedSubDeformerHandle::new(deformer::SubDeformerHandle::new(obj)),
                ),
                _ => TypedObjectHandle::Unknown(obj),
            },
            "Geometry" => TypedObjectHandle::Geometry(geometry::TypedGeometryHandle::new(
                geometry::GeometryHandle::new(obj),
            )),
            "Material" => TypedObjectHandle::Material(material::MaterialHandle::new(obj)),
            "Model" => {
                TypedObjectHandle::Model(model::TypedModelHandle::new(model::ModelHandle::new(obj)))
            }
            "NodeAttribute" => {
                TypedObjectHandle::NodeAttribute(nodeattribute::TypedNodeAttributeHandle::new(
                    nodeattribute::NodeAttributeHandle::new(obj),
                ))
            }
            "Texture" => TypedObjectHandle::Texture(texture::TextureHandle::new(obj)),
            "Video" => {
                TypedObjectHandle::Video(video::TypedVideoHandle::new(video::VideoHandle::new(obj)))
            }
            _ => TypedObjectHandle::Unknown(obj),
        }
    }
}

impl<'a> std::ops::Deref for TypedObjectHandle<'a> {
    type Target = ObjectHandle<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            TypedObjectHandle::Deformer(o) => &**o,
            TypedObjectHandle::Geometry(o) => &**o,
            TypedObjectHandle::Material(o) => &**o,
            TypedObjectHandle::Model(o) => &**o,
            TypedObjectHandle::NodeAttribute(o) => &**o,
            TypedObjectHandle::SubDeformer(o) => &**o,
            TypedObjectHandle::Texture(o) => &**o,
            TypedObjectHandle::Video(o) => &**o,
            TypedObjectHandle::Unknown(o) => o,
            TypedObjectHandle::__Nonexhaustive => panic!("`__Nonexhaustive` should not be used"),
        }
    }
}
