//! `SubDeformer` object (cluster).

use failure::{format_err, Error};

use crate::v7400::object::{
    deformer::{self, SubDeformerHandle},
    TypedObjectHandle,
};

define_object_subtype! {
    /// `SubDeformer` node handle (cluster).
    ClusterHandle: SubDeformerHandle
}

impl<'a> ClusterHandle<'a> {
    /// Returns the parant deformer skin.
    pub fn skin(&self) -> Result<deformer::SkinHandle<'a>, Error> {
        self.source_objects()
            .filter(|obj| obj.label().is_none())
            .filter_map(|obj| obj.object_handle())
            .filter_map(|obj| match obj.get_typed() {
                TypedObjectHandle::Deformer(deformer::TypedDeformerHandle::Skin(o)) => Some(o),
                _ => None,
            })
            .next()
            .ok_or_else(|| {
                format_err!(
                    "Subdeformer cluster object should have a parent deformer skin: object={:?}",
                    self
                )
            })
    }
}
