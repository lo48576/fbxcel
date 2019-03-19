//! Object properties and related stuff.

pub use self::{
    node::{PropertyHandle, PropertyNodeId},
    object_props::ObjectProperties,
    properties::{PropertiesHandle, PropertiesNodeId},
};

mod node;
mod object_props;
mod properties;
