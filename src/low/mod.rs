//! Low-level or primitive data types for FBX binary.

pub use self::fbx_header::FbxHeader;
pub use self::version::FbxVersion;

mod fbx_header;
mod version;
