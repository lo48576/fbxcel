//! Parser data source.

use std::io;

use byteorder::{LittleEndian, ReadBytesExt};

pub use self::position_cache::PositionCacheReader;
pub use self::source::{PlainSource, SeekableSource};

mod position_cache;
mod source;

/// A trait for types which can be data sources.
pub trait ParserSource: Sized + io::Read {
    /// Returns the offset of a byte which would be read next.
    // Reader types with `io::Seek` can implement this by unwrapping
    // `self.seek(SeekFrom::Current(0))`, but this can be inefficient and use of
    // `PositionCacheReader` is reccomended.
    fn position(&self) -> u64;

    /// Skips the given size.
    // Reader types can make this more efficient by implementing using
    // `io::Seek::seek` if possible.
    fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        // NOTE: `let mut limited = self.by_ref().take(distance);` is E0507.
        let mut limited = io::Read::take(self.by_ref(), distance);
        io::copy(&mut limited, &mut io::sink())?;
        Ok(())
    }

    /// Skips to the given position.
    ///
    /// # Panics
    ///
    /// Panics if the given position is behind current position.
    // Reader types can make this more efficient by implementing using
    // `io::Seek::seek` if possible.
    fn skip_to(&mut self, pos: u64) -> io::Result<()> {
        let distance = pos
            .checked_sub(self.position())
            .expect("Attempt to skip backward");
        self.skip_distance(distance)
    }
}

impl<R: ParserSource> ParserSource for &mut R {
    fn position(&self) -> u64 {
        (**self).position()
    }

    fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        (**self).skip_distance(distance)
    }

    fn skip_to(&mut self, pos: u64) -> io::Result<()> {
        (**self).skip_to(pos)
    }
}

/// Extension trait for parser source type.
pub(crate) trait ParserSourceExt: ParserSource {
    /// Reads a `u8` value.
    fn read_u8(&mut self) -> io::Result<u8> {
        ReadBytesExt::read_u8(self)
    }

    /// Reads a `u16` value.
    fn read_u16(&mut self) -> io::Result<u16> {
        ReadBytesExt::read_u16::<LittleEndian>(self)
    }

    /// Reads a `u32` value.
    fn read_u32(&mut self) -> io::Result<u32> {
        ReadBytesExt::read_u32::<LittleEndian>(self)
    }

    /// Reads a `u64` value.
    fn read_u64(&mut self) -> io::Result<u64> {
        ReadBytesExt::read_u64::<LittleEndian>(self)
    }
}

impl<R: ParserSource> ParserSourceExt for R {}
