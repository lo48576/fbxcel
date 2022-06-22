//! Attribute type loader.

use std::io;

use crate::{
    low::v7400::AttributeType,
    pull_parser::{v7400::LoadAttribute, Result},
};

/// Loader for node attribute type ([`AttributeType`]).
///
/// This returns only node attribute type ([`AttributeType`]) and discands
/// its real value.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeLoader;

impl LoadAttribute for TypeLoader {
    type Output = AttributeType;

    fn expecting(&self) -> String {
        "any type".into()
    }

    fn load_bool(self, _: bool) -> Result<Self::Output> {
        Ok(AttributeType::Bool)
    }

    fn load_i16(self, _: i16) -> Result<Self::Output> {
        Ok(AttributeType::I16)
    }

    fn load_i32(self, _: i32) -> Result<Self::Output> {
        Ok(AttributeType::I32)
    }

    fn load_i64(self, _: i64) -> Result<Self::Output> {
        Ok(AttributeType::I64)
    }

    fn load_f32(self, _: f32) -> Result<Self::Output> {
        Ok(AttributeType::F32)
    }

    fn load_f64(self, _: f64) -> Result<Self::Output> {
        Ok(AttributeType::F64)
    }

    fn load_seq_bool(
        self,
        _: impl Iterator<Item = Result<bool>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrBool)
    }

    fn load_seq_i32(
        self,
        _: impl Iterator<Item = Result<i32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrI32)
    }

    fn load_seq_i64(
        self,
        _: impl Iterator<Item = Result<i64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrI64)
    }

    fn load_seq_f32(
        self,
        _: impl Iterator<Item = Result<f32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrF32)
    }

    fn load_seq_f64(
        self,
        _: impl Iterator<Item = Result<f64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeType::ArrF64)
    }

    fn load_binary(self, _: impl io::Read, _len: u64) -> Result<Self::Output> {
        Ok(AttributeType::Binary)
    }

    fn load_string(self, _: impl io::Read, _len: u64) -> Result<Self::Output> {
        Ok(AttributeType::String)
    }
}
