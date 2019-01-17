//! FBX DOM for FBX 7.4 (or compatible versions).

pub(crate) use self::core::Core;
pub use self::document::{Document, Loader};
pub use self::node::NodeId;
pub(crate) use self::node::{IntoRawNodeId, StrSym};
pub use self::parsed::ParsedData;

mod connection;
mod core;
mod document;
pub mod node;
pub mod object;
mod parsed;
