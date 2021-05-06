//! Low-level or primitive data types for FBX binary.

#[cfg(feature = "writer")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "writer")))]
pub(crate) use self::fbx_header::MAGIC;
pub use self::{
    fbx_header::{FbxHeader, HeaderError},
    version::FbxVersion,
};

mod fbx_header;
pub mod v7400;
mod version;
