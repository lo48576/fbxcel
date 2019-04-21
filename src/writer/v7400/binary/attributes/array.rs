//! Array attributes things.

use std::{
    convert::TryFrom,
    io::{self, Seek, Write},
};

use crate::{
    low::v7400::{ArrayAttributeEncoding, ArrayAttributeHeader, AttributeType},
    writer::v7400::binary::{
        attributes::IntoBytes, AttributesWriter, CompressionError, Error, Result,
    },
};

/// A trait for types which can be represented as multiple bytes array.
pub(crate) trait IntoBytesMulti<E>: Sized {
    /// Calls the given function with the bytes array multiple times.
    fn call_with_le_bytes_multi(
        self,
        f: impl FnMut(&[u8]) -> std::result::Result<(), E>,
    ) -> std::result::Result<usize, E>;
}

impl<T: IntoBytes, E, I: IntoIterator<Item = std::result::Result<T, E>>> IntoBytesMulti<E> for I {
    fn call_with_le_bytes_multi(
        self,
        mut f: impl FnMut(&[u8]) -> std::result::Result<(), E>,
    ) -> std::result::Result<usize, E> {
        let mut count = 0usize;
        self.into_iter()
            .inspect(|_| count = count.checked_add(1).expect("Too many elements"))
            .try_for_each(|elem| elem?.call_with_le_bytes(&mut f))?;

        Ok(count)
    }
}

/// Writes array elements into the given writer.
pub(crate) fn write_elements_result_iter<T, E>(
    mut writer: impl Write,
    iter: impl IntoIterator<Item = std::result::Result<T, E>>,
) -> Result<u32>
where
    T: IntoBytes,
    E: Into<Error>,
{
    let elements_count = iter
        .into_iter()
        .map(|res| res.map_err(Into::into))
        .call_with_le_bytes_multi(|bytes| writer.write_all(bytes).map_err(Into::into))?;
    let elements_count = u32::try_from(elements_count)
        .map_err(|_| Error::TooManyArrayAttributeElements(elements_count + 1))?;

    Ok(elements_count)
}

/// Writes the given array attribute header.
pub(crate) fn write_array_header(
    mut writer: impl Write,
    header: &ArrayAttributeHeader,
) -> io::Result<()> {
    writer.write_all(&header.elements_count.to_le_bytes())?;
    writer.write_all(&header.encoding.to_u32().to_le_bytes())?;
    writer.write_all(&header.bytelen.to_le_bytes())?;

    Ok(())
}

/// Writes the given array attribute.
pub(crate) fn write_array_attr_result_iter<W: Write + Seek, T: IntoBytes, E: Into<Error>>(
    writer: &mut AttributesWriter<'_, W>,
    ty: AttributeType,
    encoding: Option<ArrayAttributeEncoding>,
    iter: impl IntoIterator<Item = std::result::Result<T, E>>,
) -> Result<()> {
    let encoding = encoding.unwrap_or(ArrayAttributeEncoding::Direct);

    let header_pos = writer.initialize_array(ty, encoding)?;

    // Write elements.
    let start_pos = writer.sink().stream_position()?;
    let elements_count = match encoding {
        ArrayAttributeEncoding::Direct => write_elements_result_iter(writer.sink(), iter)?,
        ArrayAttributeEncoding::Zlib => {
            let mut sink = libflate::zlib::Encoder::new(writer.sink())?;
            let count = write_elements_result_iter(&mut sink, iter)?;
            sink.finish()
                .into_result()
                .map_err(CompressionError::Zlib)?;
            count
        }
    };
    let end_pos = writer.sink().stream_position()?;
    let bytelen = end_pos - start_pos;

    // Calculate header fields.
    let bytelen = u32::try_from(bytelen).map_err(|_| Error::AttributeTooLong(bytelen as usize))?;

    // Write real array header.
    writer.finalize_array(
        header_pos,
        &ArrayAttributeHeader {
            elements_count,
            encoding,
            bytelen,
        },
    )?;

    Ok(())
}
