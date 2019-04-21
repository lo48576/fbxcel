//! Array attributes things.

use std::{convert::TryFrom, io::Write};

use crate::writer::v7400::binary::{attributes::IntoBytes, Error, Result};

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
pub(crate) fn write_elements_direct_iter<T>(
    writer: impl Write,
    iter: impl IntoIterator<Item = T>,
) -> Result<u32>
where
    T: IntoBytes,
{
    /// A dummy type for impossible error.
    enum Never {}
    impl From<Never> for Error {
        fn from(_: Never) -> Self {
            unreachable!("Should never happen")
        }
    }
    write_elements_result_iter(writer, iter.into_iter().map(Ok::<_, Never>))
}

/// Writes array elements into the given writer.
pub(crate) fn write_elements_result_iter<T, E>(
    mut writer: impl Write,
    iter: impl IntoIterator<Item = std::result::Result<T, E>>,
) -> Result<u32>
where
    T: IntoBytes,
    E: Into<Error>,
    //Error: From<E>,
{
    let elements_count = iter
        .into_iter()
        .map(|res| res.map_err(Into::into))
        .call_with_le_bytes_multi(|bytes| writer.write_all(bytes).map_err(Into::into))?;
    let elements_count = u32::try_from(elements_count)
        .map_err(|_| Error::TooManyArrayAttributeElements(elements_count + 1))?;

    Ok(elements_count)
}
