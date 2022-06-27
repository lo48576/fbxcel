//! FBX parser version types.

use log::info;

use crate::low::FbxVersion;

/// Parser version for each version of FBX.
///
/// Some parser supports multiple versions of FBX binary.
/// Each variants of this type corresponds to a parser implementation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum ParserVersion {
    /// FBX 7.4 and 7.5.
    V7400,
}

impl ParserVersion {
    /// Returns the parser version corresponding to the given FBX version.
    #[must_use]
    pub fn from_fbx_version(fbx_version: FbxVersion) -> Option<Self> {
        let raw = fbx_version.raw();
        match raw {
            7000..=7999 => {
                if raw < 7400 {
                    info!("<FBX-7.4 might be successfully read, but unsupported");
                } else if raw > 7500 {
                    info!(">FBX-7.5 might be successfully read, but unsupported");
                }
                Some(ParserVersion::V7400)
            }
            _ => None,
        }
    }
}
