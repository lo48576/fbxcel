//! DOM error.

pub use self::load::LoadError;
pub(crate) use self::load::{LoadErrorKind, StructureError};

mod load;
