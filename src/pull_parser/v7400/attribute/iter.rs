//! Node attribute iterators.

use std::io;
use std::iter;

use crate::pull_parser::{
    v7400::attribute::{loader::LoadAttribute, Attributes},
    ParserSource, Result,
};

/// Creates size hint from the given attributes and loaders.
#[must_use]
fn make_size_hint_for_attrs<R, V>(
    attributes: &Attributes<'_, R>,
    loaders: &impl Iterator<Item = V>,
) -> (usize, Option<usize>)
where
    R: ParserSource,
    V: LoadAttribute,
{
    let (loaders_min, loaders_max) = loaders.size_hint();
    let attrs_rest = attributes.rest_count() as usize;
    let min = std::cmp::min(attrs_rest, loaders_min);
    let max = loaders_max.map_or_else(usize::max_value, |v| std::cmp::min(attrs_rest, v));

    (min, Some(max))
}

/// Loads the next attrbute.
#[must_use]
fn load_next<R, V>(
    attributes: &mut Attributes<'_, R>,
    loaders: &mut impl Iterator<Item = V>,
) -> Option<Result<V::Output>>
where
    R: ParserSource,
    V: LoadAttribute,
{
    let loader = loaders.next()?;
    attributes.load_next(loader).transpose()
}

/// Loads the next attrbute with buffered I/O.
#[must_use]
fn load_next_buffered<R, V>(
    attributes: &mut Attributes<'_, R>,
    loaders: &mut impl Iterator<Item = V>,
) -> Option<Result<V::Output>>
where
    R: ParserSource + io::BufRead,
    V: LoadAttribute,
{
    let loader = loaders.next()?;
    attributes.load_next(loader).transpose()
}

/// Node attributes iterator.
#[derive(Debug)]
pub struct BorrowedIter<'a, 'r, R, I> {
    /// Attributes.
    attributes: &'a mut Attributes<'r, R>,
    /// Loaders.
    loaders: I,
}

impl<'a, 'r, R, I, V> BorrowedIter<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    /// Creates a new iterator.
    #[inline]
    #[must_use]
    pub(crate) fn new(attributes: &'a mut Attributes<'r, R>, loaders: I) -> Self {
        Self {
            attributes,
            loaders,
        }
    }
}

impl<'a, 'r, R, I, V> Iterator for BorrowedIter<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    type Item = Result<V::Output>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        load_next(self.attributes, &mut self.loaders)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(self.attributes, &self.loaders)
    }
}

impl<'a, 'r, R, I, V> iter::FusedIterator for BorrowedIter<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
}

/// Node attributes iterator with buffered I/O.
#[derive(Debug)]
pub struct BorrowedIterBuffered<'a, 'r, R, I> {
    /// Attributes.
    attributes: &'a mut Attributes<'r, R>,
    /// Loaders.
    loaders: I,
}

impl<'a, 'r, R, I, V> BorrowedIterBuffered<'a, 'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    /// Creates a new iterator.
    #[inline]
    #[must_use]
    pub(crate) fn new(attributes: &'a mut Attributes<'r, R>, loaders: I) -> Self {
        Self {
            attributes,
            loaders,
        }
    }
}

impl<'a, 'r, R, I, V> Iterator for BorrowedIterBuffered<'a, 'r, R, I>
where
    R: ParserSource + io::BufRead,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    type Item = Result<V::Output>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        load_next_buffered(self.attributes, &mut self.loaders)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(self.attributes, &self.loaders)
    }
}

impl<'a, 'r, R, I, V> iter::FusedIterator for BorrowedIterBuffered<'a, 'r, R, I>
where
    R: ParserSource + io::BufRead,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
}

/// Node attributes iterator.
#[derive(Debug)]
pub struct OwnedIter<'r, R, I> {
    /// Attributes.
    attributes: Attributes<'r, R>,
    /// Loaders.
    loaders: I,
}

impl<'r, R, I, V> OwnedIter<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    /// Creates a new `Iter`.
    #[inline]
    #[must_use]
    pub(crate) fn new(attributes: Attributes<'r, R>, loaders: I) -> Self {
        Self {
            attributes,
            loaders,
        }
    }
}

impl<'r, R, I, V> Iterator for OwnedIter<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    type Item = Result<V::Output>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        load_next(&mut self.attributes, &mut self.loaders)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(&self.attributes, &self.loaders)
    }
}

impl<'r, R, I, V> iter::FusedIterator for OwnedIter<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
}

/// Node attributes iterator with buffered I/O.
#[derive(Debug)]
pub struct OwnedIterBuffered<'r, R, I> {
    /// Attributes.
    attributes: Attributes<'r, R>,
    /// Loaders.
    loaders: I,
}

impl<'r, R, I, V> OwnedIterBuffered<'r, R, I>
where
    R: ParserSource,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    /// Creates a new iterator.
    #[inline]
    #[must_use]
    pub(crate) fn new(attributes: Attributes<'r, R>, loaders: I) -> Self {
        Self {
            attributes,
            loaders,
        }
    }
}

impl<'r, R, I, V> Iterator for OwnedIterBuffered<'r, R, I>
where
    R: ParserSource + io::BufRead,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
    type Item = Result<V::Output>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        load_next_buffered(&mut self.attributes, &mut self.loaders)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        make_size_hint_for_attrs(&self.attributes, &self.loaders)
    }
}

impl<'r, R, I, V> iter::FusedIterator for OwnedIterBuffered<'r, R, I>
where
    R: ParserSource + io::BufRead,
    I: Iterator<Item = V>,
    V: LoadAttribute,
{
}
