//! Pull parser for FBX binary.

pub use self::error::{Error, Result, Warning};
pub use self::position::SyntacticPosition;
pub use self::reader::ParserSource;
pub use self::version::ParserVersion;

pub mod error;
mod position;
pub mod reader;
pub mod v7400;
mod version;
