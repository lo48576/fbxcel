//! `Video` object.

use crate::v7400::object::ObjectHandle;

pub use self::clip::ClipHandle;

mod clip;

define_typed_handle! {
    /// Typed video handle.
    TypedVideoHandle(VideoHandle) {
        /// Clip.
        ("Video", "Clip") => Clip(ClipHandle),
    }
}

define_object_subtype! {
    /// `Video` node handle.
    VideoHandle: ObjectHandle
}
