//! Parsed node data.

use std::collections::HashMap;

use crate::dom::v7400::object::model::{ModelNodeData, ModelNodeId};
use crate::dom::v7400::object::scene::{SceneNodeData, SceneNodeId};
use crate::dom::v7400::object::{ObjectMeta, ObjectNodeId};

/// Parsed node data.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ParsedData {
    /// Object meta.
    object_meta: HashMap<ObjectNodeId, ObjectMeta>,
    /// Scene nodes.
    scenes: HashMap<SceneNodeId, SceneNodeData>,
    /// Model nodes.
    models: HashMap<ModelNodeId, ModelNodeData>,
}

impl ParsedData {
    /// Returns the map of object meta.
    pub fn object_meta(&self) -> &HashMap<ObjectNodeId, ObjectMeta> {
        &self.object_meta
    }

    /// Returns the map of object meta.
    pub(crate) fn object_meta_mut(&mut self) -> &mut HashMap<ObjectNodeId, ObjectMeta> {
        &mut self.object_meta
    }

    /// Returns the map of scene data.
    pub fn scenes(&self) -> &HashMap<SceneNodeId, SceneNodeData> {
        &self.scenes
    }

    /// Returns the map of scene data
    pub(crate) fn scenes_mut(&mut self) -> &mut HashMap<SceneNodeId, SceneNodeData> {
        &mut self.scenes
    }

    /// Returns the map of models data.
    pub fn models(&self) -> &HashMap<ModelNodeId, ModelNodeData> {
        &self.models
    }

    /// Returns the map of scene data
    pub(crate) fn models_mut(&mut self) -> &mut HashMap<ModelNodeId, ModelNodeData> {
        &mut self.models
    }
}
