//! Invalid operation.

use std::error;
use std::fmt;

/// Warning.
#[derive(Debug)]
pub enum Warning {
    /// Node name is empty.
    EmptyNodeName,
}

impl error::Error for Warning {}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Warning::EmptyNodeName => write!(f, "Node name is empty"),
        }
    }
}
