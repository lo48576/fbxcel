//! Types and functions for all supported versions.

use std::io::{Read, Seek};

use log::warn;

pub use self::error::{Error, Result};
use crate::{
    low::{self, FbxVersion},
    pull_parser::{self, any::AnyParser},
    tree,
};

mod error;

/// FBX tree type with any supported version.
#[non_exhaustive]
pub enum AnyTree {
    /// FBX 7.4 or later.
    V7400(
        FbxVersion,
        tree::v7400::Tree,
        std::result::Result<Box<low::v7400::FbxFooter>, pull_parser::Error>,
    ),
}

impl AnyTree {
    /// Loads a tree from the given reader.
    ///
    /// This works for seekable readers (which implement [`std::io::Seek`]), but
    /// [`from_seekable_reader`][`Self::from_seekable_reader`] should be used for them, because it is more
    /// efficent.
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        match pull_parser::any::from_reader(reader)? {
            AnyParser::V7400(mut parser) => {
                let fbx_version = parser.fbx_version();
                parser.set_warning_handler(|w, pos| {
                    warn!("WARNING: {} (pos={:?})", w, pos);
                    Ok(())
                });
                let tree_loader = tree::v7400::Loader::new();
                let (tree, footer) = tree_loader.load(&mut parser)?;
                Ok(AnyTree::V7400(fbx_version, tree, footer))
            }
        }
    }

    /// Loads a tree from the given seekable reader.
    pub fn from_seekable_reader(reader: impl Read + Seek) -> Result<Self> {
        match pull_parser::any::from_seekable_reader(reader)? {
            AnyParser::V7400(mut parser) => {
                let fbx_version = parser.fbx_version();
                parser.set_warning_handler(|w, pos| {
                    warn!("WARNING: {} (pos={:?})", w, pos);
                    Ok(())
                });
                let tree_loader = tree::v7400::Loader::new();
                let (tree, footer) = tree_loader.load(&mut parser)?;
                Ok(AnyTree::V7400(fbx_version, tree, footer))
            }
        }
    }

    /// Returns the FBX version of the document the tree came from.
    #[inline]
    #[must_use]
    pub fn fbx_version(&self) -> FbxVersion {
        match self {
            Self::V7400(ver, _, _) => *ver,
        }
    }
}
