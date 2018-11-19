//! Reader with position cache.

use std::io;
use std::io::SeekFrom;

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
    /// Creates a new `PositionCache`.
    pub fn new(inner: R) -> Self {
        Self { inner, position: 0 }
    }

    /// Creates a new `PositionCache` with the given offset.
    pub fn with_offset(inner: R, offset: usize) -> Self {
        Self {
            inner,
            position: offset,
        }
    }

    /// Unwraps the wrapper and returns the inner reader.
    pub fn into_inner(self) -> R {
        self.inner
    }

    /// Returns the current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Skips the given distance.
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
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt);
        self.advance(amt);
    }
}

impl<R: io::Read> super::ParserSource for PositionCacheReader<R> {
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
