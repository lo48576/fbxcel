//! Array-type node attributes.

use std::io;
use std::marker::PhantomData;

use byteorder::LittleEndian;
use libflate::zlib::Decoder as ZlibDecoder;

use crate::low::v7400::ArrayAttributeEncoding;

use super::super::error::DataError;
use super::super::{ParserSource, ParserSourceExt, Result};

/// A header type for array-type attributes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ArrayHeader {
    /// Number of elements.
    elements_count: u32,
    /// Encoding.
    encoding: ArrayAttributeEncoding,
    /// Elements length in bytes.
    bytelen: u32,
}

impl ArrayHeader {
    /// Reads and returns the array-type attribute header.
    pub(crate) fn from_reader<R>(mut reader: R) -> Result<Self>
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

/// Attribute stream decoder.
// `io::BufRead` is not implemented for `ZlibDecoder`.
#[derive(Debug)]
pub enum AttributeStreamDecoder<R> {
    /// Direct stream.
    Direct(R),
    /// Zlib-decoded stream.
    Zlib(ZlibDecoder<R>),
}

impl<R: io::Read> AttributeStreamDecoder<R> {
    /// Creates a new decoded reader.
    pub fn create(encoding: ArrayAttributeEncoding, reader: R) -> Result<Self> {
        match encoding {
            ArrayAttributeEncoding::Direct => Ok(AttributeStreamDecoder::Direct(reader)),
            ArrayAttributeEncoding::Zlib => Ok(AttributeStreamDecoder::Zlib(
                ZlibDecoder::new(reader)
                    .map_err(|e| DataError::BrokenCompression(encoding.into(), e.into()))?,
            )),
        }
    }
}

impl<R: io::Read> io::Read for AttributeStreamDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            AttributeStreamDecoder::Direct(reader) => reader.read(buf),
            AttributeStreamDecoder::Zlib(reader) => reader.read(buf),
        }
    }
}

/// Array attribute values iterator for `{i,f}{32,64}` array.
#[derive(Debug, Clone, Copy)]
pub struct ArrayAttributeValues<R, E> {
    /// Decoded reader.
    reader: R,
    /// Number of total elements.
    total_elements: u32,
    /// Number of rest elements.
    rest_elements: u32,
    /// Element type.
    _element_type: PhantomData<E>,
}

impl<R, E> ArrayAttributeValues<R, E>
where
    R: io::Read,
{
    /// Creates a new `ArrayAttributeValues`.
    pub(crate) fn new(reader: R, total_elements: u32) -> Self {
        Self {
            reader,
            total_elements,
            rest_elements: total_elements,
            _element_type: PhantomData,
        }
    }
}

macro_rules! impl_array_attr_values {
    ($ty_elem:ty, $read_elem:ident) => {
        impl<R: io::Read> Iterator for ArrayAttributeValues<R, $ty_elem> {
            type Item = Result<$ty_elem>;

            fn next(&mut self) -> Option<Self::Item> {
                use byteorder::ReadBytesExt;

                if self.rest_elements == 0 {
                    return None;
                }
                match self.reader.$read_elem::<LittleEndian>() {
                    Ok(v) => {
                        self.rest_elements = self
                            .rest_elements
                            .checked_sub(1)
                            .expect("This should be executed only when there are rest elements");
                        Some(Ok(v))
                    }
                    Err(e) => Some(Err(e.into())),
                }
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, Some(self.rest_elements as usize))
            }
        }
    };
}

impl_array_attr_values! { i32, read_i32 }
impl_array_attr_values! { i64, read_i64 }
impl_array_attr_values! { f32, read_f32 }
impl_array_attr_values! { f64, read_f64 }

/// Array attribute values iterator for `bool` array.
#[derive(Debug, Clone, Copy)]
pub struct BooleanArrayAttributeValues<R> {
    /// Decoded reader.
    reader: R,
    /// Number of total elements.
    total_elements: u32,
    /// Number of rest elements.
    rest_elements: u32,
    /// Whether the attribute has incorrect boolean value representation.
    has_incorrect_boolean_value: bool,
}

impl<R: io::Read> BooleanArrayAttributeValues<R> {
    /// Creates a new `BooleanArrayAttributeValues`.
    pub(crate) fn new(reader: R, total_elements: u32) -> Self {
        Self {
            reader,
            total_elements,
            rest_elements: total_elements,
            has_incorrect_boolean_value: false,
        }
    }

    /// Returns whether the attribute has incorrect boolean value
    /// representation.
    // Allow `dead_code` because this will be used when the warning feature is
    // implemented.
    #[allow(dead_code)]
    pub(crate) fn has_incorrect_boolean_value(&self) -> bool {
        self.has_incorrect_boolean_value
    }
}

impl<R: io::Read> Iterator for BooleanArrayAttributeValues<R> {
    type Item = Result<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        use byteorder::ReadBytesExt;

        if self.rest_elements == 0 {
            return None;
        }
        match self.reader.read_u8() {
            Ok(raw) => {
                self.rest_elements = self
                    .rest_elements
                    .checked_sub(1)
                    .expect("This should be executed only when there are rest elements");
                if raw != b'T' && raw != b'Y' {
                    self.has_incorrect_boolean_value = true;
                }
                let v = (raw & 1) != 0;
                Some(Ok(v))
            }
            Err(e) => Some(Err(e.into())),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.rest_elements as usize))
    }
}
