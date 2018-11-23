//! Low-level data types for binary and stirng type node attributes.

use std::io;

use crate::pull_parser::v7400::FromReader;
use crate::pull_parser::Error as ParserError;

/// A header type for array-type attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpecialAttributeHeader {
    /// Elements length in bytes.
    bytelen: u32,
}

impl SpecialAttributeHeader {
    /// Returns content array length in bytes.
    ///
    /// This length does not include the header size.
    pub fn bytelen(self) -> u32 {
        self.bytelen
    }
}

impl FromReader for SpecialAttributeHeader {
    fn from_reader(reader: &mut impl io::Read) -> Result<Self, ParserError> {
        let bytelen = u32::from_reader(reader)?;

        Ok(Self { bytelen })
    }
}
