//! Direct attribute value loader.

use std::io;

use crate::{
    low::v7400::AttributeValue,
    pull_parser::{v7400::LoadAttribute, Result},
};

/// Loader for [`AttributeValue`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DirectLoader;

impl LoadAttribute for DirectLoader {
    type Output = AttributeValue;

    #[inline]
    fn expecting(&self) -> String {
        "any type".into()
    }

    #[inline]
    fn load_bool(self, v: bool) -> Result<Self::Output> {
        Ok(AttributeValue::Bool(v))
    }

    #[inline]
    fn load_i16(self, v: i16) -> Result<Self::Output> {
        Ok(AttributeValue::I16(v))
    }

    #[inline]
    fn load_i32(self, v: i32) -> Result<Self::Output> {
        Ok(AttributeValue::I32(v))
    }

    #[inline]
    fn load_i64(self, v: i64) -> Result<Self::Output> {
        Ok(AttributeValue::I64(v))
    }

    #[inline]
    fn load_f32(self, v: f32) -> Result<Self::Output> {
        Ok(AttributeValue::F32(v))
    }

    #[inline]
    fn load_f64(self, v: f64) -> Result<Self::Output> {
        Ok(AttributeValue::F64(v))
    }

    #[inline]
    fn load_seq_bool(
        self,
        iter: impl Iterator<Item = Result<bool>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeValue::ArrBool(iter.collect::<Result<_>>()?))
    }

    #[inline]
    fn load_seq_i32(
        self,
        iter: impl Iterator<Item = Result<i32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeValue::ArrI32(iter.collect::<Result<_>>()?))
    }

    #[inline]
    fn load_seq_i64(
        self,
        iter: impl Iterator<Item = Result<i64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeValue::ArrI64(iter.collect::<Result<_>>()?))
    }

    #[inline]
    fn load_seq_f32(
        self,
        iter: impl Iterator<Item = Result<f32>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeValue::ArrF32(iter.collect::<Result<_>>()?))
    }

    #[inline]
    fn load_seq_f64(
        self,
        iter: impl Iterator<Item = Result<f64>>,
        _len: usize,
    ) -> Result<Self::Output> {
        Ok(AttributeValue::ArrF64(iter.collect::<Result<_>>()?))
    }

    #[inline]
    fn load_binary(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = Vec::with_capacity(len as usize);
        reader.read_to_end(&mut buf)?;
        Ok(AttributeValue::Binary(buf))
    }

    #[inline]
    fn load_string(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = String::with_capacity(len as usize);
        reader.read_to_string(&mut buf)?;
        Ok(AttributeValue::String(buf))
    }
}
