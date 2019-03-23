//! `Texture` object.

use crate::v7400::object::{video, ObjectHandle, TypedObjectHandle};

define_object_subtype! {
    /// `Texture` node handle.
    TextureHandle: ObjectHandle
}

impl<'a> TextureHandle<'a> {
    /// Returns a video clip object if available.
    pub fn video_clip(&self) -> Option<video::ClipHandle<'a>> {
        self.destination_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Video(video::TypedVideoHandle::Clip(o)) => Some(o),
                _ => None,
            })
            .next()
    }
}
