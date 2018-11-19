//! Node attributes.

use std::num::NonZeroU64;

use super::{Parser, ParserSource};

/// Node attributes reader.
#[derive(Debug)]
pub struct Attributes<'a, R: 'a> {
    /// Total number of attributes of the current node.
    total_count: u64,
    /// Rest number of attributes of the current node.
    rest_count: u64,
    /// End offset of the previous attribute, if available.
    ///
    /// "End offset" means a next byte of the last byte of the previous
    /// attribute.
    prev_attr_end_offset: Option<NonZeroU64>,
    /// Parser.
    parser: &'a mut Parser<R>,
}

impl<'a, R: 'a + ParserSource> Attributes<'a, R> {
    /// Creates a new `Attributes`.
    pub(crate) fn from_parser(parser: &'a mut Parser<R>) -> Self {
        let total_count = parser.current_attributes_count();
        Self {
            total_count,
            rest_count: total_count,
            prev_attr_end_offset: None,
            parser,
        }
    }

    /// Returns the total number of attributes.
    pub fn total_count(&self) -> u64 {
        self.total_count
    }

    /// Returns the rest number of attributes.
    pub fn rest_count(&self) -> u64 {
        self.rest_count
    }
}
