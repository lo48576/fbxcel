//! The excellent FBX library.
//!
//! [`low`] module provides low-level data types such as FBX header, node
//! attribute value, etc.
//!
//! [`pull_parser`] module provides pull parser for FBX binary format.
//! ASCII format is not supported.
//!
#![cfg_attr(feature = "tree", doc = "[`tree`] ")]
#![cfg_attr(not(feature = "tree"), doc = "`tree` ")]
//! module provides tree types, which allow users to access FBX data as
//! tree, not as stream of parser events.
//! To use `tree` module, enable `tree` feature.
//!
#![cfg_attr(feature = "writer", doc = "[`writer`] ")]
#![cfg_attr(not(feature = "writer"), doc = "`writer` ")]
//! module provides writer types.
//! To use `writer` module, enable `writer` feature.
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::missing_docs_in_private_items)]

pub mod low;
pub mod pull_parser;
#[cfg(feature = "tree")]
#[cfg_attr(docsrs, doc(cfg(feature = "tree")))]
pub mod tree;
#[cfg(feature = "writer")]
#[cfg_attr(docsrs, doc(cfg(feature = "writer")))]
pub mod writer;
