//! FBX DOM utils for FBX v7.4 or later.

pub use self::{
    document::{Document, Loader},
    error::LoadError,
};

mod document;
pub(crate) mod error;
pub mod object;
