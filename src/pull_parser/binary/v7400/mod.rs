//! Parser-related stuff for BX 7.4 or later.

use super::error;
use super::{FbxVersion, ParserSource, ParserVersion, Result};

pub use self::parser::{from_reader, from_seekable_reader, Parser};

mod parser;
