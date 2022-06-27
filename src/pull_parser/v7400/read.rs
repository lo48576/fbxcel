//! Reader functions and traits.

use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::pull_parser::{v7400::Parser, ParserSource, Result};

/// A trait for types readable from a reader.
pub(crate) trait FromReader: Sized {
    /// Reads the data from the given reader.
    fn from_reader(reader: &mut impl io::Read) -> Result<Self>;
}

impl FromReader for u8 {
    #[inline]
    fn from_reader(reader: &mut impl io::Read) -> Result<Self> {
        Ok(reader.read_u8()?)
    }
}

/// Implement `FromReader` trait for primitive types.
macro_rules! impl_from_reader_for_primitives {
    ($ty:ty, $read_method:ident) => {
        impl FromReader for $ty {
            #[inline]
            fn from_reader(reader: &mut impl io::Read) -> Result<Self> {
                Ok(reader.$read_method::<LittleEndian>()?)
            }
        }
    };
}

impl_from_reader_for_primitives! { u16, read_u16 }
impl_from_reader_for_primitives! { u32, read_u32 }
impl_from_reader_for_primitives! { u64, read_u64 }
impl_from_reader_for_primitives! { i16, read_i16 }
impl_from_reader_for_primitives! { i32, read_i32 }
impl_from_reader_for_primitives! { i64, read_i64 }
impl_from_reader_for_primitives! { f32, read_f32 }
impl_from_reader_for_primitives! { f64, read_f64 }

/// A trait for types readable from a parser.
pub(crate) trait FromParser: Sized {
    /// Reads the data from the given parser.
    fn read_from_parser<R: ParserSource>(parser: &mut Parser<R>) -> Result<Self>;
}

impl<T: FromReader> FromParser for T {
    #[inline]
    fn read_from_parser<R: ParserSource>(parser: &mut Parser<R>) -> Result<Self> {
        FromReader::from_reader(parser.reader())
    }
}
