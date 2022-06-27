//! Reader with position cache.

use std::io::{self, SeekFrom};

use crate::pull_parser::ParserSource;

/// A reader with position cache.
///
/// # Panics
///
/// Panics if the position overflows.
// This does not implement `io::Seek` because this does not track absolute
// read positon of the inner reader.
#[derive(Debug, Clone, Copy)]
pub struct PositionCacheReader<R> {
    /// Inner reader.
    inner: R,
    /// Current position.
    position: usize,
}

impl<R: io::Read> PositionCacheReader<R> {
    /// Creates a new `PositionCacheReader`.
    #[inline]
    #[must_use]
    pub fn new(inner: R) -> Self {
        Self { inner, position: 0 }
    }

    /// Creates a new `PositionCacheReader` with the given offset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel::pull_parser::reader::PositionCacheReader;
    /// let msg = "Hello, world!";
    /// let len = msg.len();
    /// let mut reader = std::io::Cursor::new(msg);
    /// let mut reader = PositionCacheReader::with_offset(&mut reader, 42);
    ///
    /// assert_eq!(reader.position(), 42, "Start position is 42");
    /// std::io::copy(&mut reader, &mut std::io::sink())
    ///     .expect("Should never fail");
    /// assert_eq!(reader.position(), len + 42);
    /// ```
    #[inline]
    #[must_use]
    pub fn with_offset(inner: R, offset: usize) -> Self {
        Self {
            inner,
            position: offset,
        }
    }

    /// Unwraps the wrapper and returns the inner reader.
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Returns the current position.
    #[inline]
    #[must_use]
    pub fn position(&self) -> usize {
        self.position
    }

    /// Skips the given distance.
    ///
    /// A seek beyond the end of a stream is allowed, but behavior is defined by
    /// the implementation.
    /// See the document for [`std::io::Seek::seek()`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel::pull_parser::reader::PositionCacheReader;
    /// use fbxcel::pull_parser::ParserSource;
    ///
    /// let msg = "Hello, world!";
    /// let len = msg.len() as u64;
    /// let mut reader = std::io::Cursor::new(msg);
    /// let mut reader = PositionCacheReader::new(&mut reader);
    ///
    /// assert_eq!(reader.position(), 0);
    /// reader.skip_distance(7).expect("Failed to skip");
    /// assert_eq!(reader.position(), 7);
    /// ```
    pub fn skip_distance(&mut self, mut distance: u64) -> io::Result<()>
    where
        R: io::Seek,
    {
        while distance > 0 {
            let part = std::cmp::min(distance, std::i64::MAX as u64);
            self.inner.seek(SeekFrom::Current(part as i64))?;
            self.advance(part as usize);
            distance -= part;
        }
        Ok(())
    }

    /// Advances the position counter.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position = self.position.checked_add(n).expect("Position overflowed");
    }
}

impl<R: io::Read> io::Read for PositionCacheReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.read(buf)?;
        self.advance(size);
        Ok(size)
    }
}

impl<R: io::BufRead> io::BufRead for PositionCacheReader<R> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
        self.advance(amt);
    }
}

impl<R: io::Read> ParserSource for PositionCacheReader<R> {
    #[inline]
    fn position(&self) -> u64 {
        self.position() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::{Cursor, Read};

    fn prepare_iota() -> Cursor<Vec<u8>> {
        let orig = (0..=255).collect::<Vec<u8>>();
        Cursor::new(orig)
    }

    #[test]
    fn read() {
        let reader = PositionCacheReader::new(prepare_iota());
        assert_eq!(
            reader.position(),
            0,
            "`PositionCacheReader::new()` should return a reader with position 0"
        );
        check_read_with_offset(reader, 0);
    }

    #[test]
    fn read_with_offset() {
        const OFFSET: usize = 60;
        let reader = PositionCacheReader::with_offset(prepare_iota(), OFFSET);
        assert_eq!(
            reader.position(),
            OFFSET,
            "`PositionCacheReader::with_offset()` should return a reader with the given offset"
        );
        check_read_with_offset(reader, OFFSET)
    }

    fn check_read_with_offset<R: Read>(mut reader: PositionCacheReader<R>, offset: usize) {
        const BUF_SIZE: usize = 128;

        let mut buf = [0; BUF_SIZE];
        let size = reader
            .read(&mut buf)
            .expect("Read from `Cursor<Vec<u8>>` should never fail");

        assert!(
            size > 0,
            "Read from non-empty `Cursor<Vec<u8>>` should obtain some data"
        );
        // "Offset" is for internal count, not for content to be read.
        // So here use `0..size`, not `OFFSET..(OFFSET+size)`.
        assert_eq!(
            &buf[..size],
            &(0..size as u8).into_iter().collect::<Vec<u8>>()[..],
            "Read should obtain correct data"
        );
        assert_eq!(
            reader.position(),
            offset + size,
            "Position should be correctly updated after a read"
        );
    }
}
