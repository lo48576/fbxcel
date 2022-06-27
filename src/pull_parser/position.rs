//! Syntactic position.

/// Syntactic position.
///
/// This contains not only byte-position, but also additional information such
/// as node path and attribute index.
///
/// This type is implemented based on FBX 7.4 data structure, and may change in
/// future if FBX syntax has breaking changes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntacticPosition {
    /// Byte position.
    pub(crate) byte_pos: u64,
    /// Beginning byte position of the node or attribute.
    pub(crate) component_byte_pos: u64,
    /// Node path.
    ///
    /// This is a vector of pairs of node indices in siblings (i.e. the number
    /// of preceding siblings) and node names.
    pub(crate) node_path: Vec<(usize, String)>,
    /// Node attribute index (if the position points an attribute).
    pub(crate) attribute_index: Option<usize>,
}

impl SyntacticPosition {
    /// Returns the byte position.
    #[inline]
    #[must_use]
    pub fn byte_pos(&self) -> u64 {
        self.byte_pos
    }

    /// Returns the beginning byte position of the node or attribute.
    #[inline]
    #[must_use]
    pub fn component_byte_pos(&self) -> u64 {
        self.component_byte_pos
    }

    /// Returns the node path.
    ///
    /// This is a vector of pairs of node indices in siblings (i.e. the number
    /// of preceding siblings) and node names.
    #[inline]
    #[must_use]
    pub fn node_path(&self) -> &[(usize, String)] {
        &self.node_path
    }

    /// Returns the node attribute index (if the position points an attribute).
    #[inline]
    #[must_use]
    pub fn attribute_index(&self) -> Option<usize> {
        self.attribute_index
    }
}
