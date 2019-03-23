//! Objects connections.

use string_interner::{self, Sym};

use crate::v7400::object::ObjectId;

pub(crate) use self::cache::ConnectionsCache;

mod cache;

/// Symbol for interned connection label.
// This is an opaque-typedef pattern.
// `string_interner::Sym` has efficient implementation, so use it internally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ConnectionLabelSym(Sym);

impl string_interner::Symbol for ConnectionLabelSym {
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

/// Type of a connected node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ConnectedNodeType {
    /// Object.
    Object,
    /// Property.
    Property,
}

/// Connection index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct ConnectionIndex(usize);

impl ConnectionIndex {
    /// Creates a new `ConnectionIndex`.
    pub(crate) fn new(i: usize) -> Self {
        Self(i)
    }

    /// Returns the index.
    pub(crate) fn value(self) -> usize {
        self.0
    }
}

/// Connection data (provided by `C` node).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Connection {
    /// Source object ID.
    source_id: ObjectId,
    /// Source node type.
    source_type: ConnectedNodeType,
    /// Destination object ID.
    destination_id: ObjectId,
    /// Destination node type.
    destination_type: ConnectedNodeType,
    /// Label.
    label: Option<ConnectionLabelSym>,
    /// Connection node index.
    index: ConnectionIndex,
}

impl Connection {
    /// Creates a new `Connection`.
    pub(crate) fn new(
        source_id: ObjectId,
        source_type: ConnectedNodeType,
        destination_id: ObjectId,
        destination_type: ConnectedNodeType,
        label: Option<ConnectionLabelSym>,
        index: ConnectionIndex,
    ) -> Self {
        Self {
            source_id,
            source_type,
            destination_id,
            destination_type,
            label,
            index,
        }
    }

    /// Returns source ID.
    pub(crate) fn source_id(&self) -> ObjectId {
        self.source_id
    }

    /// Returns destination ID.
    pub(crate) fn destination_id(&self) -> ObjectId {
        self.destination_id
    }

    /// Returns label symbol.
    pub(crate) fn label_sym(&self) -> Option<ConnectionLabelSym> {
        self.label
    }

    /// Returns connection index.
    pub(crate) fn index(&self) -> ConnectionIndex {
        self.index
    }
}
