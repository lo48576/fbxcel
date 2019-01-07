//! Node attribute iterators.

use std::io;

use crate::pull_parser::v7400::attribute::visitor::VisitAttribute;
use crate::pull_parser::v7400::attribute::Attributes;
use crate::pull_parser::{ParserSource, Result};

/// Creates size hint from the given attributes and visitors.
fn make_size_hint_for_attrs<R, V>(
    attributes: &Attributes<R>,
    visitors: &impl Iterator<Item = V>,
) -> (usize, Option<usize>)
where
    R: ParserSource,
    V: VisitAttribute,
{
    let (visitors_min, visitors_max) = visitors.size_hint();
    let attrs_rest = attributes.rest_count() as usize;
    let min = std::cmp::min(attrs_rest, visitors_min);
    let max = visitors_max.map_or_else(usize::max_value, |v| std::cmp::min(attrs_rest, v));

    (min, Some(max))
}

/// Visits the next attrbute.
fn visit_next<R, V>(
    attributes: &mut Attributes<R>,
    visitors: &mut impl Iterator<Item = V>,
) -> Option<Result<V::Output>>
where
    R: ParserSource,
    V: VisitAttribute,
{
    let visitor = visitors.next()?;

    // TODO: Use `transpose` when it is stabilized.
    // See <https://github.com/rust-lang/rust/issues/47338> for detail.
    match attributes.visit_next(visitor) {
        Ok(Some(v)) => Some(Ok(v)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

/// Visits the next attrbute with buffered I/O.
fn visit_next_buffered<R, V>(
    attributes: &mut Attributes<R>,
    visitors: &mut impl Iterator<Item = V>,
) -> Option<Result<V::Output>>
where
    R: ParserSource + io::BufRead,
    V: VisitAttribute,
{
    let visitor = visitors.next()?;

    // TODO: Use `transpose` when it is stabilized.
    // See <https://github.com/rust-lang/rust/issues/47338> for detail.
    match attributes.visit_next_buffered(visitor) {
        Ok(Some(v)) => Some(Ok(v)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

/// Node attributes iterator.
#[derive(Debug)]
pub struct BorrowedIter<'a, 'r, R, I> {
    /// Attributes.
    attributes: &'a mut Attributes<'r, R>,
    /// Visitors.
    visitors: I,
}

impl<'a, 'r, R, I, V> BorrowedIter<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    /// Creates a new `Iter`.
    pub(crate) fn new(attributes: &'a mut Attributes<'r, R>, visitors: I) -> Self {
        Self {
            attributes,
            visitors,
        }
    }
}

impl<'a, 'r, R, I, V> Iterator for BorrowedIter<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    type Item = Result<V::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        visit_next(&mut self.attributes, &mut self.visitors)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(&self.attributes, &self.visitors)
    }
}

/// Node attributes iterator with buffered I/O.
#[derive(Debug)]
pub struct BorrowedIterBuffered<'a, 'r, R, I> {
    /// Attributes.
    attributes: &'a mut Attributes<'r, R>,
    /// Visitors.
    visitors: I,
}

impl<'a, 'r, R, I, V> BorrowedIterBuffered<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    /// Creates a new `IterBuffered`.
    pub(crate) fn new(attributes: &'a mut Attributes<'r, R>, visitors: I) -> Self {
        Self {
            attributes,
            visitors,
        }
    }
}

impl<'a, 'r, R, I, V> Iterator for BorrowedIterBuffered<'a, 'r, R, I>
where
    R: ParserSource + io::BufRead,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    type Item = Result<V::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        visit_next_buffered(&mut self.attributes, &mut self.visitors)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(&self.attributes, &self.visitors)
    }
}

/// Node attributes iterator.
#[derive(Debug)]
pub struct OwnedIter<'r, R, I> {
    /// Attributes.
    attributes: Attributes<'r, R>,
    /// Visitors.
    visitors: I,
}

impl<'r, R, I, V> OwnedIter<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    /// Creates a new `Iter`.
    pub(crate) fn new(attributes: Attributes<'r, R>, visitors: I) -> Self {
        Self {
            attributes,
            visitors,
        }
    }
}

impl<'r, R, I, V> Iterator for OwnedIter<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    type Item = Result<V::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        visit_next(&mut self.attributes, &mut self.visitors)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(&self.attributes, &self.visitors)
    }
}

/// Node attributes iterator with buffered I/O.
#[derive(Debug)]
pub struct OwnedIterBuffered<'r, R, I> {
    /// Attributes.
    attributes: Attributes<'r, R>,
    /// Visitors.
    visitors: I,
}

impl<'r, R, I, V> OwnedIterBuffered<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    /// Creates a new `IterBuffered`.
    pub(crate) fn new(attributes: Attributes<'r, R>, visitors: I) -> Self {
        Self {
            attributes,
            visitors,
        }
    }
}

impl<'r, R, I, V> Iterator for OwnedIterBuffered<'r, R, I>
where
    R: ParserSource + io::BufRead,
    I: Iterator<Item = V>,
    V: VisitAttribute,
{
    type Item = Result<V::Output>;

    fn next(&mut self) -> Option<Self::Item> {
        visit_next_buffered(&mut self.attributes, &mut self.visitors)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(&self.attributes, &self.visitors)
    }
}
