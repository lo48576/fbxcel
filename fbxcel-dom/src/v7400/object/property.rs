//! Object properties and related stuff.

pub use self::{
    loader::LoadProperty,
    node::{PropertyHandle, PropertyNodeId},
    object_props::ObjectProperties,
    properties::{PropertiesHandle, PropertiesNodeId},
};

mod loader;
pub mod loaders;
mod node;
mod object_props;
mod properties;
