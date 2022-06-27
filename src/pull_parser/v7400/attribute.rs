//! Node attributes.

use std::io;

use crate::{
    low::v7400::{ArrayAttributeHeader, AttributeType, SpecialAttributeHeader},
    pull_parser::{
        error::DataError,
        v7400::{FromReader, Parser},
        ParserSource, Result, SyntacticPosition, Warning,
    },
};

use self::array::{ArrayAttributeValues, AttributeStreamDecoder, BooleanArrayAttributeValues};
pub use self::loader::LoadAttribute;

mod array;
pub mod iter;
mod loader;
pub mod loaders;

/// Node attributes reader.
#[derive(Debug)]
pub struct Attributes<'a, R> {
    /// Total number of attributes of the current node.
    total_count: u64,
    /// Rest number of attributes of the current node.
    rest_count: u64,
    /// Beginning offset of the next attribute (if available).
    ///
    /// This is almost same as "end offset of the previous attribute (if
    /// available)".
    next_attr_start_offset: u64,
    /// Parser.
    parser: &'a mut Parser<R>,
}

impl<'a, R: 'a + ParserSource> Attributes<'a, R> {
    /// Creates a new `Attributes`.
    #[must_use]
    pub(crate) fn from_parser(parser: &'a mut Parser<R>) -> Self {
        let total_count = parser.current_attributes_count();
        let pos = parser.reader().position();
        Self {
            total_count,
            rest_count: total_count,
            next_attr_start_offset: pos,
            parser,
        }
    }

    /// Returns the total number of attributes.
    #[inline]
    #[must_use]
    pub fn total_count(&self) -> u64 {
        self.total_count
    }

    /// Returns the number of the rest attributes.
    #[inline]
    #[must_use]
    pub fn rest_count(&self) -> u64 {
        self.rest_count
    }

    /// Updates the next attribute start offset according to the given size (in
    /// bytes).
    fn update_next_attr_start_offset(&mut self, size: u64) {
        self.next_attr_start_offset = self
            .parser
            .reader()
            .position()
            .checked_add(size)
            .expect("FBX data too large");
    }

    /// Runs the given function with the health check and update.
    pub(crate) fn do_with_health_check<T, F>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Self, u64, usize) -> Result<T>,
    {
        self.parser.ensure_continuable()?;

        let start_pos = self.next_attr_start_offset;
        let attr_index = (self.total_count - self.rest_count) as usize;

        match f(self, start_pos, attr_index) {
            Ok(v) => Ok(v),
            Err(e) => {
                let err_pos = self.position(start_pos, attr_index);
                self.parser.set_aborted(err_pos.clone());
                Err(e.and_position(err_pos))
            }
        }
    }

    /// Returns the next attribute type.
    fn read_next_attr_type(&mut self) -> Result<Option<AttributeType>> {
        if self.rest_count() == 0 {
            return Ok(None);
        }

        // Skip the previous attribute value if it remains.
        if self.parser.reader().position() < self.next_attr_start_offset {
            self.parser.reader().skip_to(self.next_attr_start_offset)?;
        }

        let attr_type = self.parser.parse::<AttributeType>()?;

        // This never overflows because `rest_count > 0` holds here.
        // Update this count after parsing is done, so that
        // `total_count - rest_count` is same as attribute index during parsing.
        self.rest_count -= 1;

        Ok(Some(attr_type))
    }

    /// Lets loader load the next node attribute.
    pub fn load_next<V>(&mut self, loader: V) -> Result<Option<V::Output>>
    where
        V: LoadAttribute,
    {
        self.do_with_health_check(|this, start_pos, attr_index| {
            let attr_type = match this.read_next_attr_type()? {
                Some(v) => v,
                None => return Ok(None),
            };
            this.load_next_impl(attr_type, loader, start_pos, attr_index)
                .map(Some)
        })
    }

    /// Lets loader load the next node attribute.
    ///
    /// This method prefers `V::load_{binary,string}_buffered` to
    /// `V::load_{binary,string}`.
    pub fn load_next_buffered<V>(&mut self, loader: V) -> Result<Option<V::Output>>
    where
        R: io::BufRead,
        V: LoadAttribute,
    {
        self.do_with_health_check(|this, start_pos, attr_index| {
            let attr_type = match this.read_next_attr_type()? {
                Some(v) => v,
                None => return Ok(None),
            };
            this.load_next_buffered_impl(attr_type, loader, start_pos, attr_index)
                .map(Some)
        })
    }

    /// Internal implementation of `load_next`.
    fn load_next_impl<V>(
        &mut self,
        attr_type: AttributeType,
        loader: V,
        start_pos: u64,
        attr_index: usize,
    ) -> Result<V::Output>
    where
        V: LoadAttribute,
    {
        match attr_type {
            AttributeType::Bool => {
                let raw = self.parser.parse::<u8>()?;
                let value = (raw & 1) != 0;
                self.update_next_attr_start_offset(0);
                if raw != b'T' && raw != b'Y' {
                    self.parser.warn(
                        Warning::IncorrectBooleanRepresentation,
                        self.position(start_pos, attr_index),
                    )?;
                }
                loader.load_bool(value)
            }
            AttributeType::I16 => {
                let value = self.parser.parse::<i16>()?;
                self.update_next_attr_start_offset(0);
                loader.load_i16(value)
            }
            AttributeType::I32 => {
                let value = self.parser.parse::<i32>()?;
                self.update_next_attr_start_offset(0);
                loader.load_i32(value)
            }
            AttributeType::I64 => {
                let value = self.parser.parse::<i64>()?;
                self.update_next_attr_start_offset(0);
                loader.load_i64(value)
            }
            AttributeType::F32 => {
                let value = self.parser.parse::<f32>()?;
                self.update_next_attr_start_offset(0);
                loader.load_f32(value)
            }
            AttributeType::F64 => {
                let value = self.parser.parse::<f64>()?;
                self.update_next_attr_start_offset(0);
                loader.load_f64(value)
            }
            AttributeType::ArrBool => {
                let header = ArrayAttributeHeader::from_reader(self.parser.reader())?;
                self.update_next_attr_start_offset(u64::from(header.bytelen));
                let reader = AttributeStreamDecoder::create(header.encoding, self.parser.reader())?;
                let count = header.elements_count;
                let mut iter = BooleanArrayAttributeValues::new(reader, count);
                let res = loader.load_seq_bool(&mut iter, count as usize)?;
                // Save `has_error` to make `iter` discardable before
                // `self.parser.warn()` call.
                let has_error = iter.has_error();
                if iter.has_incorrect_boolean_value() {
                    self.parser.warn(
                        Warning::IncorrectBooleanRepresentation,
                        self.position(start_pos, attr_index),
                    )?;
                }
                if has_error {
                    return Err(DataError::NodeAttributeError.into());
                }
                Ok(res)
            }
            AttributeType::ArrI32 => {
                let header = ArrayAttributeHeader::from_reader(self.parser.reader())?;
                self.update_next_attr_start_offset(u64::from(header.bytelen));
                let reader = AttributeStreamDecoder::create(header.encoding, self.parser.reader())?;
                let count = header.elements_count;
                let mut iter = ArrayAttributeValues::<_, i32>::new(reader, count);
                let res = loader.load_seq_i32(&mut iter, count as usize)?;
                if iter.has_error() {
                    return Err(DataError::NodeAttributeError.into());
                }
                Ok(res)
            }
            AttributeType::ArrI64 => {
                let header = ArrayAttributeHeader::from_reader(self.parser.reader())?;
                self.update_next_attr_start_offset(u64::from(header.bytelen));
                let reader = AttributeStreamDecoder::create(header.encoding, self.parser.reader())?;
                let count = header.elements_count;
                let mut iter = ArrayAttributeValues::<_, i64>::new(reader, count);
                let res = loader.load_seq_i64(&mut iter, count as usize)?;
                if iter.has_error() {
                    return Err(DataError::NodeAttributeError.into());
                }
                Ok(res)
            }
            AttributeType::ArrF32 => {
                let header = ArrayAttributeHeader::from_reader(self.parser.reader())?;
                self.update_next_attr_start_offset(u64::from(header.bytelen));
                let reader = AttributeStreamDecoder::create(header.encoding, self.parser.reader())?;
                let count = header.elements_count;
                let mut iter = ArrayAttributeValues::<_, f32>::new(reader, count);
                let res = loader.load_seq_f32(&mut iter, count as usize)?;
                if iter.has_error() {
                    return Err(DataError::NodeAttributeError.into());
                }
                Ok(res)
            }
            AttributeType::ArrF64 => {
                let header = ArrayAttributeHeader::from_reader(self.parser.reader())?;
                self.update_next_attr_start_offset(u64::from(header.bytelen));
                let reader = AttributeStreamDecoder::create(header.encoding, self.parser.reader())?;
                let count = header.elements_count;
                let mut iter = ArrayAttributeValues::<_, f64>::new(reader, count);
                let res = loader.load_seq_f64(&mut iter, count as usize)?;
                if iter.has_error() {
                    return Err(DataError::NodeAttributeError.into());
                }
                Ok(res)
            }
            AttributeType::Binary => {
                let header = self.parser.parse::<SpecialAttributeHeader>()?;
                let bytelen = u64::from(header.bytelen);
                self.update_next_attr_start_offset(bytelen);
                // `self.parser.reader().by_ref().take(bytelen)` is rejected by
                // borrowck (of rustc 1.31.0-beta.15 (4b3a1d911 2018-11-20)).
                let reader = io::Read::take(self.parser.reader(), bytelen);
                loader.load_binary(reader, bytelen)
            }
            AttributeType::String => {
                let header = self.parser.parse::<SpecialAttributeHeader>()?;
                let bytelen = u64::from(header.bytelen);
                self.update_next_attr_start_offset(bytelen);
                // `self.parser.reader().by_ref().take(bytelen)` is rejected by
                // borrowck (of rustc 1.31.0-beta.15 (4b3a1d911 2018-11-20)).
                let reader = io::Read::take(self.parser.reader(), bytelen);
                loader.load_string(reader, bytelen)
            }
        }
    }

    /// Internal implementation of `load_next_buffered`.
    fn load_next_buffered_impl<V>(
        &mut self,
        attr_type: AttributeType,
        loader: V,
        start_pos: u64,
        attr_index: usize,
    ) -> Result<V::Output>
    where
        R: io::BufRead,
        V: LoadAttribute,
    {
        match attr_type {
            AttributeType::Binary => {
                let header = self.parser.parse::<SpecialAttributeHeader>()?;
                let bytelen = u64::from(header.bytelen);
                self.update_next_attr_start_offset(bytelen);
                // `self.parser.reader().by_ref().take(bytelen)` is rejected by
                // borrowck (of rustc 1.31.0-beta.15 (4b3a1d911 2018-11-20)).
                let reader = io::Read::take(self.parser.reader(), bytelen);
                loader.load_binary_buffered(reader, bytelen)
            }
            AttributeType::String => {
                let header = self.parser.parse::<SpecialAttributeHeader>()?;
                let bytelen = u64::from(header.bytelen);
                self.update_next_attr_start_offset(bytelen);
                // `self.parser.reader().by_ref().take(bytelen)` is rejected by
                // borrowck (of rustc 1.31.0-beta.15 (4b3a1d911 2018-11-20)).
                let reader = io::Read::take(self.parser.reader(), bytelen);
                loader.load_string_buffered(reader, bytelen)
            }
            _ => self.load_next_impl(attr_type, loader, start_pos, attr_index),
        }
    }

    /// Returns the syntactic position of the attribute currently reading.
    #[inline]
    #[must_use]
    fn position(&self, start_pos: u64, index: usize) -> SyntacticPosition {
        SyntacticPosition {
            component_byte_pos: start_pos,
            attribute_index: Some(index),
            ..self.parser.position()
        }
    }

    /// Creates an iterator emitting attribute values.
    #[inline]
    pub fn iter<V, I>(&mut self, loaders: I) -> iter::BorrowedIter<'_, 'a, R, I::IntoIter>
    where
        V: LoadAttribute,
        I: IntoIterator<Item = V>,
    {
        iter::BorrowedIter::new(self, loaders.into_iter())
    }

    /// Creates an iterator emitting attribute values with buffered I/O.
    #[inline]
    pub fn iter_buffered<V, I>(
        &mut self,
        loaders: I,
    ) -> iter::BorrowedIterBuffered<'_, 'a, R, I::IntoIter>
    where
        R: io::BufRead,
        V: LoadAttribute,
        I: IntoIterator<Item = V>,
    {
        iter::BorrowedIterBuffered::new(self, loaders.into_iter())
    }

    /// Creates an iterator emitting attribute values.
    #[inline]
    pub fn into_iter<V, I>(self, loaders: I) -> iter::OwnedIter<'a, R, I::IntoIter>
    where
        V: LoadAttribute,
        I: IntoIterator<Item = V>,
    {
        iter::OwnedIter::new(self, loaders.into_iter())
    }

    /// Creates an iterator emitting attribute values with buffered I/O.
    #[inline]
    pub fn into_iter_buffered<V, I>(self, loaders: I) -> iter::OwnedIterBuffered<'a, R, I::IntoIter>
    where
        R: io::BufRead,
        V: LoadAttribute,
        I: IntoIterator<Item = V>,
    {
        iter::OwnedIterBuffered::new(self, loaders.into_iter())
    }
}
