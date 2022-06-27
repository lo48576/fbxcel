//! Node attributes writer.

use std::{
    convert::TryFrom,
    io::{self, Seek, SeekFrom, Write},
};

use crate::{
    low::v7400::{ArrayAttributeEncoding, ArrayAttributeHeader, AttributeType},
    writer::v7400::binary::{Error, Result, Writer},
};

mod array;

/// A dummy type for impossible error.
pub(crate) enum Never {}

impl From<Never> for Error {
    fn from(_: Never) -> Self {
        unreachable!("Should never happen")
    }
}

/// A trait for types which can be represented as single bytes array.
pub(crate) trait IntoBytes: Sized {
    /// Calls the given function with the bytes array.
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R;
}

impl IntoBytes for bool {
    #[inline]
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R {
        let v = if self { b'Y' } else { b'T' };
        f(&v.to_le_bytes())
    }
}

impl IntoBytes for i16 {
    #[inline]
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R {
        f(&self.to_le_bytes())
    }
}

impl IntoBytes for i32 {
    #[inline]
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R {
        f(&self.to_le_bytes())
    }
}

impl IntoBytes for i64 {
    #[inline]
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R {
        f(&self.to_le_bytes())
    }
}

impl IntoBytes for f32 {
    #[inline]
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R {
        f(&self.to_bits().to_le_bytes())
    }
}

impl IntoBytes for f64 {
    #[inline]
    fn call_with_le_bytes<R>(self, f: impl FnOnce(&[u8]) -> R) -> R {
        f(&self.to_bits().to_le_bytes())
    }
}

/// Node attributes writer.
///
/// See [module documentation](index.html) for usage.
pub struct AttributesWriter<'a, W: Write> {
    /// Inner writer.
    writer: &'a mut Writer<W>,
}

/// Implement `append_*` methods for single value.
macro_rules! impl_single_attr_append {
    ($(
        $(#[$meta:meta])*
        $method:ident($ty:ty): $variant:ident;
    )*) => {
        $(
            $(#[$meta])*
            pub fn $method(&mut self, v: $ty) -> Result<()> {
                self.update_node_header()?;
                self.write_type_code(AttributeType::$variant)?;
                v.call_with_le_bytes(|bytes| self.writer.sink().write_all(bytes))
                    .map_err(Into::into)
            }
        )*
    }
}

/// Implement `append_*` methods for array values.
macro_rules! impl_arr_from_iter {
    ($(
        $(#[$meta:meta])*
        $name:ident: $ty_elem:ty {
            from_result_iter: $name_from_result_iter:ident,
            tyval: $tyval:ident,
        },
    )*) => {$(
        $(#[$meta])*
        #[inline]
        pub fn $name(
            &mut self,
            encoding: impl Into<Option<ArrayAttributeEncoding>>,
            iter: impl IntoIterator<Item = $ty_elem>,
        ) -> Result<()> {
            array::write_array_attr_result_iter(
                self,
                AttributeType::$tyval,
                encoding.into(),
                iter.into_iter().map(Ok::<_, Never>),
            )
        }

        $(#[$meta])*
        #[inline]
        pub fn $name_from_result_iter<E>(
            &mut self,
            encoding: impl Into<Option<ArrayAttributeEncoding>>,
            iter: impl IntoIterator<Item = std::result::Result<$ty_elem, E>>,
        ) -> Result<()>
        where
            E: Into<Box<dyn std::error::Error + 'static>>,
        {
            array::write_array_attr_result_iter(
                self,
                AttributeType::$tyval,
                encoding.into(),
                iter.into_iter().map(|res| res.map_err(|e| Error::UserDefined(e.into()))),
            )
        }
    )*}
}

impl<'a, W: Write + Seek> AttributesWriter<'a, W> {
    /// Creates a new `AttributesWriter`.
    #[inline]
    #[must_use]
    pub(crate) fn new(writer: &'a mut Writer<W>) -> Self {
        Self { writer }
    }

    /// Returns the inner writer.
    #[inline]
    #[must_use]
    pub(crate) fn sink(&mut self) -> &mut W {
        self.writer.sink()
    }

    /// Writes the given attribute type as type code.
    fn write_type_code(&mut self, ty: AttributeType) -> Result<()> {
        self.writer
            .sink()
            .write_all(&ty.type_code().to_le_bytes())
            .map_err(Into::into)
    }

    /// Updates the node header.
    fn update_node_header(&mut self) -> Result<()> {
        let node_header = self
            .writer
            .current_node_header()
            .expect("Should never fail: some nodes must be open if `AttributesWriter` exists");
        node_header.num_attributes =
            node_header
                .num_attributes
                .checked_add(1)
                .ok_or(Error::TooManyAttributes(
                    node_header.num_attributes as usize,
                ))?;

        Ok(())
    }

    impl_single_attr_append! {
        /// Writes a single boolean attribute.
        append_bool(bool): Bool;
        /// Writes a single `i16` attribute.
        append_i16(i16): I16;
        /// Writes a single `i32` attribute.
        append_i32(i32): I32;
        /// Writes a single `i64` attribute.
        append_i64(i64): I64;
        /// Writes a single `f32` attribute.
        append_f32(f32): F32;
        /// Writes a single `f64` attribute.
        append_f64(f64): F64;
    }

    /// Writes the given array attribute header.
    #[inline]
    fn write_array_header(&mut self, header: &ArrayAttributeHeader) -> Result<()> {
        array::write_array_header(self.writer.sink(), header).map_err(Into::into)
    }

    /// Writes some headers for an array attibute, and returns header position.
    pub(crate) fn initialize_array(
        &mut self,
        ty: AttributeType,
        encoding: ArrayAttributeEncoding,
    ) -> Result<u64> {
        self.update_node_header()?;

        // Write attribute header.
        self.write_type_code(ty)?;
        let header_pos = self.writer.sink().stream_position()?;

        // Write array header placeholder.
        self.write_array_header(&ArrayAttributeHeader {
            elements_count: 0,
            encoding,
            bytelen: 0,
        })?;

        Ok(header_pos)
    }

    /// Updates an array attribute header.
    ///
    /// Note that this should be called at the end of the array attribute.
    fn finalize_array(&mut self, header_pos: u64, header: &ArrayAttributeHeader) -> Result<()> {
        // Write real array header.
        let end_pos = self.writer.sink().stream_position()?;
        self.writer.sink().seek(SeekFrom::Start(header_pos))?;
        self.write_array_header(header)?;
        self.writer.sink().seek(SeekFrom::Start(end_pos))?;

        Ok(())
    }

    impl_arr_from_iter! {
        /// Writes a boolean array attribute.
        append_arr_bool_from_iter: bool {
            from_result_iter: append_arr_bool_from_result_iter,
            tyval: ArrBool,
        },

        /// Writes an `i32` array attribute.
        append_arr_i32_from_iter: i32 {
            from_result_iter: append_arr_i32_from_result_iter,
            tyval: ArrI32,
        },

        /// Writes an `i64` array attribute.
        append_arr_i64_from_iter: i64 {
            from_result_iter: append_arr_i64_from_result_iter,
            tyval: ArrI64,
        },

        /// Writes an `f32` array attribute.
        append_arr_f32_from_iter: f32 {
            from_result_iter: append_arr_f32_from_result_iter,
            tyval: ArrF32,
        },

        /// Writes an `f64` array attribute.
        append_arr_f64_from_iter: f64 {
            from_result_iter: append_arr_f64_from_result_iter,
            tyval: ArrF64,
        },
    }

    /// Writes some headers for a special attribute, and returns the special
    /// header position.
    fn initialize_special(&mut self, ty: AttributeType) -> Result<u64> {
        self.update_node_header()?;

        // Write attribute header.
        self.write_type_code(ty)?;

        // Write special attribute header (dummy).
        let header_pos = self.writer.sink().stream_position()?;
        self.writer.sink().write_all(&0u32.to_le_bytes())?;

        Ok(header_pos)
    }

    /// Updates an array attribute header.
    ///
    /// Note that this should be called at the end of the array attribute.
    fn finalize_special(&mut self, header_pos: u64, bytelen: usize) -> Result<()> {
        // Calculate header fields.
        let bytelen = u32::try_from(bytelen).map_err(|_| Error::AttributeTooLong(bytelen))?;

        // Write real special attribute header.
        let end_pos = self.writer.sink().stream_position()?;
        self.writer.sink().seek(SeekFrom::Start(header_pos))?;
        self.writer.sink().write_all(&bytelen.to_le_bytes())?;
        self.writer.sink().seek(SeekFrom::Start(end_pos))?;

        Ok(())
    }

    /// Writes a binary attribute.
    pub fn append_binary_direct(&mut self, binary: &[u8]) -> Result<()> {
        let header_pos = self.initialize_special(AttributeType::Binary)?;

        self.writer.sink().write_all(binary)?;

        self.finalize_special(header_pos, binary.len())?;

        Ok(())
    }

    /// Writes a string attribute.
    pub fn append_string_direct(&mut self, string: &str) -> Result<()> {
        let header_pos = self.initialize_special(AttributeType::String)?;

        self.writer.sink().write_all(string.as_ref())?;

        self.finalize_special(header_pos, string.len())?;

        Ok(())
    }

    /// Writes a binary attribute read from the given reader.
    pub fn append_binary_from_reader(&mut self, mut reader: impl io::Read) -> Result<()> {
        let header_pos = self.initialize_special(AttributeType::Binary)?;

        // Write bytes.
        let written_len = io::copy(&mut reader, self.writer.sink())?;

        self.finalize_special(header_pos, written_len as usize)?;

        Ok(())
    }

    /// Writes a binary attribute from the given iterator.
    pub fn append_binary_from_iter(&mut self, iter: impl IntoIterator<Item = u8>) -> Result<()> {
        let header_pos = self.initialize_special(AttributeType::Binary)?;

        let mut len = 0usize;
        iter.into_iter().try_for_each(|v| -> Result<_> {
            self.writer.sink().write_all(&[v])?;
            len = len
                .checked_add(1)
                .ok_or(Error::AttributeTooLong(std::usize::MAX))?;

            Ok(())
        })?;

        self.finalize_special(header_pos, len)?;

        Ok(())
    }

    /// Writes a binary attribute from the given iterator.
    pub fn append_binary_from_result_iter<E>(
        &mut self,
        iter: impl IntoIterator<Item = std::result::Result<u8, E>>,
    ) -> Result<()>
    where
        E: Into<Box<dyn std::error::Error + 'static>>,
    {
        let header_pos = self.initialize_special(AttributeType::Binary)?;

        let mut len = 0usize;
        iter.into_iter().try_for_each(|v| -> Result<_> {
            let v = v.map_err(|e| Error::UserDefined(e.into()))?;
            self.writer.sink().write_all(&[v])?;
            len = len
                .checked_add(1)
                .ok_or(Error::AttributeTooLong(std::usize::MAX))?;

            Ok(())
        })?;

        self.finalize_special(header_pos, len)?;

        Ok(())
    }

    /// Writes a string attribute from the given iterator.
    pub fn append_string_from_iter(&mut self, iter: impl IntoIterator<Item = char>) -> Result<()> {
        let header_pos = self.initialize_special(AttributeType::String)?;

        let buf = &mut [0u8; 4];
        let mut len = 0usize;
        iter.into_iter().try_for_each(|c| -> Result<_> {
            let char_len = c.encode_utf8(buf).len();
            self.writer.sink().write_all(buf)?;
            len = len
                .checked_add(char_len)
                .ok_or(Error::AttributeTooLong(std::usize::MAX))?;

            Ok(())
        })?;

        self.finalize_special(header_pos, len)?;

        Ok(())
    }

    /// Writes a string attribute from the given iterator.
    pub fn append_string_from_result_iter<E>(
        &mut self,
        iter: impl IntoIterator<Item = std::result::Result<char, E>>,
    ) -> Result<()>
    where
        E: Into<Box<dyn std::error::Error + 'static>>,
    {
        let header_pos = self.initialize_special(AttributeType::String)?;

        let buf = &mut [0u8; 4];
        let mut len = 0usize;
        iter.into_iter().try_for_each(|c| -> Result<_> {
            let c = c.map_err(|e| Error::UserDefined(e.into()))?;
            let char_len = c.encode_utf8(buf).len();
            self.writer.sink().write_all(buf)?;
            len = len
                .checked_add(char_len)
                .ok_or(Error::AttributeTooLong(std::usize::MAX))?;

            Ok(())
        })?;

        self.finalize_special(header_pos, len)?;

        Ok(())
    }
}
