//! Low-level or primitive data types for FBX 7.4 and compatible versions.

pub(crate) use self::array_attribute::{ArrayAttributeEncoding, ArrayAttributeHeader};
pub use self::attribute_type::AttributeType;
pub use self::fbx_footer::FbxFooter;
pub(crate) use self::node_header::NodeHeader;
pub(crate) use self::special_attribute::SpecialAttributeHeader;

mod array_attribute;
mod attribute_type;
mod fbx_footer;
mod node_header;
mod special_attribute;
