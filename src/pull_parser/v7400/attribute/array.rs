//! Array-type node attributes.

use std::{io, marker::PhantomData};

use byteorder::LittleEndian;
use libflate::zlib::Decoder as ZlibDecoder;

use crate::{
    low::v7400::ArrayAttributeEncoding,
    pull_parser::{error::DataError, Result},
};

/// Attribute stream decoder.
// `io::BufRead` is not implemented for `ZlibDecoder`.
#[derive(Debug)]
pub(crate) enum AttributeStreamDecoder<R> {
    /// Direct stream.
    Direct(R),
    /// Zlib-decoded stream.
    Zlib(ZlibDecoder<R>),
}

impl<R: io::Read> AttributeStreamDecoder<R> {
    /// Creates a new decoded reader.
    pub(crate) fn create(encoding: ArrayAttributeEncoding, reader: R) -> Result<Self> {
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
pub(crate) struct ArrayAttributeValues<R, E> {
    /// Decoded reader.
    reader: R,
    // `total_elements`: unused.
    ///// Number of total elements.
    //total_elements: u32,
    /// Number of rest elements.
    rest_elements: u32,
    /// Whether an error is happened.
    has_error: bool,
    /// Element type.
    _element_type: PhantomData<E>,
}

impl<R, E> ArrayAttributeValues<R, E>
where
    R: io::Read,
{
    /// Creates a new `ArrayAttributeValues`.
    #[inline]
    #[must_use]
    pub(crate) fn new(reader: R, total_elements: u32) -> Self {
        Self {
            reader,
            //total_elements,
            rest_elements: total_elements,
            has_error: false,
            _element_type: PhantomData,
        }
    }

    /// Returns whether an error happened or not.
    #[inline]
    #[must_use]
    pub(crate) fn has_error(&self) -> bool {
        self.has_error
    }
}

/// Implement common traits for `ArrayAttributeValues`.
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
                    Err(e) => {
                        self.has_error = true;
                        Some(Err(e.into()))
                    }
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, Some(self.rest_elements as usize))
            }
        }

        impl<R: io::Read> std::iter::FusedIterator for ArrayAttributeValues<R, $ty_elem> {}
    };
}

impl_array_attr_values! { i32, read_i32 }
impl_array_attr_values! { i64, read_i64 }
impl_array_attr_values! { f32, read_f32 }
impl_array_attr_values! { f64, read_f64 }

/// Array attribute values iterator for `bool` array.
#[derive(Debug, Clone, Copy)]
pub(crate) struct BooleanArrayAttributeValues<R> {
    /// Decoded reader.
    reader: R,
    // `total_elements`: unused.
    ///// Number of total elements.
    //total_elements: u32,
    /// Number of rest elements.
    rest_elements: u32,
    /// Whether an error is happened.
    has_error: bool,
    /// Whether the attribute has incorrect boolean value representation.
    has_incorrect_boolean_value: bool,
}

impl<R: io::Read> BooleanArrayAttributeValues<R> {
    /// Creates a new `BooleanArrayAttributeValues`.
    #[inline]
    #[must_use]
    pub(crate) fn new(reader: R, total_elements: u32) -> Self {
        Self {
            reader,
            //total_elements,
            rest_elements: total_elements,
            has_error: false,
            has_incorrect_boolean_value: false,
        }
    }

    /// Returns whether the attribute has incorrect boolean value
    /// representation.
    #[inline]
    #[must_use]
    pub(crate) fn has_incorrect_boolean_value(&self) -> bool {
        self.has_incorrect_boolean_value
    }

    /// Returns whether an error happened or not.
    #[inline]
    #[must_use]
    pub(crate) fn has_error(&self) -> bool {
        self.has_error
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
            Err(e) => {
                self.has_error = true;
                Some(Err(e.into()))
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.rest_elements as usize))
    }
}

impl<R: io::Read> std::iter::FusedIterator for BooleanArrayAttributeValues<R> {}
