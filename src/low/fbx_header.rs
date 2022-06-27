//! FBX binary header.

use std::{error, fmt, io};

use byteorder::{LittleEndian, ReadBytesExt};
use log::info;

use crate::{low::FbxVersion, pull_parser::ParserVersion};

/// Magic binary length.
const MAGIC_LEN: usize = 23;

/// Magic binary.
pub(crate) const MAGIC: &[u8; MAGIC_LEN] = b"Kaydara FBX Binary  \x00\x1a\x00";

/// Header read error.
#[derive(Debug)]
pub enum HeaderError {
    /// I/O error.
    Io(io::Error),
    /// Magic binary is not detected.
    MagicNotDetected,
}

impl error::Error for HeaderError {}

impl fmt::Display for HeaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HeaderError::Io(e) => e.fmt(f),
            HeaderError::MagicNotDetected => f.write_str("FBX magic binary is not detected"),
        }
    }
}

impl From<io::Error> for HeaderError {
    #[inline]
    fn from(e: io::Error) -> Self {
        HeaderError::Io(e)
    }
}

/// FBX binary header.
///
/// This type represents a binary header for all supported versions of FBX.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FbxHeader {
    /// FBX version.
    version: FbxVersion,
}

impl FbxHeader {
    /// Reads an FBX header from the given reader.
    pub fn load(mut reader: impl io::Read) -> Result<Self, HeaderError> {
        // Check magic.
        let mut magic_buf = [0u8; MAGIC_LEN];
        reader.read_exact(&mut magic_buf)?;
        if magic_buf != *MAGIC {
            return Err(HeaderError::MagicNotDetected);
        }

        // Read FBX version.
        let version = reader.read_u32::<LittleEndian>()?;
        info!("FBX header is detected, version={}", version);

        Ok(FbxHeader {
            version: FbxVersion::new(version),
        })
    }

    /// Returns FBX version.
    #[inline]
    #[must_use]
    pub fn version(self) -> FbxVersion {
        self.version
    }

    /// Returns FBX parser version.
    ///
    /// Returns `None` if no parser supports the FBX version.
    #[inline]
    #[must_use]
    pub fn parser_version(self) -> Option<ParserVersion> {
        ParserVersion::from_fbx_version(self.version())
    }

    /// Returns header length in bytes.
    #[inline]
    #[must_use]
    pub(crate) const fn len(self) -> usize {
        /// FBX version length.
        const VERSION_LEN: usize = 4;

        MAGIC_LEN + VERSION_LEN
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    #[test]
    fn header_ok() {
        let raw_header = b"Kaydara FBX Binary  \x00\x1a\x00\xe8\x1c\x00\x00";
        let mut cursor = Cursor::new(raw_header);
        let header = FbxHeader::load(cursor.by_ref()).expect("Should never fail");
        assert_eq!(
            header.version(),
            FbxVersion::new(7400),
            "Header and version should be detected correctly"
        );
        assert_eq!(
            cursor.position() as usize,
            raw_header.len(),
            "Header should be read completely"
        );
    }

    #[test]
    fn magic_ng() {
        let wrong_header = b"Kaydara FBX Binary  \x00\xff\x00\xe8\x1c\x00\x00";
        let mut cursor = Cursor::new(wrong_header);
        // `HeaderError` may contain `io::Error` and is not comparable.
        assert!(
            matches!(
                FbxHeader::load(cursor.by_ref()),
                Err(HeaderError::MagicNotDetected)
            ),
            "Invalid magic should be reported by `MagicNotDetected`"
        );
        assert!(
            (cursor.position() as usize) < wrong_header.len(),
            "Header should not be read too much if the magic is not detected"
        );
    }
}
