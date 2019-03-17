//! Low-level data types for binary and stirng type node attributes.

use std::io;

use crate::pull_parser::{v7400::FromReader, Error as ParserError};

/// A header type for array-type attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct SpecialAttributeHeader {
    /// Elements length in bytes.
    pub(crate) bytelen: u32,
}

impl FromReader for SpecialAttributeHeader {
    fn from_reader(reader: &mut impl io::Read) -> Result<Self, ParserError> {
        let bytelen = u32::from_reader(reader)?;

        Ok(Self { bytelen })
    }
}
