//! Pull parser for FBX binary.

pub use self::error::{Error, Result};
pub use self::header::FbxHeader;
pub use self::reader::ParserSource;
use self::reader::ParserSourceExt;
pub use self::version::ParserVersion;

pub mod error;
pub mod header;
pub mod reader;
pub mod v7400;
mod version;
