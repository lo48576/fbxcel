//! Attribute type visitor.

use std::io;

use super::super::AttributeType;
use super::{Result, VisitAttribute};

/// Visitor for direct attribute value.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeVisitor;

impl VisitAttribute for TypeVisitor {
    type Output = AttributeType;

    fn expecting(&self) -> String {
        "any type".into()
    }

    fn visit_bool(self, _: bool) -> Result<Self::Output> {
        Ok(AttributeType::Bool)
    }

    fn visit_i16(self, _: i16) -> Result<Self::Output> {
        Ok(AttributeType::I16)
    }

    fn visit_i32(self, _: i32) -> Result<Self::Output> {
        Ok(AttributeType::I32)
    }

    fn visit_i64(self, _: i64) -> Result<Self::Output> {
        Ok(AttributeType::I64)
    }

    fn visit_f32(self, _: f32) -> Result<Self::Output> {
        Ok(AttributeType::F32)
    }

    fn visit_f64(self, _: f64) -> Result<Self::Output> {
        Ok(AttributeType::F64)
    }

    fn visit_seq_bool(
        self,
        _: impl Iterator<Item = Result<bool>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrBool)
    }

    fn visit_seq_i32(
        self,
        _: impl Iterator<Item = Result<i32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrI32)
    }

    fn visit_seq_i64(
        self,
        _: impl Iterator<Item = Result<i64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrI64)
    }

    fn visit_seq_f32(
        self,
        _: impl Iterator<Item = Result<f32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrF32)
    }

    fn visit_seq_f64(
        self,
        _: impl Iterator<Item = Result<f64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrF64)
    }

    fn visit_binary(self, _: impl io::Read, _len: u64) -> Result<Self::Output> {
        Ok(AttributeType::Binary)
    }

    fn visit_string(self, _: impl io::Read, _len: u64) -> Result<Self::Output> {
        Ok(AttributeType::String)
    }
}
