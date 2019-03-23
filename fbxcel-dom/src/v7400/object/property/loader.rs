//! Property loader.

use crate::v7400::object::property::PropertyHandle;

/// A trait for property loader types.
pub trait LoadProperty<'a>: Sized {
    /// Value type.
    type Value;
    /// Error type.
    type Error;

    /// Describes the expecting value.
    fn expecting(&self) -> String;

    /// Loads a value from the property handle if possible.
    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error>;
}

impl<'a, T> LoadProperty<'a> for &'_ T
where
    T: Copy + LoadProperty<'a>,
{
    type Value = <T as LoadProperty<'a>>::Value;
    type Error = <T as LoadProperty<'a>>::Error;

    fn expecting(&self) -> String {
        (*self).expecting()
    }

    fn load(self, node: &PropertyHandle<'a>) -> Result<Self::Value, Self::Error> {
        (*self).load(node)
    }
}
