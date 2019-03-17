//! The excellent FBX library.
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

#[cfg(feature = "domcast")]
pub mod domcast;
pub mod low;
pub mod pull_parser;
#[cfg(feature = "tree")]
pub mod tree;
