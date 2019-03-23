//! `Material` object.

use crate::v7400::object::{model, texture, ObjectHandle, TypedObjectHandle};

define_object_subtype! {
    /// `Material` node handle.
    MaterialHandle: ObjectHandle
}

impl<'a> MaterialHandle<'a> {
    /// Returns an iterator of parent model mesh objects.
    pub fn meshes(&self) -> impl Iterator<Item = model::MeshHandle<'a>> + 'a {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Model(model::TypedModelHandle::Mesh(o)) => Some(o),
                _ => None,
            })
    }

    /// Returns a diffuse color texture object if available.
    pub fn diffuse_texture(&self) -> Option<texture::TextureHandle<'a>> {
        get_texture_node(self, "DiffuseColor")
    }

    /// Returns a transparent color texture object if available.
    pub fn transparent_texture(&self) -> Option<texture::TextureHandle<'a>> {
        get_texture_node(self, "TransparentColor")
    }
}

/// Returns a texture object connected with the given label, if available.
fn get_texture_node<'a>(
    obj: &MaterialHandle<'a>,
    label: &str,
) -> Option<texture::TextureHandle<'a>> {
    obj.destination_objects()
        .filter(|obj| obj.label() == Some(label))
        .filter_map(|obj| obj.object_handle())
        .filter_map(|obj| match obj.get_typed() {
            TypedObjectHandle::Texture(o) => Some(o),
            _ => None,
        })
        .next()
}
