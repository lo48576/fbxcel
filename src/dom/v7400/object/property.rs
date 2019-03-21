//! Object properties and related stuff.

pub use self::{
    node::{PropertyHandle, PropertyNodeId},
    object_props::ObjectProperties,
    properties::{PropertiesHandle, PropertiesNodeId},
    value::LoadPropertyValue,
};

pub mod loaders;
mod node;
mod object_props;
mod properties;
mod value;
