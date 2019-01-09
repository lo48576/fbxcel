//! Object.

use string_interner::StringInterner;

use crate::dom::v7400::{NodeId, StrSym};
use crate::dom::AccessError;
use crate::pull_parser::v7400::attribute::DirectAttributeValue;

/// Metadata of object node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ObjectMeta {
    /// Object ID.
    id: ObjectId,
    /// Name (if exists).
    name: Option<String>,
    /// Class.
    class: StrSym,
    /// Subclass.
    subclass: StrSym,
}

impl ObjectMeta {
    /// Returns ID.
    pub fn id(&self) -> ObjectId {
        self.id
    }

    /// Creates `ObjectMeta` from the given attributes.
    pub(crate) fn from_attributes(
        attrs: &[DirectAttributeValue],
        strings: &mut StringInterner<StrSym>,
    ) -> Result<Self, AccessError> {
        // Get ID.
        let id = match attrs.get(0) {
            Some(DirectAttributeValue::I64(v)) => ObjectId::new(*v),
            Some(_) => return Err(AccessError::UnexpectedAttributeType(Some(0))),
            None => return Err(AccessError::AttributeNotFound(Some(0))),
        };

        // Get name and class.
        let (name, class) = match attrs.get(1) {
            Some(DirectAttributeValue::String(name_class)) => {
                name_class.find("\u{0}\u{1}").map_or_else(
                    || (None, ""),
                    |sep_pos| {
                        (
                            Some(name_class[0..sep_pos].to_owned()),
                            &name_class[sep_pos + 2..],
                        )
                    },
                )
            }
            Some(_) => return Err(AccessError::UnexpectedAttributeType(Some(1))),
            None => return Err(AccessError::AttributeNotFound(Some(1))),
        };
        let class = strings.get_or_intern(class);

        // Get subclass.
        let subclass = match attrs.get(2) {
            Some(DirectAttributeValue::String(v)) => strings.get_or_intern(v.as_ref()),
            Some(_) => return Err(AccessError::UnexpectedAttributeType(Some(2))),
            None => return Err(AccessError::AttributeNotFound(Some(2))),
        };

        Ok(Self {
            id,
            name,
            class,
            subclass,
        })
    }
}

/// Object node ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectNodeId(NodeId);

impl ObjectNodeId {
    /// Creates a new `ObjectNodeId`.
    pub(crate) fn new(id: NodeId) -> Self {
        ObjectNodeId(id)
    }
}

/// Object ID.
///
/// This is not objcet node ID.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObjectId(i64);

impl ObjectId {
    /// Creates a new `ObjectId`.
    pub(crate) fn new(v: i64) -> Self {
        ObjectId(v)
    }
}
