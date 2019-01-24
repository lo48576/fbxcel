//! Direct attribute value type.

use crate::low::v7400::AttributeType;

/// Direct attribute value type.
#[derive(Debug, Clone, PartialEq)]
pub enum DirectAttributeValue {
    /// Single `bool`.
    Bool(bool),
    /// Single `i16`.
    I16(i16),
    /// Single `i32`.
    I32(i32),
    /// Single `i64`.
    I64(i64),
    /// Single `f32`.
    F32(f32),
    /// Single `f64`.
    F64(f64),
    /// Array of `bool`.
    ArrBool(Vec<bool>),
    /// Array of `i32`.
    ArrI32(Vec<i32>),
    /// Array of `i64`.
    ArrI64(Vec<i64>),
    /// Array of `f32`.
    ArrF32(Vec<f32>),
    /// Array of `f64`.
    ArrF64(Vec<f64>),
    /// UTF-8 string.
    String(String),
    /// Binary.
    Binary(Vec<u8>),
}

macro_rules! impl_val_getter {
    ($method:ident, $variant:ident, $ty_ret:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $method(&self) -> Option<$ty_ret> {
            match self {
                DirectAttributeValue::$variant(v) => Some(*v),
                _ => None,
            }
        }
    }
}

macro_rules! impl_ref_getter {
    ($method:ident, $variant:ident, $ty_ret:ty, $doc:expr) => {
        #[doc = $doc]
        pub fn $method(&self) -> Option<&$ty_ret> {
            match self {
                DirectAttributeValue::$variant(v) => Some(v),
                _ => None,
            }
        }
    }
}

impl DirectAttributeValue {
    /// Returns the value type.
    pub fn type_(&self) -> AttributeType {
        match self {
            DirectAttributeValue::Bool(_) => AttributeType::Bool,
            DirectAttributeValue::I16(_) => AttributeType::I16,
            DirectAttributeValue::I32(_) => AttributeType::I32,
            DirectAttributeValue::I64(_) => AttributeType::I64,
            DirectAttributeValue::F32(_) => AttributeType::F32,
            DirectAttributeValue::F64(_) => AttributeType::F64,
            DirectAttributeValue::ArrBool(_) => AttributeType::ArrBool,
            DirectAttributeValue::ArrI32(_) => AttributeType::ArrI32,
            DirectAttributeValue::ArrI64(_) => AttributeType::ArrI64,
            DirectAttributeValue::ArrF32(_) => AttributeType::ArrF32,
            DirectAttributeValue::ArrF64(_) => AttributeType::ArrF64,
            DirectAttributeValue::String(_) => AttributeType::String,
            DirectAttributeValue::Binary(_) => AttributeType::Binary,
        }
    }

    impl_val_getter! {
        get_bool,
        Bool,
        bool,
        "Returns the the inner `bool` value, if available."
    }

    impl_val_getter! {
        get_i16,
        I16,
        i16,
        "Returns the the inner `i16` value, if available."
    }

    impl_val_getter! {
        get_i32,
        I32,
        i32,
        "Returns the the inner `i32` value, if available."
    }

    impl_val_getter! {
        get_i64,
        I64,
        i64,
        "Returns the the inner `i64` value, if available."
    }

    impl_val_getter! {
        get_f32,
        F32,
        f32,
        "Returns the the inner `f32` value, if available."
    }

    impl_val_getter! {
        get_f64,
        F64,
        f64,
        "Returns the the inner `f64` value, if available."
    }

    impl_ref_getter! {
        get_arr_bool,
        ArrBool,
        [bool],
        "Returns the reference to the inner `bool` slice, if available."
    }

    impl_ref_getter! {
        get_arr_i32,
        ArrI32,
        [i32],
        "Returns the reference to the inner `i32` slice, if available."
    }

    impl_ref_getter! {
        get_arr_i64,
        ArrI64,
        [i64],
        "Returns the reference to the inner `i64` slice, if available."
    }

    impl_ref_getter! {
        get_arr_f32,
        ArrF32,
        [f32],
        "Returns the reference to the inner `f32` slice, if available."
    }

    impl_ref_getter! {
        get_arr_f64,
        ArrF64,
        [f64],
        "Returns the reference to the inner `f64` slice, if available."
    }

    impl_ref_getter! {
        get_string,
        String,
        str,
        "Returns the reference to the inner string slice, if available."
    }

    impl_ref_getter! {
        get_binary,
        Binary,
        [u8],
        "Returns the reference to the inner binary data, if available."
    }
}
