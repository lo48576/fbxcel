//! FBX and parser version types.

use log::info;

/// Parser version for each version of FBX.
///
/// Some parser supports multiple versions of FBX binary.
/// Variants of this type corresponds to parser version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParserVersion {
    /// FBX 7.4 and 7.5.
    V7400,
}

/// FBX version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FbxVersion(u32);

impl FbxVersion {
    /// Creates a new `FbxVersion`.
    pub(crate) fn new(version: u32) -> Self {
        FbxVersion(version)
    }

    /// Returns the raw value.
    ///
    /// For example, `7400` for FBX 7.4.
    pub(crate) fn raw(self) -> u32 {
        self.0
    }

    /// Returns the major version.
    ///
    /// For example, `7` for FBX 7.4.
    pub fn major(self) -> u32 {
        self.raw() / 1000
    }

    /// Returns the minor version.
    ///
    /// For example, `4` for FBX 7.4.
    pub fn minor(self) -> u32 {
        (self.raw() % 1000) / 100
    }

    /// Returns the major and minor verison.
    ///
    /// For example, `(7, 4)` for FBX 7.4.
    pub fn major_minor(self) -> (u32, u32) {
        let major = self.major();
        let minor = self.minor();
        (major, minor)
    }

    /// Returns the corresponding parser version.
    pub fn parser_version(self) -> Option<ParserVersion> {
        let raw = self.raw();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let major = 7;
        let minor = 4;
        let raw = major * 1000 + minor * 100;
        let ver = FbxVersion(raw);
        assert_eq!(ver.raw(), raw, "Should return raw value");
        assert_eq!(ver.major(), major, "Should return major version");
        assert_eq!(ver.minor(), minor, "Should return minor version");
        assert_eq!(
            ver.major_minor(),
            (major, minor),
            "Should return major and minor version"
        );
    }
}
