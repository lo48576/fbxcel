//! Reader functions and traits.

use std::io;

use crate::pull_parser::{v7400::Parser, ParserSource, Result};

/// A trait for types readable from a reader.
pub(crate) trait FromReader: Sized {
    /// Reads the data from the given reader.
    fn from_reader(reader: &mut impl io::Read) -> Result<Self>;
}

impl FromReader for u8 {
    #[inline]
    fn from_reader(reader: &mut impl io::Read) -> Result<Self> {
        let mut buf = 0_u8;
        reader.read_exact(std::slice::from_mut(&mut buf))?;
        Ok(buf)
    }
}

/// Implement `FromReader` trait for primitive types.
macro_rules! impl_from_reader_for_primitives {
    ($ty:ty) => {
        impl FromReader for $ty {
            #[inline]
            fn from_reader(reader: &mut impl io::Read) -> Result<Self> {
                let mut buf = [0_u8; std::mem::size_of::<$ty>()];
                reader.read_exact(&mut buf)?;
                Ok(<$ty>::from_le_bytes(buf))
            }
        }
    };
}

impl_from_reader_for_primitives! { u16 }
impl_from_reader_for_primitives! { u32 }
impl_from_reader_for_primitives! { u64 }
impl_from_reader_for_primitives! { i16 }
impl_from_reader_for_primitives! { i32 }
impl_from_reader_for_primitives! { i64 }
impl_from_reader_for_primitives! { f32 }
impl_from_reader_for_primitives! { f64 }

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
