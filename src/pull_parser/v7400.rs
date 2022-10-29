//! Parser-related stuff for FBX 7.4 or later.
//!
//! To see how to setup a parser, see module documentation of [`pull_parser`][`super`].

pub(crate) use self::read::{FromParser, FromReader};
pub use self::{
    attribute::{Attributes, LoadAttribute},
    event::{Event, StartNode},
    parser::Parser,
};

pub mod attribute;
mod event;
mod parser;
mod read;
