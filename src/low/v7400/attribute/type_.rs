//! Node attribute type.

use std::io;

use crate::pull_parser::{error::DataError, v7400::FromReader, Error as ParserError};

/// Node attribute type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    /// Single `bool`.
    Bool,
    /// Single `i16`.
    I16,
    /// Single `i32`.
    I32,
    /// Single `i64`.
    I64,
    /// Single `f32`.
    F32,
    /// Single `f64`.
    F64,
    /// Array of `bool`.
    ArrBool,
    /// Array of `i32`.
    ArrI32,
    /// Array of `i64`.
    ArrI64,
    /// Array of `f32`.
    ArrF32,
    /// Array of `f64`.
    ArrF64,
    /// Binary.
    Binary,
    /// UTF-8 string.
    String,
}

impl AttributeType {
    /// Creates an `AttributeType` from the given type code.
    #[must_use]
    pub(crate) fn from_type_code(code: u8) -> Option<Self> {
        match code {
            b'C' => Some(AttributeType::Bool),
            b'Y' => Some(AttributeType::I16),
            b'I' => Some(AttributeType::I32),
            b'L' => Some(AttributeType::I64),
            b'F' => Some(AttributeType::F32),
            b'D' => Some(AttributeType::F64),
            b'b' => Some(AttributeType::ArrBool),
            b'i' => Some(AttributeType::ArrI32),
            b'l' => Some(AttributeType::ArrI64),
            b'f' => Some(AttributeType::ArrF32),
            b'd' => Some(AttributeType::ArrF64),
            b'R' => Some(AttributeType::Binary),
            b'S' => Some(AttributeType::String),
            _ => None,
        }
    }

    /// Returns the type code.
    #[cfg(feature = "writer")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "writer")))]
    #[must_use]
    pub(crate) fn type_code(self) -> u8 {
        match self {
            AttributeType::Bool => b'C',
            AttributeType::I16 => b'Y',
            AttributeType::I32 => b'I',
            AttributeType::I64 => b'L',
            AttributeType::F32 => b'F',
            AttributeType::F64 => b'D',
            AttributeType::ArrBool => b'b',
            AttributeType::ArrI32 => b'i',
            AttributeType::ArrI64 => b'l',
            AttributeType::ArrF32 => b'f',
            AttributeType::ArrF64 => b'd',
            AttributeType::Binary => b'R',
            AttributeType::String => b'S',
        }
    }
}

impl FromReader for AttributeType {
    fn from_reader(reader: &mut impl io::Read) -> Result<Self, ParserError> {
        let type_code = u8::from_reader(reader)?;
        let attr_type = Self::from_type_code(type_code)
            .ok_or(DataError::InvalidAttributeTypeCode(type_code))?;
        Ok(attr_type)
    }
}
