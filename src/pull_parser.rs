//! Pull parser for FBX binary.

pub use self::{
    error::{Error, Result, Warning},
    position::SyntacticPosition,
    reader::ParserSource,
    version::ParserVersion,
};

pub mod error;
mod position;
pub mod reader;
pub mod v7400;
mod version;
