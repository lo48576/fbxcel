//! Parser data source.
//!
//! FBX parsers requires cursor position tracking and skipping.
//! These features are used through `ParserSource` trait.
//!
//! For reader types which does not provide these features, `reader::*Source`
//! wrappers are provided.
//! Using those wrappers, any types implementing `std::io::Read` can be used as
//! source reader for parsers.
//!
//! Usually users don't need to deal with these types and traits, because parser
//! types or modules will provide simple functions to automatically wrap readers
//! if necessary.

use std::io;

pub use self::{
    position_cache::PositionCacheReader,
    source::{PlainSource, SeekableSource},
};

mod position_cache;
mod source;

/// A trait for types which can be data sources.
///
/// Users can implement this manually, but usually it is enough to use wrappers
/// in the [`reader`] module.
///
/// [`reader`]: index.html
pub trait ParserSource: Sized + io::Read {
    /// Returns the offset of a byte which would be read next.
    ///
    /// This is called many times during parsing, so it is desirable to be fast
    /// as possible.
    ///
    /// Reader types with [`std::io::Seek`] can implement this as
    /// `self.stream_position().unwrap()`, but this is fallible and
    /// can be inefficient.
    /// Use of [`PositionCacheReader`] is reccomended.
    ///
    /// [`std::io::Seek`]: https://doc.rust-lang.org/stable/std/io/trait.Seek.html
    /// [`PositionCacheReader`]: struct.PositionCacheReader.html
    fn position(&self) -> u64;

    /// Skips (seeks formward) the given size.
    ///
    /// Reader types can make this more efficient using [`std::io::Seek::seek`]
    /// if possible.
    ///
    /// # Examples
    ///
    /// ```
    /// use fbxcel::pull_parser::{ParserSource, reader::PlainSource};
    ///
    /// let msg = "Hello, world!";
    /// let len = msg.len() as u64;
    /// let mut reader = std::io::Cursor::new(msg);
    /// let mut reader = PlainSource::new(&mut reader);
    ///
    /// assert_eq!(reader.position(), 0);
    /// reader.skip_distance(7).expect("Failed to skip");
    /// assert_eq!(reader.position(), 7);
    /// ```
    ///
    /// [`std::io::Seek::seek`]:
    /// https://doc.rust-lang.org/stable/std/io/trait.Seek.html#tymethod.seek
    fn skip_distance(&mut self, distance: u64) -> io::Result<()> {
        // NOTE: `let mut limited = self.by_ref().take(distance);` is E0507.
        let mut limited = io::Read::take(self.by_ref(), distance);
        io::copy(&mut limited, &mut io::sink())?;
        Ok(())
    }

    /// Skips (seeks forward) to the given position.
    ///
    /// Reader types can make this more efficient using [`std::io::Seek::seek`]
    /// if possible.
    ///
    /// # Panics
    ///
    /// Panics if the given position is behind the current position.
    ///
    /// ```
    /// use fbxcel::pull_parser::{ParserSource, reader::PlainSource};
    ///
    /// let msg = "Hello, world!";
    /// let len = msg.len() as u64;
    /// let mut reader = std::io::Cursor::new(msg);
    /// let mut reader = PlainSource::new(&mut reader);
    ///
    /// assert_eq!(reader.position(), 0);
    /// reader.skip_to(2).expect("Failed to skip");
    /// assert_eq!(reader.position(), 2);
    /// reader.skip_to(7).expect("Failed to skip");
    /// assert_eq!(reader.position(), 7);
    /// ```
    ///
    /// [`std::io::Seek::seek`]:
    /// https://doc.rust-lang.org/stable/std/io/trait.Seek.html#tymethod.seek
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
