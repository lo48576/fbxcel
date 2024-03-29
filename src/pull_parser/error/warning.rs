//! Invalid operation.

use std::{error, fmt};

/// Warning.
#[derive(Debug)]
#[non_exhaustive]
pub enum Warning {
    /// Node name is empty.
    EmptyNodeName,
    /// Extra (unexpected) node end marker found.
    ExtraNodeEndMarker,
    /// Incorrect boolean representation.
    ///
    /// Boolean value in node attributes should be some prescribed value
    /// (for example `b'T'` and `b'Y'` for FBX 7.4).
    /// Official SDK and tools may emit those values correctly, but some
    /// third-party exporters emits them wrongly with `0x00` and `0x01`, and
    /// those will be ignored by official SDK and tools.
    IncorrectBooleanRepresentation,
    /// Footer padding length is invalid.
    InvalidFooterPaddingLength(usize, usize),
    /// Missing a node end marker where the marker is expected.
    MissingNodeEndMarker,
    /// Unexpected value for footer fields (mainly for unknown fields).
    UnexpectedFooterFieldValue,
}

impl error::Error for Warning {}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Warning::EmptyNodeName => write!(f, "Node name is empty"),
            Warning::ExtraNodeEndMarker => write!(f, "Extra (unexpected) node end marker found"),
            Warning::IncorrectBooleanRepresentation => {
                write!(f, "Incorrect boolean representation")
            }
            Warning::InvalidFooterPaddingLength(expected, got) => write!(
                f,
                "Invalid footer padding length: expected {} bytes, got {} bytes",
                expected, got
            ),
            Warning::MissingNodeEndMarker => write!(f, "Missing node end marker"),
            Warning::UnexpectedFooterFieldValue => write!(f, "Unexpected footer field value"),
        }
    }
}
