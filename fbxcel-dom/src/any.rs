//! Types and functions for all supported versions.

use std::io::{Read, Seek};

use fbxcel::tree::any::AnyTree;

pub use self::error::{Error, Result};

mod error;

/// FBX tree type with any supported version.
pub enum AnyDocument {
    /// FBX 7.4 or later.
    V7400(Box<crate::v7400::Document>),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl AnyDocument {
    /// Loads a document from the given reader.
    ///
    /// This works for seekable readers (which implement `std::io::Seek`), but
    /// `from_seekable_reader` should be used for them, because it is more
    /// efficent.
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        match AnyTree::from_reader(reader)? {
            AnyTree::V7400(tree, _footer) => {
                let doc = crate::v7400::Loader::new().load_from_tree(tree)?;
                Ok(AnyDocument::V7400(Box::new(doc)))
            }
            AnyTree::__Nonexhaustive => unreachable!("`__Nonexhaustive` should never be used"),
        }
    }

    /// Loads a document from the given seekable reader.
    pub fn from_seekable_reader(reader: impl Read + Seek) -> Result<Self> {
        match AnyTree::from_seekable_reader(reader)? {
            AnyTree::V7400(tree, _footer) => {
                let doc = crate::v7400::Loader::new().load_from_tree(tree)?;
                Ok(AnyDocument::V7400(Box::new(doc)))
            }
            AnyTree::__Nonexhaustive => unreachable!("`__Nonexhaustive` should never be used"),
        }
    }
}
