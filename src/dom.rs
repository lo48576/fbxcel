//! FBX DOM.
//!
//! This module is enabled by `dom` feature.

pub use self::error::{AccessError, LoadError};

mod error;
pub mod v7400;
