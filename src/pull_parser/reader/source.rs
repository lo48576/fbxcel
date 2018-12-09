//! Wrapper types for parser sources.

use std::io;

use crate::pull_parser::reader::PositionCacheReader;
use crate::pull_parser::ParserSource;

/// Source with plain reader backend.
///
/// This may be inefficient, but works with any reader types.
#[derive(Debug, Clone, Copy)]
pub struct PlainSource<R> {
    /// Inner reader.
    inner: PositionCacheReader<R>,
}

impl<R: io::Read> PlainSource<R> {
    /// Creates a new `PlainSource`.
    pub fn new(inner: R) -> Self {
        Self {
            inner: PositionCacheReader::new(inner),
        }
    }

    /// Creates a new `PlainSource` with the given offset.
    pub fn with_offset(inner: R, offset: usize) -> Self {
        Self {
            inner: PositionCacheReader::with_offset(inner, offset),
        }
    }
}

impl<R: io::Read> io::Read for PlainSource<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::BufRead> io::BufRead for PlainSource<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<R: io::Read> ParserSource for PlainSource<R> {
    fn position(&self) -> u64 {
        self.inner.position() as u64
    }

    fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        // NOTE: `self.inner.take(distance)` is E0507.
        io::copy(
            &mut io::Read::take(&mut self.inner, distance),
            &mut io::sink(),
        )?;
        Ok(())
    }
}

/// Source with seekable reader backend.
///
/// This may be efficient, but works only with reader types implementing
/// [`std::io::Seek`].
#[derive(Debug, Clone, Copy)]
pub struct SeekableSource<R> {
    /// Inner reader.
    inner: PositionCacheReader<R>,
}

impl<R: io::Read + io::Seek> SeekableSource<R> {
    /// Creates a new `SeekableSource`.
    pub fn new(inner: R) -> Self {
        Self {
            inner: PositionCacheReader::new(inner),
        }
    }

    /// Creates a new `SeekableSource` with the given offset.
    pub fn with_offset(inner: R, offset: usize) -> Self {
        Self {
            inner: PositionCacheReader::with_offset(inner, offset),
        }
    }
}

impl<R: io::Read> io::Read for SeekableSource<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::BufRead> io::BufRead for SeekableSource<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<R: io::Read + io::Seek> ParserSource for SeekableSource<R> {
    fn position(&self) -> u64 {
        self.inner.position() as u64
    }

    fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        // `PositionCacheReader<R>::skip_distance` will be available only when
        // `R: io::Seek`, and it will use `io::Seek::seek` efficiently.
        self.inner.skip_distance(distance)
    }
}
