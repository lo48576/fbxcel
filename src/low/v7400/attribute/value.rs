//! Node attribute value.

use crate::low::v7400::AttributeType;

/// Node attribute value.
///
/// To get a value of the specific type easily, use `get_*()` or
/// `get_*_or_type()` method.
///
/// * `get_*()` returns `Option<_>`.
///     + If a value of the expected type available, returns `Some(_)`.
///     + If not, returns `None`.
/// * `get_*_or_type()` returns `Result<_, AttributeType>`.
///     + If a value of the expected type available, returns `Ok(_)`.
///     + If not, returns `Ok(ty)` where `ty` is value type (same value as
///       returned by [`type_`][`Self::type_()`] method.
#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
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

/// Implement direct value getters.
macro_rules! impl_val_getter {
    ($variant:ident, $ty_ret:ty, $opt_getter:ident, $opt_doc:expr, $res_getter:ident, $res_doc:expr,) => {
        #[doc = $opt_doc]
        #[inline]
        #[must_use]
        pub fn $opt_getter(&self) -> Option<$ty_ret> {
            match self {
                AttributeValue::$variant(v) => Some(*v),
                _ => None,
            }
        }

        #[doc = $res_doc]
        pub fn $res_getter(&self) -> Result<$ty_ret, AttributeType> {
            match self {
                AttributeValue::$variant(v) => Ok(*v),
                _ => Err(self.type_()),
            }
        }
    };
}

/// Implement value reference getters.
macro_rules! impl_ref_getter {
    ($variant:ident, $ty_ret:ty, $opt_getter:ident, $opt_doc:expr, $res_getter:ident, $res_doc:expr,) => {
        #[doc = $opt_doc]
        #[inline]
        #[must_use]
        pub fn $opt_getter(&self) -> Option<&$ty_ret> {
            match self {
                AttributeValue::$variant(v) => Some(v),
                _ => None,
            }
        }

        #[doc = $res_doc]
        pub fn $res_getter(&self) -> Result<&$ty_ret, AttributeType> {
            match self {
                AttributeValue::$variant(v) => Ok(v),
                _ => Err(self.type_()),
            }
        }
    };
}

impl AttributeValue {
    /// Returns the value type.
    #[must_use]
    pub fn type_(&self) -> AttributeType {
        match self {
            AttributeValue::Bool(_) => AttributeType::Bool,
            AttributeValue::I16(_) => AttributeType::I16,
            AttributeValue::I32(_) => AttributeType::I32,
            AttributeValue::I64(_) => AttributeType::I64,
            AttributeValue::F32(_) => AttributeType::F32,
            AttributeValue::F64(_) => AttributeType::F64,
            AttributeValue::ArrBool(_) => AttributeType::ArrBool,
            AttributeValue::ArrI32(_) => AttributeType::ArrI32,
            AttributeValue::ArrI64(_) => AttributeType::ArrI64,
            AttributeValue::ArrF32(_) => AttributeType::ArrF32,
            AttributeValue::ArrF64(_) => AttributeType::ArrF64,
            AttributeValue::String(_) => AttributeType::String,
            AttributeValue::Binary(_) => AttributeType::Binary,
        }
    }

    impl_val_getter! {
        Bool,
        bool,
        get_bool,
        "Returns the the inner `bool` value, if available.",
        get_bool_or_type,
        "Returns the the inner `bool` value, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_val_getter! {
        I16,
        i16,
        get_i16,
        "Returns the the inner `i16` value, if available.",
        get_i16_or_type,
        "Returns the the inner `i16` value, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_val_getter! {
        I32,
        i32,
        get_i32,
        "Returns the the inner `i32` value, if available.",
        get_i32_or_type,
        "Returns the the inner `i32` value, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_val_getter! {
        I64,
        i64,
        get_i64,
        "Returns the the inner `i64` value, if available.",
        get_i64_or_type,
        "Returns the the inner `i64` value, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_val_getter! {
        F32,
        f32,
        get_f32,
        "Returns the the inner `f32` value, if available.",
        get_f32_or_type,
        "Returns the the inner `f32` value, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_val_getter! {
        F64,
        f64,
        get_f64,
        "Returns the the inner `f64` value, if available.",
        get_f64_or_type,
        "Returns the the inner `f64` value, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        ArrBool,
        [bool],
        get_arr_bool,
        "Returns the reference to the inner `bool` slice, if available.",
        get_arr_bool_or_type,
        "Returns the reference to the inner `bool` slice, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        ArrI32,
        [i32],
        get_arr_i32,
        "Returns the reference to the inner `i32` slice, if available.",
        get_arr_i32_or_type,
        "Returns the reference to the inner `i32` slice, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        ArrI64,
        [i64],
        get_arr_i64,
        "Returns the reference to the inner `i64` slice, if available.",
        get_arr_i64_or_type,
        "Returns the reference to the inner `i64` slice, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        ArrF32,
        [f32],
        get_arr_f32,
        "Returns the reference to the inner `f32` slice, if available.",
        get_arr_f32_or_type,
        "Returns the reference to the inner `f32` slice, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        ArrF64,
        [f64],
        get_arr_f64,
        "Returns the reference to the inner `f64` slice, if available.",
        get_arr_f64_or_type,
        "Returns the reference to the inner `f64` slice, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        String,
        str,
        get_string,
        "Returns the reference to the inner string slice, if available.",
        get_string_or_type,
        "Returns the reference to the inner string slice, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    impl_ref_getter! {
        Binary,
        [u8],
        get_binary,
        "Returns the reference to the inner binary data, if available.",
        get_binary_or_type,
        "Returns the reference to the inner binary data, if available.\n\nReturns `Err(type)` on type mismatch.",
    }

    /// Compares attribute values strictly.
    ///
    /// "Strictly" means, `f32` and `f64` values are compared bitwise.
    pub fn strict_eq(&self, other: &Self) -> bool {
        use AttributeValue::*;

        match (self, other) {
            (Bool(l), Bool(r)) => l == r,
            (I16(l), I16(r)) => l == r,
            (I32(l), I32(r)) => l == r,
            (I64(l), I64(r)) => l == r,
            (F32(l), F32(r)) => l.to_bits() == r.to_bits(),
            (F64(l), F64(r)) => l.to_bits() == r.to_bits(),
            (ArrBool(l), ArrBool(r)) => l == r,
            (ArrI32(l), ArrI32(r)) => l == r,
            (ArrI64(l), ArrI64(r)) => l == r,
            (ArrF32(l), ArrF32(r)) => l
                .iter()
                .map(|v| v.to_bits())
                .eq(r.iter().map(|v| v.to_bits())),
            (ArrF64(l), ArrF64(r)) => l
                .iter()
                .map(|v| v.to_bits())
                .eq(r.iter().map(|v| v.to_bits())),
            (Binary(l), Binary(r)) => l == r,
            (String(l), String(r)) => l == r,
            _ => false,
        }
    }
}

/// Implement `From` trait.
macro_rules! impl_from {
    (direct: $ty:ty, $variant:ident) => {
        impl From<$ty> for AttributeValue {
            #[inline]
            fn from(v: $ty) -> Self {
                AttributeValue::$variant(v.into())
            }
        }
    };
    (map: $ty:ty, $variant:ident, $arg:ident, $v:expr) => {
        impl From<$ty> for AttributeValue {
            #[inline]
            fn from($arg: $ty) -> Self {
                AttributeValue::$variant($v)
            }
        }
    };
}

impl_from! { direct: bool, Bool }
impl_from! { direct: i16, I16 }
impl_from! { direct: i32, I32 }
impl_from! { direct: i64, I64 }
impl_from! { direct: f32, F32 }
impl_from! { direct: f64, F64 }
impl_from! { direct: Vec<bool>, ArrBool }
impl_from! { direct: Vec<i32>, ArrI32 }
impl_from! { direct: Vec<i64>, ArrI64 }
impl_from! { direct: Vec<f32>, ArrF32 }
impl_from! { direct: Vec<f64>, ArrF64 }
impl_from! { direct: Vec<u8>, Binary }
impl_from! { direct: String, String }
impl_from! { map: &[bool], ArrBool, v, v.to_owned() }
impl_from! { map: &[i32], ArrI32, v, v.to_owned() }
impl_from! { map: &[i64], ArrI64, v, v.to_owned() }
impl_from! { map: &[f32], ArrF32, v, v.to_owned() }
impl_from! { map: &[f64], ArrF64, v, v.to_owned() }
impl_from! { map: &[u8], Binary, v, v.to_owned() }
impl_from! { map: &str, String, v, v.to_owned() }
