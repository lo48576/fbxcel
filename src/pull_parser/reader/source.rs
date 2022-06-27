//! Wrapper types for parser sources.

use std::io;

use crate::pull_parser::{reader::PositionCacheReader, ParserSource};

/// Source with plain reader backend.
///
/// This may be inefficient, but works with any reader types.
/// It is recommended to use [`SeekableSource`] if the reader implements
/// [`std::io::Seek`].
///
/// This internally uses `PositionCacheReader`, so users don't need to wrap
/// readers by `PositionCacheReader` manually.
#[derive(Debug, Clone, Copy)]
pub struct PlainSource<R> {
    /// Inner reader.
    inner: PositionCacheReader<R>,
}

impl<R: io::Read> PlainSource<R> {
    /// Creates a new `PlainSource`.
    #[inline]
    #[must_use]
    pub fn new(inner: R) -> Self {
        Self {
            inner: PositionCacheReader::new(inner),
        }
    }

    /// Creates a new `PlainSource` with the given offset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel::pull_parser::reader::PlainSource;
    /// use fbxcel::pull_parser::ParserSource;
    ///
    /// let msg = "Hello, world!";
    /// let len = msg.len() as u64;
    /// let mut reader = std::io::Cursor::new(msg);
    /// let mut reader = PlainSource::with_offset(&mut reader, 42);
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
            inner: PositionCacheReader::with_offset(inner, offset),
        }
    }
}

impl<R: io::Read> io::Read for PlainSource<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::BufRead> io::BufRead for PlainSource<R> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<R: io::Read> ParserSource for PlainSource<R> {
    #[inline]
    fn position(&self) -> u64 {
        self.inner.position() as u64
    }

    #[inline]
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
/// This may be more efficient than [`PlainSource`], but works only with reader
/// types implementing `std::io::Seek`.
///
/// This internally uses [`PositionCacheReader`], so users don't need to wrap
/// readers by [`PositionCacheReader`] manually.
#[derive(Debug, Clone, Copy)]
pub struct SeekableSource<R> {
    /// Inner reader.
    inner: PositionCacheReader<R>,
}

impl<R: io::Read + io::Seek> SeekableSource<R> {
    /// Creates a new `SeekableSource`.
    #[inline]
    #[must_use]
    pub fn new(inner: R) -> Self {
        Self {
            inner: PositionCacheReader::new(inner),
        }
    }

    /// Creates a new `SeekableSource` with the given offset.
    ///
    /// # Examples
    ///
    /// ```
    /// # use fbxcel::pull_parser::reader::SeekableSource;
    /// use fbxcel::pull_parser::ParserSource;
    ///
    /// let msg = "Hello, world!";
    /// let len = msg.len() as u64;
    /// let mut reader = std::io::Cursor::new(msg);
    /// let mut reader = SeekableSource::with_offset(&mut reader, 42);
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
            inner: PositionCacheReader::with_offset(inner, offset),
        }
    }
}

impl<R: io::Read> io::Read for SeekableSource<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: io::BufRead> io::BufRead for SeekableSource<R> {
    #[inline]
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.inner.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amt: usize) {
        self.inner.consume(amt)
    }
}

impl<R: io::Read + io::Seek> ParserSource for SeekableSource<R> {
    #[inline]
    fn position(&self) -> u64 {
        self.inner.position() as u64
    }

    #[inline]
    fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        // `PositionCacheReader<R>::skip_distance` will be available only when
        // `R: io::Seek`, and it will use `io::Seek::seek` efficiently.
        self.inner.skip_distance(distance)
    }
}
