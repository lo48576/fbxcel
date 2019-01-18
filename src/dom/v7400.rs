//! FBX DOM for FBX 7.4 (or compatible versions).

pub use self::core::Core;
pub use self::document::{Document, Loader};
pub use self::node::{DowncastId, NodeId};
pub use self::parsed::ParsedData;

mod connection;
mod core;
mod document;
pub mod node;
pub mod object;
mod parsed;

/// Symbol for interned string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct StrSym(usize);

impl string_interner::Symbol for StrSym {
    fn from_usize(v: usize) -> Self {
        StrSym(v)
    }

    fn to_usize(self) -> usize {
        self.0
    }
}
