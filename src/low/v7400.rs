//! Low-level or primitive data types for FBX 7.4 and compatible versions.

pub(crate) use self::{
    array_attribute::{ArrayAttributeEncoding, ArrayAttributeHeader},
    node_header::NodeHeader,
    special_attribute::SpecialAttributeHeader,
};
pub use self::{attribute_type::AttributeType, fbx_footer::FbxFooter};

mod array_attribute;
mod attribute_type;
mod fbx_footer;
mod node_header;
mod special_attribute;
