//! FBX DOM for FBX 7.4 (or compatible versions).

use string_interner::Sym;

pub use self::core::Core;
pub use self::document::{Document, Loader};
pub use self::node::{DowncastId, NodeId};
pub use self::parsed::ParsedData;

mod core;
mod document;
pub mod node;
pub mod object;
mod parsed;

/// Symbol for interned string.
// This is an opaque-typedef pattern.
// `string_interner::Sym` has efficient implementation, so use it internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct StrSym(Sym);

impl string_interner::Symbol for StrSym {
    /// This may panic if the given value is too large.
    ///
    /// As of writing this, string-interner 0.7.0 panics if the given value is
    /// greater than `u32::max_value() - 1`.
    /// See [`string_interner::Sym`] for detail.
    fn from_usize(v: usize) -> Self {
        StrSym(Sym::from_usize(v))
    }

    fn to_usize(self) -> usize {
        self.0.to_usize()
    }
}
