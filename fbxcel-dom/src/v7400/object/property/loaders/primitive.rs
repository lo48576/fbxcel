//! Primitive property loaders.

use std::marker::PhantomData;

use failure::format_err;
use fbxcel::low::v7400::AttributeValue;

use crate::v7400::object::property::{loaders::check_attrs_len, LoadProperty, PropertyHandle};

/// Primitive type value loader.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader automatically does safe conversion, i.e. you can load `i32`
/// value from raw `i16` attribute.
///
/// Note that `f32` and `f64` will be implicitly converted in both directions
/// by this loader.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveLoader<T>(PhantomData<fn() -> T>);

impl<T> PrimitiveLoader<T> {
    /// Creates a new `PrimitiveLoader`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for PrimitiveLoader<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for PrimitiveLoader<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for PrimitiveLoader<T> {}

impl LoadProperty<'_> for PrimitiveLoader<bool> {
    type Value = bool;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "boolean".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "boolean")?;
        match value_part[0] {
            AttributeValue::Bool(v) => Ok(v),
            AttributeValue::I16(v) => Ok(v != 0),
            AttributeValue::I32(v) => Ok(v != 0),
            AttributeValue::I64(v) => Ok(v != 0),
            ref v => Err(prop_type_err!("boolean", v.type_(), node)),
        }
    }
}

impl LoadProperty<'_> for PrimitiveLoader<i16> {
    type Value = i16;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`i16`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`i16`")?;
        value_part[0]
            .get_i16_or_type()
            .map_err(|ty| prop_type_err!("`i16`", ty, node))
    }
}

impl LoadProperty<'_> for PrimitiveLoader<u16> {
    type Value = u16;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`u16`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`u16`")?;
        value_part[0]
            .get_i16_or_type()
            .map(|v| v as u16)
            .map_err(|ty| prop_type_err!("`u16`", ty, node))
    }
}

impl LoadProperty<'_> for PrimitiveLoader<i32> {
    type Value = i32;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`i32`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`i32`")?;
        match value_part[0] {
            AttributeValue::I16(v) => Ok(i32::from(v)),
            AttributeValue::I32(v) => Ok(v),
            ref v => Err(prop_type_err!("i32", v.type_(), node)),
        }
    }
}

impl LoadProperty<'_> for PrimitiveLoader<u32> {
    type Value = u32;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`u32`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`u32`")?;
        match value_part[0] {
            AttributeValue::I16(v) => Ok(i32::from(v) as u32),
            AttributeValue::I32(v) => Ok(v as u32),
            ref v => Err(prop_type_err!("u32", v.type_(), node)),
        }
    }
}

impl LoadProperty<'_> for PrimitiveLoader<i64> {
    type Value = i64;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`i64`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`i64`")?;
        match value_part[0] {
            AttributeValue::I16(v) => Ok(i64::from(v)),
            AttributeValue::I32(v) => Ok(i64::from(v)),
            AttributeValue::I64(v) => Ok(v),
            ref v => Err(prop_type_err!("i64", v.type_(), node)),
        }
    }
}

impl LoadProperty<'_> for PrimitiveLoader<u64> {
    type Value = u64;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`u64`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`u64`")?;
        match value_part[0] {
            AttributeValue::I16(v) => Ok(i64::from(v) as u64),
            AttributeValue::I32(v) => Ok(i64::from(v) as u64),
            AttributeValue::I64(v) => Ok(v as u64),
            ref v => Err(prop_type_err!("u64", v.type_(), node)),
        }
    }
}

impl LoadProperty<'_> for PrimitiveLoader<f32> {
    type Value = f32;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`f32`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`f32`")?;
        match value_part[0] {
            AttributeValue::F32(v) => Ok(v),
            AttributeValue::F64(v) => Ok(v as f32),
            ref v => Err(prop_type_err!("i64", v.type_(), node)),
        }
    }
}

impl LoadProperty<'_> for PrimitiveLoader<f64> {
    type Value = f64;
    type Error = failure::Error;

    fn expecting(&self) -> String {
        "`f64`".into()
    }

    fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
        let value_part = check_attrs_len(node, 1, "`f64`")?;
        match value_part[0] {
            AttributeValue::F32(v) => Ok(f64::from(v)),
            AttributeValue::F64(v) => Ok(v),
            ref v => Err(prop_type_err!("i64", v.type_(), node)),
        }
    }
}
