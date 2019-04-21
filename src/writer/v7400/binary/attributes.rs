//! Node attributes writer.

use std::{
    convert::TryFrom,
    io::{self, Seek, SeekFrom, Write},
    mem::size_of,
};

use crate::{
    low::v7400::{ArrayAttributeEncoding, ArrayAttributeHeader, AttributeType},
    writer::v7400::binary::{Error, Result, Writer},
};

/// Node attributes writer.
pub struct AttributesWriter<'a, W: Write> {
    /// Inner writer.
    writer: &'a mut Writer<W>,
}

macro_rules! impl_arr_from_iter {
    ($(
        $(#[$meta:meta])*
        $name:ident: $ty_elem:ty {
            from_result_iter: $name_from_result_iter:ident,
            tyval: $tyval:ident,
            ty_real: $ty_real:ty,
            to_bytes: $to_bytes:expr,
        },
    )*) => {$(
        $(#[$meta])*
        pub fn $name(
            &mut self,
            encoding: impl Into<Option<ArrayAttributeEncoding>>,
            iter: impl IntoIterator<Item = $ty_elem>,
        ) -> Result<()> {
            let encoding = encoding.into().unwrap_or(ArrayAttributeEncoding::Direct);
            if encoding != ArrayAttributeEncoding::Direct {
                unimplemented!("encoding={:?}", encoding);
            }

            let header_pos = self.initialize_array(AttributeType::$tyval, encoding)?;

            // Write elements.
            let mut elements_count = 0u32;
            iter.into_iter().try_for_each(|elem| -> Result<()> {
                elements_count = elements_count
                    .checked_add(1)
                    .ok_or_else(|| Error::TooManyArrayAttributeElements(elements_count as usize + 1))?;
                self.writer.sink().write_all(
                    &{$to_bytes}(elem)
                )?;

                Ok(())
            })?;

            // Calculate header fields.
            let bytelen = elements_count as usize * size_of::<$ty_real>();
            let bytelen = u32::try_from(bytelen).map_err(|_| Error::AttributeTooLong(bytelen))?;

            // Write real array header.
            self.finalize_array(
                header_pos,
                &ArrayAttributeHeader {
                    elements_count,
                    encoding,
                    bytelen,
                },
            )?;

            Ok(())
        }

        $(#[$meta])*
        pub fn $name_from_result_iter<E>(
            &mut self,
            encoding: impl Into<Option<ArrayAttributeEncoding>>,
            iter: impl IntoIterator<Item = std::result::Result<$ty_elem, E>>,
        ) -> Result<()>
        where
            E: Into<Box<std::error::Error + 'static>>,
        {
            let encoding = encoding.into().unwrap_or(ArrayAttributeEncoding::Direct);
            if encoding != ArrayAttributeEncoding::Direct {
                unimplemented!("encoding={:?}", encoding);
            }

            let header_pos = self.initialize_array(AttributeType::$tyval, encoding)?;

            // Write elements.
            let mut elements_count = 0u32;
            iter.into_iter().try_for_each(|elem| -> Result<()> {
                let elem = elem.map_err(|e| Error::UserDefined(e.into()))?;
                elements_count = elements_count
                    .checked_add(1)
                    .ok_or_else(|| Error::TooManyArrayAttributeElements(elements_count as usize + 1))?;
                self.writer.sink().write_all(
                    &{$to_bytes}(elem)
                )?;

                Ok(())
            })?;

            // Calculate header fields.
            let bytelen = elements_count as usize * size_of::<$ty_real>();
            let bytelen = u32::try_from(bytelen).map_err(|_| Error::AttributeTooLong(bytelen))?;

            // Write real array header.
            self.finalize_array(
                header_pos,
                &ArrayAttributeHeader {
                    elements_count,
                    encoding,
                    bytelen,
                },
            )?;

            Ok(())
        }
    )*}
}

impl<'a, W: Write + Seek> AttributesWriter<'a, W> {
    /// Creates a new `AttributesWriter`.
    pub(crate) fn new(writer: &'a mut Writer<W>) -> Self {
        Self { writer }
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
        node_header.num_attributes = node_header
            .num_attributes
            .checked_add(1)
            .ok_or_else(|| Error::TooManyAttributes(node_header.num_attributes as usize))?;

        Ok(())
    }

    /// Writes a single boolean attribute.
    pub fn append_bool(&mut self, v: bool) -> Result<()> {
        self.update_node_header()?;
        self.write_type_code(AttributeType::Bool)?;
        let v = if v { b'Y' } else { b'T' };
        self.writer
            .sink()
            .write_all(&v.to_le_bytes())
            .map_err(Into::into)
    }

    /// Writes a single `i16` attribute.
    pub fn append_i16(&mut self, v: i16) -> Result<()> {
        self.update_node_header()?;
        self.write_type_code(AttributeType::I16)?;
        self.writer
            .sink()
            .write_all(&v.to_le_bytes())
            .map_err(Into::into)
    }

    /// Writes a single `i32` attribute.
    pub fn append_i32(&mut self, v: i32) -> Result<()> {
        self.update_node_header()?;
        self.write_type_code(AttributeType::I32)?;
        self.writer
            .sink()
            .write_all(&v.to_le_bytes())
            .map_err(Into::into)
    }

    /// Writes a single `i64` attribute.
    pub fn append_i64(&mut self, v: i64) -> Result<()> {
        self.update_node_header()?;
        self.write_type_code(AttributeType::I64)?;
        self.writer
            .sink()
            .write_all(&v.to_le_bytes())
            .map_err(Into::into)
    }

    /// Writes a single `f32` attribute.
    pub fn append_f32(&mut self, v: f32) -> Result<()> {
        self.update_node_header()?;
        self.write_type_code(AttributeType::F32)?;
        self.writer
            .sink()
            .write_all(&v.to_bits().to_le_bytes())
            .map_err(Into::into)
    }

    /// Writes a single `f64` attribute.
    pub fn append_f64(&mut self, v: f64) -> Result<()> {
        self.update_node_header()?;
        self.write_type_code(AttributeType::F64)?;
        self.writer
            .sink()
            .write_all(&v.to_bits().to_le_bytes())
            .map_err(Into::into)
    }

    /// Writes the given array attribute header.
    fn write_array_header(&mut self, header: &ArrayAttributeHeader) -> Result<()> {
        self.writer
            .sink()
            .write_all(&header.elements_count.to_le_bytes())?;
        self.writer
            .sink()
            .write_all(&header.encoding.to_u32().to_le_bytes())?;
        self.writer
            .sink()
            .write_all(&header.bytelen.to_le_bytes())?;

        Ok(())
    }

    /// Writes some headers for an array attibute, and returns header position.
    fn initialize_array(
        &mut self,
        ty: AttributeType,
        encoding: ArrayAttributeEncoding,
    ) -> Result<u64> {
        self.update_node_header()?;

        // Write attribute header.
        self.write_type_code(ty)?;
        let header_pos = self.writer.sink().seek(SeekFrom::Current(0))?;

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
        let end_pos = self.writer.sink().seek(SeekFrom::Current(0))?;
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
            ty_real: u8,
            to_bytes: |elem: bool| if elem { [b'Y'] } else { [b'T'] },
        },

        /// Writes an `i32` array attribute.
        append_arr_i32_from_iter: i32 {
            from_result_iter: append_arr_i32_from_result_iter,
            tyval: ArrI32,
            ty_real: i32,
            to_bytes: |elem: i32| elem.to_le_bytes(),
        },

        /// Writes an `i64` array attribute.
        append_arr_i64_from_iter: i64 {
            from_result_iter: append_arr_i64_from_result_iter,
            tyval: ArrI64,
            ty_real: i64,
            to_bytes: |elem: i64| elem.to_le_bytes(),
        },

        /// Writes an `f32` array attribute.
        append_arr_f32_from_iter: f32 {
            from_result_iter: append_arr_f32_from_result_iter,
            tyval: ArrI32,
            ty_real: f32,
            to_bytes: |elem: f32| elem.to_bits().to_le_bytes(),
        },

        /// Writes an `f64` array attribute.
        append_arr_f64_from_iter: f64 {
            from_result_iter: append_arr_f64_from_result_iter,
            tyval: ArrI64,
            ty_real: f64,
            to_bytes: |elem: f64| elem.to_bits().to_le_bytes(),
        },
    }

    /// Writes some headers for a special attribute, and returns the special
    /// header position.
    fn initialize_special(&mut self, ty: AttributeType) -> Result<u64> {
        self.update_node_header()?;

        // Write attribute header.
        self.write_type_code(ty)?;

        // Write special attribute header (dummy).
        let header_pos = self.writer.sink().seek(SeekFrom::Current(0))?;
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
        let end_pos = self.writer.sink().seek(SeekFrom::Current(0))?;
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
                .ok_or_else(|| Error::AttributeTooLong(std::usize::MAX))?;

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
        E: Into<Box<std::error::Error + 'static>>,
    {
        let header_pos = self.initialize_special(AttributeType::Binary)?;

        let mut len = 0usize;
        iter.into_iter().try_for_each(|v| -> Result<_> {
            let v = v.map_err(|e| Error::UserDefined(e.into()))?;
            self.writer.sink().write_all(&[v])?;
            len = len
                .checked_add(1)
                .ok_or_else(|| Error::AttributeTooLong(std::usize::MAX))?;

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
                .ok_or_else(|| Error::AttributeTooLong(std::usize::MAX))?;

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
        E: Into<Box<std::error::Error + 'static>>,
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
                .ok_or_else(|| Error::AttributeTooLong(std::usize::MAX))?;

            Ok(())
        })?;

        self.finalize_special(header_pos, len)?;

        Ok(())
    }
}
