//! Pull parser for FBX binary.

pub use self::error::{Error, Result};
pub use self::reader::ParserSource;
pub(crate) use self::reader::ParserSourceExt;
pub use self::version::ParserVersion;

pub mod error;
pub mod reader;
pub mod v7400;
mod version;
