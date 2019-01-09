//! FBX DOM for FBX 7.4 (or compatible versions).

pub(crate) use self::core::Core;
pub use self::document::{Document, Loader};
pub(crate) use self::node::{IntoRawNodeId, NodeData, StrSym};
pub use self::node::{Node, NodeId};

mod connection;
mod core;
mod document;
mod node;
pub mod object;
