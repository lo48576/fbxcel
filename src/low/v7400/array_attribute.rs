//! Low-level data types related to array type node attributes.

use crate::pull_parser::error::{Compression, DataError};
use crate::pull_parser::Error as ParserError;
use crate::pull_parser::{ParserSource, ParserSourceExt};

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

/// A header type for array-type attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArrayAttributeHeader {
    /// Number of elements.
    elements_count: u32,
    /// Encoding.
    encoding: ArrayAttributeEncoding,
    /// Elements length in bytes.
    bytelen: u32,
}

impl ArrayAttributeHeader {
    /// Reads and returns the array-type attribute header.
    pub(crate) fn from_reader<R>(mut reader: R) -> Result<Self, ParserError>
    where
        R: ParserSource,
    {
        let elements_count = reader.read_u32()?;
        let raw_encoding = reader.read_u32()?;
        let encoding = ArrayAttributeEncoding::from_u32(raw_encoding)
            .ok_or_else(|| DataError::InvalidArrayAttributeEncoding(raw_encoding))?;
        let bytelen = reader.read_u32()?;

        Ok(Self {
            elements_count,
            encoding,
            bytelen,
        })
    }

    /// Returns number of elements.
    pub fn elements_count(&self) -> u32 {
        self.elements_count
    }

    /// Returns array encoding.
    pub fn encoding(&self) -> ArrayAttributeEncoding {
        self.encoding
    }

    /// Returns content array length in bytes.
    ///
    /// This length does not include the header size.
    pub fn bytelen(&self) -> u32 {
        self.bytelen
    }
}
