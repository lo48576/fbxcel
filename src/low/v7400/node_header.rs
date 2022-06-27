//! Node header.

use crate::pull_parser::{
    v7400::{FromParser, Parser},
    Error as ParserError, ParserSource,
};

/// Node header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct NodeHeader {
    /// End offset of the node.
    pub(crate) end_offset: u64,
    /// The number of the node attributes.
    pub(crate) num_attributes: u64,
    /// Length of the node attributes in bytes.
    pub(crate) bytelen_attributes: u64,
    /// Length of the node name in bytes.
    pub(crate) bytelen_name: u8,
}

impl NodeHeader {
    /// Checks whether the entry indicates end of a node.
    #[inline]
    #[must_use]
    pub(crate) fn is_node_end(&self) -> bool {
        self.end_offset == 0
            && self.num_attributes == 0
            && self.bytelen_attributes == 0
            && self.bytelen_name == 0
    }

    /// Returns node end marker.
    #[cfg(feature = "writer")]
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "writer")))]
    #[inline]
    #[must_use]
    pub(crate) fn node_end() -> Self {
        Self {
            end_offset: 0,
            num_attributes: 0,
            bytelen_attributes: 0,
            bytelen_name: 0,
        }
    }
}

impl FromParser for NodeHeader {
    fn read_from_parser<R>(parser: &mut Parser<R>) -> Result<Self, ParserError>
    where
        R: ParserSource,
    {
        let (end_offset, num_attributes, bytelen_attributes) = if parser.fbx_version().raw() < 7500
        {
            let eo = u64::from(parser.parse::<u32>()?);
            let na = u64::from(parser.parse::<u32>()?);
            let ba = u64::from(parser.parse::<u32>()?);
            (eo, na, ba)
        } else {
            let eo = parser.parse::<u64>()?;
            let na = parser.parse::<u64>()?;
            let ba = parser.parse::<u64>()?;
            (eo, na, ba)
        };
        let bytelen_name = parser.parse::<u8>()?;

        Ok(Self {
            end_offset,
            num_attributes,
            bytelen_attributes,
            bytelen_name,
        })
    }
}
