//! FBX DOM for FBX 7.4 (or compatible versions).

pub use self::document::{Document, Loader};
pub(crate) use self::node::NodeData;
pub(crate) use self::node::StrSym;
pub use self::node::{Node, NodeId};

mod document;
mod node;
