//! `mint` integration.
//!
//! Enabled by `mint` feature.

use std::marker::PhantomData;

use failure::format_err;
use mint;

use crate::v7400::object::property::{loaders::check_attrs_len, LoadProperty, PropertyHandle};

/// Mint type loader.
///
/// Enabled by `mint` feature.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// Note that `f32` and `f64` is **NOT** converted automatically by this loader.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MintLoader<T>(PhantomData<fn() -> T>);

impl<T> MintLoader<T> {
    /// Creates a new `MintLoader`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T> Default for MintLoader<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Clone for MintLoader<T> {
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<T> Copy for MintLoader<T> {}

macro_rules! read_nth_value {
    ($node:expr, $value_part:expr, $getter:ident, $target_name:expr, $index:expr) => {
        $value_part[$index]
            .$getter()
            .map_err(|ty| prop_type_err!($target_name, ty, $node))?
    };
}

macro_rules! load_mint_value {
    (vec<2>, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        mint::Vector2 {
            x: read_nth_value!($node, $value_part, $getter, $target_name, 0),
            y: read_nth_value!($node, $value_part, $getter, $target_name, 1),
        }
    };
    (vec<3>, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        mint::Vector3 {
            x: read_nth_value!($node, $value_part, $getter, $target_name, 0),
            y: read_nth_value!($node, $value_part, $getter, $target_name, 1),
            z: read_nth_value!($node, $value_part, $getter, $target_name, 2),
        }
    };
    (vec<4>, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        mint::Vector4 {
            x: read_nth_value!($node, $value_part, $getter, $target_name, 0),
            y: read_nth_value!($node, $value_part, $getter, $target_name, 1),
            z: read_nth_value!($node, $value_part, $getter, $target_name, 2),
            w: read_nth_value!($node, $value_part, $getter, $target_name, 3),
        }
    };
    (mat_col<16>, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        mint::ColumnMatrix4 {
            x: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 0),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 1),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 2),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 3),
            },
            y: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 4),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 5),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 6),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 7),
            },
            z: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 8),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 9),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 10),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 11),
            },
            w: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 12),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 13),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 14),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 15),
            },
        }
    };
    (mat_row<16>, $node:expr, $value_part:expr, $getter:ident, $target_name:expr) => {
        mint::RowMatrix4 {
            x: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 0),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 4),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 8),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 12),
            },
            y: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 1),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 5),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 9),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 13),
            },
            z: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 2),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 6),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 10),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 14),
            },
            w: mint::Vector4 {
                x: read_nth_value!($node, $value_part, $getter, $target_name, 3),
                y: read_nth_value!($node, $value_part, $getter, $target_name, 7),
                z: read_nth_value!($node, $value_part, $getter, $target_name, 11),
                w: read_nth_value!($node, $value_part, $getter, $target_name, 15),
            },
        }
    };
}

macro_rules! impl_loader {
    ($ty_elem:ty, $getter:ident, $kind:tt, $base:ident, $len:tt) => {
        impl_loader! {
            @impl,
            $ty_elem,
            $getter,
            $kind,
            $base,
            $len,
            concat!(
                "`mint::",
                stringify!($base),
                "<",
                stringify!($ty_target),
                ">`"
            )
        }
    };
    (@impl, $ty_elem:ty, $getter:ident, $kind:tt, $base:ident, $len:tt, $target_name:expr) => {
        impl LoadProperty<'_> for MintLoader<mint::$base<$ty_elem>> {
            type Value = mint::$base<$ty_elem>;
            type Error = failure::Error;

            fn expecting(&self) -> String {
                $target_name.into()
            }

            fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
                let value_part = check_attrs_len(node, $len, $target_name)?;
                Ok(load_mint_value!(
                    $kind<$len>,
                    node,
                    value_part,
                    $getter,
                    $target_name
                ))
            }
        }
    };
}

impl_loader! { f32, get_f32_or_type, vec, Vector2, 2 }
impl_loader! { f64, get_f64_or_type, vec, Vector2, 2 }
impl_loader! { f32, get_f32_or_type, vec, Vector3, 3 }
impl_loader! { f64, get_f64_or_type, vec, Vector3, 3 }
impl_loader! { f32, get_f32_or_type, vec, Vector4, 4 }
impl_loader! { f64, get_f64_or_type, vec, Vector4, 4 }
impl_loader! { f32, get_f32_or_type, mat_col, ColumnMatrix4, 16 }
impl_loader! { f64, get_f64_or_type, mat_col, ColumnMatrix4, 16 }
impl_loader! { f32, get_f32_or_type, mat_row, RowMatrix4, 16 }
impl_loader! { f64, get_f64_or_type, mat_row, RowMatrix4, 16 }
