//! Single type visitor.

use std::io;

use super::{Result, VisitAttribute};

/// Visitor for primitive types.
///
/// Supported types are: [`bool`], [`i16`] , [`i32`], [`i64`], [`f32`], [`f64`].
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimitiveVisitor<T>(std::marker::PhantomData<T>);

/// Generates `VisitAttribute` implementations for `PrimitiveVisitor<T>`.
macro_rules! impl_visit_attribute_for_primitives {
    ($ty:ty, $method_name:ident, $expecting_type:expr) => {
        impl VisitAttribute for PrimitiveVisitor<$ty> {
            type Output = $ty;

            fn expecting(&self) -> String {
                $expecting_type.into()
            }

            fn $method_name(self, v: $ty) -> Result<Self::Output> {
                Ok(v)
            }
        }
    };
}

impl_visit_attribute_for_primitives!(bool, visit_bool, "single boolean");
impl_visit_attribute_for_primitives!(i16, visit_i16, "single i16");
impl_visit_attribute_for_primitives!(i32, visit_i32, "single i32");
impl_visit_attribute_for_primitives!(i64, visit_i64, "single i64");
impl_visit_attribute_for_primitives!(f32, visit_f32, "single f32");
impl_visit_attribute_for_primitives!(f64, visit_f64, "single f64");

/// Visitor for array types.
///
/// Supported types are: `Vec<{bool, i32, i64, f32, f64}>`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArrayVisitor<T>(std::marker::PhantomData<T>);

/// Generates `VisitAttribute` implementations for `PrimitiveVisitor<T>`.
macro_rules! impl_visit_attribute_for_arrays {
    ($ty:ty, $method_name:ident, $expecting_type:expr) => {
        impl VisitAttribute for ArrayVisitor<Vec<$ty>> {
            type Output = Vec<$ty>;

            fn expecting(&self) -> String {
                $expecting_type.into()
            }

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

impl_visit_attribute_for_arrays!(bool, visit_seq_bool, "boolean array");
impl_visit_attribute_for_arrays!(i32, visit_seq_i32, "i32 array");
impl_visit_attribute_for_arrays!(i64, visit_seq_i64, "i64 array");
impl_visit_attribute_for_arrays!(f32, visit_seq_f32, "f32 array");
impl_visit_attribute_for_arrays!(f64, visit_seq_f64, "f64 array");

/// Visitor for a binary.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BinaryVisitor;

impl VisitAttribute for BinaryVisitor {
    type Output = Vec<u8>;

    fn expecting(&self) -> String {
        "binary".into()
    }

    fn visit_binary(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = Vec::with_capacity(len as usize);
        reader.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

/// Visitor for a string.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StringVisitor;

impl VisitAttribute for StringVisitor {
    type Output = String;

    fn expecting(&self) -> String {
        "string".into()
    }

    fn visit_string(self, mut reader: impl io::Read, len: u64) -> Result<Self::Output> {
        let mut buf = String::with_capacity(len as usize);
        reader.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
