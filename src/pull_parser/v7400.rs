//! Parser-related stuff for FBX 7.4 or later.

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
