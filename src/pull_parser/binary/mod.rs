//! FBX binary parser.

pub use self::reader::ParserSource;
pub use self::version::{FbxVersion, ParserVersion};

pub mod header;
pub mod reader;
mod version;
