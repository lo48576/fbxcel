//! FBX footer.

/// FBX footer padding length.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FbxFooterPaddingLength {
    /// Default (correct) value.
    Default,
    /// Forced specified value, which can be wrong.
    Forced(u8),
}

impl Default for FbxFooterPaddingLength {
    fn default() -> Self {
        FbxFooterPaddingLength::Default
    }
}

/// FBX 7.4 footer.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FbxFooter<'a> {
    /// Unknown (semirandom) 16-bytes data.
    ///
    /// This field is expected to have prescribed upper 4 bits, i.e. the field
    /// is `fx bx ax 0x dx cx dx 6x bx 7x fx 8x 1x fx 2x 7x` if the FBX data is
    /// exported from official SDK.
    ///
    /// Note that third party exporter will use completely random data.
    pub unknown1: Option<&'a [u8; 16]>,
    /// Padding length.
    ///
    /// Padding is `padding_len` `0`s.
    /// `padding_len >= 0 && padding <= 15` should hold.
    ///
    /// Note that third party exporter will not use correct padding length.
    pub padding_len: FbxFooterPaddingLength,
    /// Unknown 4-bytes data.
    ///
    /// This is expected to be `[0u8; 4]`.
    pub unknown2: Option<[u8; 4]>,
    /// Unknown 16-bytes data.
    ///
    /// This is expected to be `[0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e,
    /// 0xec, 0xe9, 0x0c, 0xe3, 0x75, 0x8f, 0x29, 0x0b]`.
    pub unknown3: Option<&'a [u8; 16]>,
}

impl<'a> FbxFooter<'a> {
    /// Returns the first unknown field or default.
    pub(crate) fn unknown1(&self) -> &'a [u8; 16] {
        /// Default value.
        const DEFAULT: [u8; 16] = [
            0xf0, 0xb1, 0xa2, 0x03, 0xd4, 0xc5, 0xd6, 0x67, 0xb8, 0x79, 0xfa, 0x8b, 0x1c, 0xfd,
            0x2e, 0x7f,
        ];
        self.unknown1.unwrap_or(&DEFAULT)
    }

    /// Returns the second unknown field or default.
    pub(crate) fn unknown2(&self) -> [u8; 4] {
        /// Default value.
        const DEFAULT: [u8; 4] = [0; 4];
        self.unknown2.unwrap_or(DEFAULT)
    }

    /// Returns the third unknown field or default.
    pub(crate) fn unknown3(&self) -> &'a [u8; 16] {
        /// Default value.
        const DEFAULT: [u8; 16] = [
            0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c, 0xe3, 0x75, 0x8f,
            0x29, 0x0b,
        ];
        self.unknown3.unwrap_or(&DEFAULT)
    }
}
