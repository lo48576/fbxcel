//! Object metadata.

use string_interner::{self, Sym};

use crate::v7400::object::ObjectId;

/// Symbol for interned object class and subclass.
// This is an opaque-typedef pattern.
// `string_interner::Sym` has efficient implementation, so use it internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ObjectClassSym(Sym);

impl string_interner::Symbol for ObjectClassSym {
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

/// Metadata of object node.
#[derive(Debug, Clone)]
pub(crate) struct ObjectMeta {
    /// Object ID.
    id: ObjectId,
    /// Name (if exists).
    name: Option<String>,
    /// Class.
    class: ObjectClassSym,
    /// Subclass.
    subclass: ObjectClassSym,
}

impl ObjectMeta {
    /// Creates a new `ObjectMeta`.
    pub(crate) fn new(
        id: ObjectId,
        name: Option<String>,
        class: ObjectClassSym,
        subclass: ObjectClassSym,
    ) -> Self {
        Self {
            id,
            name,
            class,
            subclass,
        }
    }

    /// Returns object ID.
    pub(crate) fn object_id(&self) -> ObjectId {
        self.id
    }

    /// Returns object name.
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| s.as_str())
    }

    /// Returns object class symbol.
    pub(crate) fn class_sym(&self) -> ObjectClassSym {
        self.class
    }

    /// Returns object subclass symbol.
    pub(crate) fn subclass_sym(&self) -> ObjectClassSym {
        self.subclass
    }
}
