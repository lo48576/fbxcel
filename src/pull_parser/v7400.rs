//! Parser-related stuff for FBX 7.4 or later.
//!
//! To see how to setup a parser, see module documentation of [`pull_parser`].
//!
//! [`pull_parser`]: ../index.html

pub(crate) use self::read::{FromParser, FromReader};
pub use self::{
    attribute::{Attributes, LoadAttribute},
    event::{Event, StartNode},
    parser::{from_reader, from_seekable_reader, Parser},
};

pub mod attribute;
mod event;
mod parser;
mod read;
