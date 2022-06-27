//! FBX version type.

/// FBX version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FbxVersion(u32);

impl FbxVersion {
    /// Version 7.4.
    pub const V7_4: Self = FbxVersion(7400);

    /// Version 7.5.
    pub const V7_5: Self = FbxVersion(7500);

    /// Creates a new `FbxVersion`.
    #[inline]
    #[must_use]
    pub(crate) const fn new(version: u32) -> Self {
        FbxVersion(version)
    }

    /// Returns the raw value.
    ///
    /// For example, `7400` for FBX 7.4.
    #[inline]
    #[must_use]
    pub(crate) const fn raw(self) -> u32 {
        self.0
    }

    /// Returns the major version.
    ///
    /// For example, `7` for FBX 7.4.
    #[inline]
    #[must_use]
    pub const fn major(self) -> u32 {
        self.raw() / 1000
    }

    /// Returns the minor version.
    ///
    /// For example, `4` for FBX 7.4.
    #[inline]
    #[must_use]
    pub const fn minor(self) -> u32 {
        (self.raw() % 1000) / 100
    }

    /// Returns a tuple of the major and minor verison.
    ///
    /// For example, `(7, 4)` for FBX 7.4.
    #[inline]
    #[must_use]
    pub const fn major_minor(self) -> (u32, u32) {
        let major = self.major();
        let minor = self.minor();
        (major, minor)
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
