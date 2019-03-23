//! Property loaders.

use failure::bail;
use fbxcel::low::v7400::AttributeValue;

use crate::v7400::object::property::PropertyHandle;

#[cfg(feature = "mint")]
pub use self::mint::MintLoader;
pub use self::{
    array::{F64Arr16Loader, F64Arr2Loader, F64Arr3Loader, F64Arr4Loader},
    binstr::{BorrowedBinaryLoader, BorrowedStringLoader, OwnedBinaryLoader, OwnedStringLoader},
    primitive::PrimitiveLoader,
    strict_primitive::{StrictF32Loader, StrictF64Loader},
};

macro_rules! prop_type_err {
    ($v:expr, $ty:expr, $node:expr) => {
        format_err!(
            "Unexpected attribute value type for boolean property: \
             expected {} but got {:?}, node_id={:?}",
            $v,
            $ty,
            $node.node_id()
        )
    };
}

mod array;
mod binstr;
#[cfg(feature = "mint")]
mod mint;
mod primitive;
mod strict_primitive;

/// Returns `Ok(value_part)` if the value part has expected length.
fn check_attrs_len<'a>(
    node: &PropertyHandle<'a>,
    expected_len: usize,
    target_name: &str,
) -> Result<&'a [AttributeValue], failure::Error> {
    let value_part = node.value_part();
    let len = value_part.len();
    if len < expected_len {
        bail!(
            "Not enough node attributes for {} property: node_id={:?}, expected {} but got {}",
            target_name,
            node.node_id(),
            expected_len,
            len
        );
    } else if len > expected_len {
        bail!(
            "Too many node attributes for {} property: node_id={:?}, expected {} but got {}",
            target_name,
            node.node_id(),
            expected_len,
            len
        );
    }

    Ok(value_part)
}
