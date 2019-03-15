//! Node name.

use string_interner::{self, Sym};

/// Symbol for interned node name.
// This is an opaque-typedef pattern.
// `string_interner::Sym` has efficient implementation, so use it internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct NodeNameSym(Sym);

impl string_interner::Symbol for NodeNameSym {
    /// This may panic if the given value is too large.
    ///
    /// As of writing this, string-interner 0.7.0 panics if the given value is
    /// greater than `u32::max_value() - 1`.
    /// See [`string_interner::Sym`] for detail.
    fn from_usize(v: usize) -> Self {
        Self(Sym::from_usize(v))
    }

    fn to_usize(self) -> usize {
        self.0.to_usize()
    }
}
