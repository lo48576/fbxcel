//! Property value.

use crate::dom::v7400::object::property::PropertyHandle;

/// A trait for property value loader types.
pub trait LoadPropertyValue<'a>: Sized {
    /// Value type.
    type Value;
    /// Error type.
    type Error;

    /// Describes the expecting value.
    fn expecting(&self) -> String;

    /// Loads a value from the property handle if possible.
    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error>;
}

impl<'a, T> LoadPropertyValue<'a> for &'_ T
where
    T: Copy + LoadPropertyValue<'a>,
{
    type Value = <T as LoadPropertyValue<'a>>::Value;
    type Error = <T as LoadPropertyValue<'a>>::Error;

    fn expecting(&self) -> String {
        (*self).expecting()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        (*self).load(node)
    }
}
