//! Low-level data types related to array type node attributes.

use crate::pull_parser::error::Compression;

/// Array attribute encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArrayAttributeEncoding {
    /// Direct values.
    Direct,
    /// Zlib compression.
    ///
    /// Zlib compression with header.
    Zlib,
}

impl ArrayAttributeEncoding {
    /// Creates a new `ArrayEncoding` from the given raw value.
    pub(crate) fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(ArrayAttributeEncoding::Direct),
            1 => Some(ArrayAttributeEncoding::Zlib),
            _ => None,
        }
    }
}

impl From<ArrayAttributeEncoding> for Compression {
    // Panics if the encoding is `Direct` (i.e. not compressed).
    fn from(v: ArrayAttributeEncoding) -> Self {
        match v {
            ArrayAttributeEncoding::Direct => unreachable!(
                "Data with `ArrayEncoding::Direct` should not cause (de)compression error"
            ),
            ArrayAttributeEncoding::Zlib => Compression::Zlib,
        }
    }
}
