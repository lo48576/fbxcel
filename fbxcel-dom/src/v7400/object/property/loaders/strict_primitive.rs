//! Strict primitive property loaders.

use failure::format_err;

use crate::v7400::object::property::{loaders::check_attrs_len, LoadProperty, PropertyHandle};

/// Strict `f32` property loader.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects `f64`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrictF32Loader;

/// Strict `f64` property loader.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects `f32`.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StrictF64Loader;

macro_rules! impl_strict_float_loader {
    ($ty_loader:ty, $ty_target:ty, $getter:ident, $target_name_str:expr) => {
        impl $ty_loader {
            /// Creates a new loader.
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl LoadProperty<'_> for $ty_loader {
            type Value = $ty_target;
            type Error = failure::Error;

            fn expecting(&self) -> String {
                $target_name_str.into()
            }

            fn load(self, node: &PropertyHandle<'_>) -> Result<Self::Value, Self::Error> {
                let value_part = check_attrs_len(node, 1, $target_name_str)?;
                value_part[0]
                    .$getter()
                    .map_err(|ty| prop_type_err!($target_name_str, ty, node))
            }
        }
    };
}

impl_strict_float_loader! { StrictF32Loader, f32, get_f32_or_type, "strict `f32`" }
impl_strict_float_loader! { StrictF64Loader, f64, get_f64_or_type, "strict `f64`" }
