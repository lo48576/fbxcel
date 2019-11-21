//! The excellent FBX library.
//!
//! `low` module provides low-level data types such as FBX header, node
//! attribute value, etc.
//!
//! `pull_parser` module provides pull parser for FBX binary format.
//! ASCII format is not supported.
//!
//! `tree` module provides tree types, which allow users to access FBX data as
//! tree, not as stream of parser events.
//! To use `tree` module, enable `tree` feature.
//!
//! `writer` module provides writer types.
//! To use `writer` module, enable `writer` feature.
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod low;
pub mod pull_parser;
#[cfg(feature = "tree")]
pub mod tree;
#[cfg(feature = "writer")]
pub mod writer;
