//! Single type loader.

use std::io;

use crate::pull_parser::{v7400::LoadAttribute, Result};

/// Loader for primitive types.
///
/// Supported types are: `bool`, `i16` , `i32`, `i64`, `f32`, and `f64`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveLoader<T>(std::marker::PhantomData<T>);

/// Generates `LoadAttribute` implementations for `PrimitiveLoader<T>`.
macro_rules! impl_load_attribute_for_primitives {
    ($ty:ty, $method_name:ident, $expecting_type:expr) => {
        impl LoadAttribute for PrimitiveLoader<$ty> {
            type Output = $ty;

            fn expecting(&self) -> String {
                $expecting_type.into()
            }

            #[inline]
            fn $method_name(self, v: $ty) -> Result<Self::Output> {
                Ok(v)
            }
        }
    };
}

impl_load_attribute_for_primitives!(bool, load_bool, "single boolean");
impl_load_attribute_for_primitives!(i16, load_i16, "single i16");
impl_load_attribute_for_primitives!(i32, load_i32, "single i32");
impl_load_attribute_for_primitives!(i64, load_i64, "single i64");
impl_load_attribute_for_primitives!(f32, load_f32, "single f32");
impl_load_attribute_for_primitives!(f64, load_f64, "single f64");

/// Loader for array types.
///
/// Supported types are: `Vec<{bool, i32, i64, f32, f64}>`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayLoader<T>(std::marker::PhantomData<T>);

/// Generates `LoadAttribute` implementations for `PrimitiveLoader<T>`.
macro_rules! impl_load_attribute_for_arrays {
    ($ty:ty, $method_name:ident, $expecting_type:expr) => {
        impl LoadAttribute for ArrayLoader<Vec<$ty>> {
            type Output = Vec<$ty>;

            fn expecting(&self) -> String {
                $expecting_type.into()
            }

            #[inline]
            fn $method_name(
                self,
                iter: impl Iterator<Item = Result<$ty>>,
                _: usize,
            ) -> Result<Self::Output> {
                iter.collect::<Result<_>>()
            }
        }
    };
}

impl_load_attribute_for_arrays!(bool, load_seq_bool, "boolean array");
impl_load_attribute_for_arrays!(i32, load_seq_i32, "i32 array");
impl_load_attribute_for_arrays!(i64, load_seq_i64, "i64 array");
impl_load_attribute_for_arrays!(f32, load_seq_f32, "f32 array");
impl_load_attribute_for_arrays!(f64, load_seq_f64, "f64 array");

/// Loader for a binary.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryLoader;

impl LoadAttribute for BinaryLoader {
    type Output = Vec<u8>;

    fn expecting(&self) -> String {
        "binary".into()
    }

    #[inline]
    fn load_binary(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = Vec::with_capacity(len as usize);
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

/// Loader for a string.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringLoader;

impl LoadAttribute for StringLoader {
    type Output = String;

    fn expecting(&self) -> String {
        "string".into()
    }

    #[inline]
    fn load_string(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = String::with_capacity(len as usize);
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
