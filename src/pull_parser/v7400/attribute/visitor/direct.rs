//! Direct attribute value visitor.

use std::io;

use crate::pull_parser::v7400::attribute::DirectAttributeValue;
use crate::pull_parser::v7400::VisitAttribute;
use crate::pull_parser::Result;

/// Visitor for direct attribute value.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectVisitor;

impl VisitAttribute for DirectVisitor {
    type Output = DirectAttributeValue;

    fn expecting(&self) -> String {
        "any type".into()
    }

    fn visit_bool(self, v: bool) -> Result<Self::Output> {
        Ok(DirectAttributeValue::Bool(v))
    }

    fn visit_i16(self, v: i16) -> Result<Self::Output> {
        Ok(DirectAttributeValue::I16(v))
    }

    fn visit_i32(self, v: i32) -> Result<Self::Output> {
        Ok(DirectAttributeValue::I32(v))
    }

    fn visit_i64(self, v: i64) -> Result<Self::Output> {
        Ok(DirectAttributeValue::I64(v))
    }

    fn visit_f32(self, v: f32) -> Result<Self::Output> {
        Ok(DirectAttributeValue::F32(v))
    }

    fn visit_f64(self, v: f64) -> Result<Self::Output> {
        Ok(DirectAttributeValue::F64(v))
    }

    fn visit_seq_bool(
        self,
        iter: impl Iterator<Item = Result<bool>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(DirectAttributeValue::ArrBool(iter.collect::<Result<_>>()?))
    }

    fn visit_seq_i32(
        self,
        iter: impl Iterator<Item = Result<i32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(DirectAttributeValue::ArrI32(iter.collect::<Result<_>>()?))
    }

    fn visit_seq_i64(
        self,
        iter: impl Iterator<Item = Result<i64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(DirectAttributeValue::ArrI64(iter.collect::<Result<_>>()?))
    }

    fn visit_seq_f32(
        self,
        iter: impl Iterator<Item = Result<f32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(DirectAttributeValue::ArrF32(iter.collect::<Result<_>>()?))
    }

    fn visit_seq_f64(
        self,
        iter: impl Iterator<Item = Result<f64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(DirectAttributeValue::ArrF64(iter.collect::<Result<_>>()?))
    }

    fn visit_binary(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = Vec::with_capacity(len as usize);
        reader.read_to_end(&mut buf)?;
        Ok(DirectAttributeValue::Binary(buf))
    }

    fn visit_string(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = String::with_capacity(len as usize);
        reader.read_to_string(&mut buf)?;
        Ok(DirectAttributeValue::String(buf))
    }
}
