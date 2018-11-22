//! Parser-related stuff for BX 7.4 or later.

use super::error;
use super::{FbxVersion, ParserSource, ParserSourceExt, ParserVersion, Result};

pub use self::attribute::{Attributes, VisitAttribute};
pub use self::event::{Event, StartNode};
pub use self::parser::{from_reader, from_seekable_reader, Parser};

pub mod attribute;
mod event;
mod parser;
