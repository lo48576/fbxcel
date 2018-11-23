//! Attribute type.

/// Attribute type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    /// Single `bool`.
    Bool,
    /// Single `i16`.
    I16,
    /// Single `i32`.
    I32,
    /// Single `i64`.
    I64,
    /// Single `f32`.
    F32,
    /// Single `f64`.
    F64,
    /// Array of `bool`.
    ArrBool,
    /// Array of `i32`.
    ArrI32,
    /// Array of `i64`.
    ArrI64,
    /// Array of `f32`.
    ArrF32,
    /// Array of `f64`.
    ArrF64,
    /// Binary.
    Binary,
    /// UTF-8 string.
    String,
}

impl AttributeType {
    /// Creates an `AttributeType` from the given type code.
    pub(crate) fn from_type_code(code: u8) -> Option<Self> {
        match code {
            b'C' => Some(AttributeType::Bool),
            b'Y' => Some(AttributeType::I16),
            b'I' => Some(AttributeType::I32),
            b'L' => Some(AttributeType::I64),
            b'F' => Some(AttributeType::F32),
            b'D' => Some(AttributeType::F64),
            b'b' => Some(AttributeType::ArrBool),
            b'i' => Some(AttributeType::ArrI32),
            b'l' => Some(AttributeType::ArrI64),
            b'f' => Some(AttributeType::ArrF32),
            b'd' => Some(AttributeType::ArrF64),
            b'R' => Some(AttributeType::Binary),
            b'S' => Some(AttributeType::String),
            _ => None,
        }
    }
}
