//! Property loaders for binary, and string.

use failure::format_err;

use crate::v7400::object::property::{loaders::check_attrs_len, LoadProperty, PropertyHandle};

/// Binary property loader that loads owned data.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects binary property even if the content is valid string.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedBinaryLoader;

/// String property loader that loads owned data.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnedStringLoader;

macro_rules! impl_owned_loader {
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
                    .map(Into::into)
                    .map_err(|ty| prop_type_err!($target_name_str, ty, node))
            }
        }
    };
}

impl_owned_loader! { OwnedBinaryLoader, Vec<u8>, get_binary_or_type, "binary" }
impl_owned_loader! { OwnedStringLoader, String, get_string_or_type, "string" }

/// Binary property loader that loads borrowed data.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
///
/// This loader rejects binary property even if the content is valid string.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BorrowedBinaryLoader;

/// String property loader that loads borrowed data.
///
/// This does minimal checks about `data_type` and `label`.
/// If you want to check property type precisely, you should make another
/// loader type by purpose.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BorrowedStringLoader;

macro_rules! impl_borrowed_loader {
    ($ty_loader:ty, $ty_target:ty, $getter:ident, $target_name_str:expr) => {
        impl $ty_loader {
            /// Creates a new loader.
            pub fn new() -> Self {
                Self::default()
            }
        }

        impl<'a> LoadProperty<'a> for $ty_loader {
            type Value = &'a $ty_target;
            type Error = failure::Error;

            fn expecting(&self) -> String {
                $target_name_str.into()
            }

            fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
                let value_part = check_attrs_len(node, 1, $target_name_str)?;
                value_part[0]
                    .$getter()
                    .map_err(|ty| prop_type_err!($target_name_str, ty, node))
            }
        }
    };
}

impl_borrowed_loader! { BorrowedBinaryLoader, [u8], get_binary_or_type, "binary" }
impl_borrowed_loader! { BorrowedStringLoader, str, get_string_or_type, "string" }
