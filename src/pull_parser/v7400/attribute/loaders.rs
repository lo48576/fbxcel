//! Node attribute loaders.

pub use self::{
    direct::DirectLoader,
    single::{ArrayLoader, BinaryLoader, PrimitiveLoader, StringLoader},
    type_::TypeLoader,
};

mod direct;
mod single;
mod type_;
