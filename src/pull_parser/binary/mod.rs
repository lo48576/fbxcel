//! FBX binary parser.

pub use self::error::{Error, Result};
pub use self::reader::ParserSource;
pub use self::version::{FbxVersion, ParserVersion};

pub mod error;
pub mod header;
pub mod reader;
pub mod v7400;
mod version;
