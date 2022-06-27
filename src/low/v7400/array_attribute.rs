//! Low-level data types related to array type node attributes.

use std::io;

use crate::pull_parser::{
    error::{Compression, DataError},
    v7400::FromReader,
    Error as ParserError,
};

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
    #[inline]
    #[must_use]
    pub(crate) fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(ArrayAttributeEncoding::Direct),
            1 => Some(ArrayAttributeEncoding::Zlib),
            _ => None,
        }
    }

    /// Returns the raw value.
    #[cfg(feature = "writer")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "writer")))]
    #[inline]
    #[must_use]
    pub(crate) fn to_u32(self) -> u32 {
        match self {
            ArrayAttributeEncoding::Direct => 0,
            ArrayAttributeEncoding::Zlib => 1,
        }
    }
}

impl From<ArrayAttributeEncoding> for Compression {
    // Panics if the encoding is `Direct` (i.e. not compressed).
    #[inline]
    fn from(v: ArrayAttributeEncoding) -> Self {
        match v {
            ArrayAttributeEncoding::Direct => unreachable!(
                "Data with `ArrayEncoding::Direct` should not cause (de)compression error"
            ),
            ArrayAttributeEncoding::Zlib => Compression::Zlib,
        }
    }
}

impl FromReader for ArrayAttributeEncoding {
    fn from_reader(reader: &mut impl io::Read) -> Result<Self, ParserError> {
        let raw_encoding = u32::from_reader(reader)?;
        let encoding = ArrayAttributeEncoding::from_u32(raw_encoding)
            .ok_or(DataError::InvalidArrayAttributeEncoding(raw_encoding))?;
        Ok(encoding)
    }
}

/// A header type for array-type attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ArrayAttributeHeader {
    /// Number of elements.
    pub(crate) elements_count: u32,
    /// Encoding.
    pub(crate) encoding: ArrayAttributeEncoding,
    /// Elements length in bytes.
    pub(crate) bytelen: u32,
}

impl FromReader for ArrayAttributeHeader {
    fn from_reader(reader: &mut impl io::Read) -> Result<Self, ParserError> {
        let elements_count = u32::from_reader(reader)?;
        let encoding = ArrayAttributeEncoding::from_reader(reader)?;
        let bytelen = u32::from_reader(reader)?;

        Ok(Self {
            elements_count,
            encoding,
            bytelen,
        })
    }
}
