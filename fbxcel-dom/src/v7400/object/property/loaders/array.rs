//! Property loaders for arrays.

use failure::format_err;

use crate::v7400::object::property::{loaders::check_attrs_len, LoadProperty, PropertyHandle};

macro_rules! impl_basic_methods {
    ($ty_loader:ty) => {
        impl $ty_loader {
            /// Creates a new loader.
            pub fn new() -> Self {
                Self::default()
            }
        }
    };
}

macro_rules! load_f64_arr {
    (@elem, $node:expr, $value_part:expr, $target_name:expr, $index:expr) => {
        $value_part[$index]
            .get_f64_or_type()
            .map_err(|ty| prop_type_err!($target_name, ty, $node))?
    };
    (2, $node:expr, $value_part:expr, $target_name:expr) => {
        [
            load_f64_arr!(@elem, $node, $value_part, $target_name, 0),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 1),
        ]
    };
    (3, $node:expr, $value_part:expr, $target_name:expr) => {
        [
            load_f64_arr!(@elem, $node, $value_part, $target_name, 0),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 1),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 2),
        ]
    };
    (4, $node:expr, $value_part:expr, $target_name:expr) => {
        [
            load_f64_arr!(@elem, $node, $value_part, $target_name, 0),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 1),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 2),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 3),
        ]
    };
    (16, $node:expr, $value_part:expr, $target_name:expr) => {
        [
            load_f64_arr!(@elem, $node, $value_part, $target_name, 0),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 1),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 2),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 3),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 4),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 5),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 6),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 7),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 8),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 9),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 10),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 11),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 12),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 13),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 14),
            load_f64_arr!(@elem, $node, $value_part, $target_name, 15),
        ]
    };
}
macro_rules! impl_f64_arr_loader {
    ($ty_loader:ty, $len:tt) => {
        impl_basic_methods! { $ty_loader }

        impl LoadProperty<'_> for $ty_loader {
            type Value = [f64; $len];
            type Error = failure::Error;

            fn expecting(&self) -> String {
                concat!("`[f64; ", $len, "]`").into()
            }

            fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
                /// Type name to use in error message.
                const TARGET_NAME: &str = concat!("`[f64; ", $len, "]`");

                let value_part = check_attrs_len(node, $len, TARGET_NAME)?;

                Ok(load_f64_arr!($len, node, value_part, TARGET_NAME))
            }
        }
    };
}

/// Property loader for `[f64; 2]` value.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects `[f32; 2]`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct F64Arr2Loader;

impl_f64_arr_loader! { F64Arr2Loader, 2 }

/// Property loader for `[f64; 3]` value.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects `[f32; 3]`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct F64Arr3Loader;

impl_f64_arr_loader! { F64Arr3Loader, 3 }

/// Property loader for `[f64; 4]` value.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects `[f32; 4]`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct F64Arr4Loader;

impl_f64_arr_loader! { F64Arr4Loader, 4 }

/// Property loader for `[f64; 16]` value.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects `[f32; 16]`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct F64Arr16Loader;

impl_f64_arr_loader! { F64Arr16Loader, 16 }
