//! Parser data source.
//!
//! FBX parsers requires cursor position tracking and skipping.
//! These features are provided by the `Reader<R>` wrapper type.

use std::fmt;
use std::io::{self, SeekFrom};

/// Possibly specialized functions for the stream.
#[derive(Clone, Copy)]
struct ReaderFnTable<R> {
    /// Skips (seeks formward) the given size.
    skip_distance: fn(&mut Reader<R>, u64) -> io::Result<()>,
}

impl<R: io::Read> ReaderFnTable<R> {
    /// Creates a new function table for a plain reader.
    #[inline]
    #[must_use]
    fn new_for_plain() -> Self {
        Self {
            skip_distance: Self::skip_distance_plain,
        }
    }

    /// Creates a new function table for a seekable reader.
    #[inline]
    #[must_use]
    fn new_for_seekable() -> Self
    where
        R: io::Seek,
    {
        Self {
            skip_distance: Self::skip_distance_seekable,
        }
    }

    /// Skips (seeks formward) the given size.
    ///
    /// More efficient implementation [`skip_distance_seekable`][`Self::skip_distance_seekable`]
    /// is provided for seekable stream.
    #[inline]
    fn skip_distance_plain(reader: &mut Reader<R>, distance: u64) -> io::Result<()> {
        // NOTE: `let mut limited = self.by_ref().take(distance);` is E0507.
        let mut limited = io::Read::take(reader.inner.by_ref(), distance);
        io::copy(&mut limited, &mut io::sink())?;
        Ok(())
    }

    /// Skips (seeks formward) the given size.
    fn skip_distance_seekable(reader: &mut Reader<R>, mut distance: u64) -> io::Result<()>
    where
        R: io::Seek,
    {
        while distance > 0 {
            let part = std::cmp::min(distance, std::i64::MAX as u64);
            reader.inner.seek(SeekFrom::Current(part as i64))?;
            reader.advance(part as usize);
            distance -= part;
        }
        Ok(())
    }
}

/// A wrapper type of the source reader.
#[derive(Clone)]
pub(crate) struct Reader<R> {
    /// Inner stream.
    inner: R,
    /// Cached current stream position.
    position: usize,
    /// Function table.
    fn_table: ReaderFnTable<R>,
}

impl<R: fmt::Debug> fmt::Debug for Reader<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Reader")
            .field("inner", &self.inner)
            .field("position", &self.position)
            .finish()
    }
}

impl<R: io::Read> Reader<R> {
    /// Creates a new reader.
    #[inline]
    #[must_use]
    pub(crate) fn new(inner: R, current_position: usize) -> Self {
        Self {
            inner,
            position: current_position,
            fn_table: ReaderFnTable::new_for_plain(),
        }
    }

    /// Creates a new reader.
    #[inline]
    #[must_use]
    pub(crate) fn with_seekable(inner: R, current_position: usize) -> Self
    where
        R: io::Seek,
    {
        Self {
            inner,
            position: current_position,
            fn_table: ReaderFnTable::new_for_seekable(),
        }
    }

    /// Returns the current position.
    #[inline]
    #[must_use]
    pub(crate) fn position(&self) -> u64 {
        self.position as u64
    }

    /// Skips the given distance.
    ///
    /// A seek beyond the end of a stream is allowed, but behavior is defined by
    /// the inner stream implementation.
    /// See the document for [`std::io::Seek::seek()`].
    #[inline]
    pub(crate) fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        (self.fn_table.skip_distance)(self, distance)
    }

    /// Skips (seeks forward) to the given position.
    ///
    /// Reader types can make this more efficient using [`std::io::Seek::seek`]
    /// if possible.
    ///
    /// # Panics
    ///
    /// Panics if the given position is behind the current position.
    #[inline]
    pub(crate) fn skip_to(&mut self, pos: u64) -> io::Result<()> {
        let distance = pos
            .checked_sub(self.position())
            .expect("Attempt to skip backward");
        self.skip_distance(distance)
    }

    /// Advances the position counter.
    #[inline]
    fn advance(&mut self, n: usize) {
        self.position = self.position.checked_add(n).expect("Position overflowed");
    }
}

impl<R: io::Read> io::Read for Reader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.read(buf)?;
        self.advance(size);
        Ok(size)
    }
}

impl<R: io::BufRead> io::BufRead for Reader<R> {
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
