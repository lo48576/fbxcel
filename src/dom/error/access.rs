//! DOM access error.

use std::error;
use std::fmt;

/// Error on DOM access.
#[derive(Debug)]
pub enum AccessError {
    /// Attribute not found.
    AttributeNotFound(Option<usize>),
    /// Invalid node attribute type or value.
    InvalidNodeAttribute(Option<String>, Option<usize>),
    /// Target node not found.
    NodeNotFound(String),
    /// Unexpected attribute type.
    UnexpectedAttributeType(Option<usize>),
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessError::AttributeNotFound(None) => {
                write!(f, "Expected more attributes but not found")
            }
            AccessError::AttributeNotFound(Some(index)) => {
                write!(f, "Attribute (index={}) not found", index)
            }
            AccessError::InvalidNodeAttribute(name, index) => {
                f.write_str("Invalid node attribute")?;
                match (name, index) {
                    (Some(name), index) => write!(f, ": node={:?}, attr-index={:?}", name, index),
                    (None, Some(index)) => write!(f, ": attr-index={}", index),
                    (None, None) => Ok(()),
                }
            }
            AccessError::NodeNotFound(desc) => write!(f, "Node not found: {}", desc),
            AccessError::UnexpectedAttributeType(index) => {
                write!(f, "Unexpected node attribute type")?;
                if let Some(index) = index {
                    write!(f, " (index={})", index)?;
                }
                Ok(())
            }
        }
    }
}

impl error::Error for AccessError {}
