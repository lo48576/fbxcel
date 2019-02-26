//! `Connections` and `C` node.

use log::{trace, warn};
use string_interner::StringInterner;

use crate::dom::error::{LoadError, StructureError};
use crate::dom::v7400::object::ObjectId;
use crate::dom::v7400::{Core, StrSym};
use crate::pull_parser::v7400::attribute::DirectAttributeValue;

/// Type of a connected node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectedNodeType {
    /// Object.
    Object,
    /// Property.
    Property,
}

/// Connection edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionEdge {
    /// Source node type.
    source_type: ConnectedNodeType,
    /// Destination node type.
    destination_type: ConnectedNodeType,
    /// Label.
    label: Option<StrSym>,
    /// Connection node index.
    index: usize,
}

impl ConnectionEdge {
    /// Returns source node type.
    pub fn source_type(&self) -> ConnectedNodeType {
        self.source_type
    }

    /// Returns destination node type.
    pub fn destination_type(&self) -> ConnectedNodeType {
        self.destination_type
    }

    /// Returns label.
    pub fn label<'a>(&self, core: &'a impl AsRef<Core>) -> Option<&'a str> {
        self.label.map(|label| {
            core.as_ref()
                .string(label)
                .expect("The string symbol is not registered in the document")
        })
    }

    /// Connection node index.
    pub fn index(&self) -> usize {
        self.index
    }
}

/// Connection data (provided by `C` node).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Connection {
    /// Edge data.
    edge: ConnectionEdge,
    /// Source object ID.
    source_id: ObjectId,
    /// Destination object ID.
    destination_id: ObjectId,
}

impl Connection {
    /// Returns source ID.
    pub fn source_id(&self) -> ObjectId {
        self.source_id
    }

    /// Returns destination ID.
    pub fn destination_id(&self) -> ObjectId {
        self.destination_id
    }

    /// Returns connection edge.
    pub fn edge(&self) -> &ConnectionEdge {
        &self.edge
    }

    /// Loads `Connection` from the given `C` node attributes.
    pub(crate) fn load_from_attributes(
        attrs: &[DirectAttributeValue],
        strings: &mut StringInterner<StrSym>,
        conn_index: usize,
    ) -> Result<Self, LoadError> {
        trace!("Loading `C` node: conn_index={:?}", conn_index);

        let ty_str = match attrs.get(0) {
            Some(DirectAttributeValue::String(v)) => v,
            Some(v) => {
                warn!(
                    "Invalid attribute[0] for `C`: expected string, got {:?}",
                    v.type_()
                );
                return Err(StructureError::unexpected_attribute_type(
                    &["Connection", "C"],
                    Some(0),
                    "string",
                    format!("{:?}", v.type_()),
                )
                .into());
            }
            None => {
                warn!("Attribute[0] not found for `C`: expected string");
                return Err(
                    StructureError::attribute_not_found(&["Connection", "C"], Some(0)).into(),
                );
            }
        };
        trace!("Got raw connection types value: {:?}", ty_str);
        let (destination_type, source_type) = match ty_str.as_ref() {
            "OO" => (ConnectedNodeType::Object, ConnectedNodeType::Object),
            "OP" => (ConnectedNodeType::Object, ConnectedNodeType::Property),
            "PO" => (ConnectedNodeType::Property, ConnectedNodeType::Object),
            "PP" => (ConnectedNodeType::Property, ConnectedNodeType::Property),
            v => {
                return Err(StructureError::unexpected_attribute_value(
                    &["Connection", "C"],
                    Some(0),
                    "connection type",
                    format!("{:?}", v),
                )
                .into());
            }
        };
        trace!(
            "Got connection types: dest={:?}, src={:?}",
            destination_type,
            source_type
        );

        let source_id = get_object_id_from_attrs(attrs, 1)?;
        trace!("Got source object ID: {:?}", source_id);
        let destination_id = get_object_id_from_attrs(attrs, 2)?;
        trace!("Got destination object ID: {:?}", destination_id);

        let label = match attrs.get(3) {
            Some(DirectAttributeValue::String(v)) => {
                trace!("Got connection label string: {:?}", v);
                Some(strings.get_or_intern(v.as_str()))
            }
            None => {
                trace!("No connection label found");
                None
            }
            Some(v) => {
                warn!(
                    "Invalid attribute[3] for `C`: expected optional string, but got {:?})",
                    v
                );
                return Err(StructureError::unexpected_attribute_type(
                    &["Connection", "C"],
                    Some(3),
                    "string or nothing",
                    format!("{:?}", v.type_()),
                )
                .into());
            }
        };

        trace!("Successfully loaded `C` node: conn_index={:?}", conn_index);

        Ok(Connection {
            edge: ConnectionEdge {
                source_type,
                destination_type,
                label,
                index: conn_index,
            },
            source_id,
            destination_id,
        })
    }
}

/// Parses the attribute into object ID.
fn get_object_id_from_attrs(
    attrs: &[DirectAttributeValue],
    index: usize,
) -> Result<ObjectId, LoadError> {
    match attrs.get(index) {
        Some(DirectAttributeValue::I64(v)) => Ok(ObjectId::new(*v)),
        Some(v) => {
            warn!(
                "Invalid attribute[{}] for `C`: expected i64, got {:?}",
                index,
                v.type_()
            );
            Err(StructureError::unexpected_attribute_type(
                &["Connections", "C"],
                Some(index),
                "`i64`",
                format!("{:?}", v.type_()),
            )
            .into())
        }
        None => {
            warn!("Attribute[{}] not found for `C`: expected i64", index);
            Err(StructureError::attribute_not_found(&["Connections", "C"], Some(index)).into())
        }
    }
}

/// Reference to connection data (provided by `C` node).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionRef<'a> {
    /// Edge data.
    edge: &'a ConnectionEdge,
    /// Source object ID.
    source_id: ObjectId,
    /// Destination object ID.
    destination_id: ObjectId,
}

impl<'a> ConnectionRef<'a> {
    /// Creates a new `ConnectionRef`.
    pub(crate) fn new(
        source_id: ObjectId,
        destination_id: ObjectId,
        edge: &'a ConnectionEdge,
    ) -> Self {
        Self {
            edge,
            source_id,
            destination_id,
        }
    }

    /// Returns source ID.
    pub fn source_id(&self) -> ObjectId {
        self.source_id
    }

    /// Returns destination ID.
    pub fn destination_id(&self) -> ObjectId {
        self.destination_id
    }

    /// Returns connection edge.
    pub fn edge(&self) -> &'a ConnectionEdge {
        &self.edge
    }
}
