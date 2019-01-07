//! Node attribute visitors.

use std::fmt;
use std::io;

use crate::pull_parser::error::DataError;
use crate::pull_parser::Result;

pub use self::direct::DirectVisitor;
pub use self::single::{ArrayVisitor, BinaryVisitor, PrimitiveVisitor, StringVisitor};
pub use self::type_::TypeVisitor;

mod direct;
mod single;
mod type_;

/// A trait for attribute visitor types.
pub trait VisitAttribute: Sized + fmt::Debug {
    /// Result type on successful read.
    type Output;

    /// Describes the expecting value.
    fn expecting(&self) -> String;

    /// Visit boolean value.
    fn visit_bool(self, _: bool) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "boolean".into()).into())
    }

    /// Visit `i16` value.
    fn visit_i16(self, _: i16) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i16".into()).into())
    }

    /// Visit `i32` value.
    fn visit_i32(self, _: i32) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i32".into()).into())
    }

    /// Visit `i64` value.
    fn visit_i64(self, _: i64) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i64".into()).into())
    }

    /// Visit `f32` value.
    fn visit_f32(self, _: f32) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f32".into()).into())
    }

    /// Visit `f64` value.
    fn visit_f64(self, _: f64) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f64".into()).into())
    }

    /// Visit boolean array.
    fn visit_seq_bool(
        self,
        _: impl Iterator<Item = Result<bool>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "boolean array".into()).into())
    }

    /// Visit `i32` array.
    fn visit_seq_i32(
        self,
        _: impl Iterator<Item = Result<i32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i32 array".into()).into())
    }

    /// Visit `i64` array.
    fn visit_seq_i64(
        self,
        _: impl Iterator<Item = Result<i64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "i64 array".into()).into())
    }

    /// Visit `f32` array.
    fn visit_seq_f32(
        self,
        _: impl Iterator<Item = Result<f32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f32 array".into()).into())
    }

    /// Visit `f64` array.
    fn visit_seq_f64(
        self,
        _: impl Iterator<Item = Result<f64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "f64 array".into()).into())
    }

    /// Visit binary value.
    ///
    /// This method should return error when the given reader returned error.
    fn visit_binary(self, _: impl io::Read, _len: u64) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "binary data".into()).into())
    }

    /// Visit binary value on buffered reader.
    ///
    /// This method should return error when the given reader returned error.
    fn visit_binary_buffered(self, reader: impl io::BufRead, len: u64) -> Result<Self::Output> {
        self.visit_binary(reader, len)
    }

    /// Visit string value.
    ///
    /// This method should return error when the given reader returned error.
    fn visit_string(self, _: impl io::Read, _len: u64) -> Result<Self::Output> {
        Err(DataError::UnexpectedAttribute(self.expecting(), "string data".into()).into())
    }

    /// Visit string value on buffered reader.
    ///
    /// This method should return error when the given reader returned error.
    fn visit_string_buffered(self, reader: impl io::BufRead, len: u64) -> Result<Self::Output> {
        self.visit_string(reader, len)
    }
}
