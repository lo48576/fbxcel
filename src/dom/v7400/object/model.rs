//! Model node.

use failure::format_err;
use log::{trace, warn};

use crate::dom::error::{LoadError, LoadErrorKind};
use crate::dom::v7400::object::ObjectNodeId;
use crate::dom::v7400::{Core, Document, NodeId, ValidateId};

define_node_id_type! {
    /// Model node ID.
    ModelNodeId {
        ancestors { ObjectNodeId, NodeId }
    }
}

impl ValidateId for ModelNodeId {
    fn validate_id(self, doc: &Document) -> bool {
        doc.parsed_node_data().models().contains_key(&self)
    }
}

/// Culling type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CullingType {
    /// Off.
    Off,
    /// On, counterclockwise.
    Ccw,
    /// On, clockwise.
    Cw,
}

impl std::str::FromStr for CullingType {
    // TODO: Use more precise error type.
    type Err = LoadError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CullingOff" => Ok(CullingType::Off),
            "CullingOnCcw" => Ok(CullingType::Ccw),
            "CullingOnCw" => Ok(CullingType::Cw),
            _ => Err(
                format_err!("Failed to parse culling type: unknown value `{:?}`", s)
                    .context(LoadErrorKind::Value)
                    .into(),
            ),
        }
    }
}

/// Model node data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModelNodeData {
    /// Shading.
    ///
    /// This seems unused at all, because the shading mode might not be
    /// representable with bool. See
    /// <http://help.autodesk.com/view/FBX/2019/ENU/?guid=FBX_Developer_Help_cpp_ref_class_fbx_node_html>.
    ///
    /// Additionally, this is unstable and not exposed officially by FBX SDK.
    /// See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.
    shading: Option<bool>,
    /// Culling.
    ///
    /// This is unstable and not exposed officially by FBX SDK.
    /// See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.
    culling: Option<CullingType>,
}

impl ModelNodeData {
    /// Culling type.
    ///
    /// This is unstable and not exposed officially by FBX SDK.
    /// See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.
    pub fn culling(&self) -> Option<CullingType> {
        self.culling
    }

    /// Loads the model object node data.
    ///
    /// This should be called for `Model` node.
    pub(crate) fn load(obj_node_id: ObjectNodeId, core: &Core) -> Result<Self, LoadError> {
        trace!("Loading scene node data from object node {:?}", obj_node_id);

        // Ignore errors for this node, because this is "unstable" field.
        // See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.
        let shading = {
            let attr0 = NodeId::from(obj_node_id)
                .children_by_name(core, "Shading")
                .next()
                .and_then(|id| id.node(core).attributes().get(0));
            match attr0.map(|v| v.get_bool_or_type()).transpose() {
                Ok(v) => v,
                Err(ty) => {
                    warn!(
                        "Invalid attribute type for `Shading` node: expected bool, got {:?}. \
                         This error is ignored because `Shading` data is unstable. \
                         See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.",
                        ty);
                    None
                }
            }
        };
        trace!(
            "Got `Shading` data: obj_node={:?}, shading={:?}",
            obj_node_id,
            shading
        );

        // Ignore errors for this node, because this is "unstable" field.
        // See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.
        let culling = {
            let attr0 = NodeId::from(obj_node_id)
                .children_by_name(core, "Culling")
                .next()
                .and_then(|id| id.node(core).attributes().get(0));
            let s = match attr0.map(|v| v.get_string_or_type()).transpose() {
                Ok(v) => v,
                Err(ty) => {
                    warn!(
                        "Invalid attribute type for `Culling` node: expected bool, got {:?}. \
                         This error is ignored because `Culling` data is unstable. \
                         See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.",
                        ty);
                    None
                }
            };
            match s.map(|s| s.parse::<CullingType>()).transpose() {
                Ok(v) => v,
                Err(e) => {
                    warn!(
                        "Invalid attribute type for `Culling` node: {}. \
                         This error is ignored because `Culling` data is unstable. \
                         See <https://forums.autodesk.com/t5/fbx-forum/culling-mode-fbx-sdk/td-p/7023030>.",
                        e);
                    None
                }
            }
        };
        trace!(
            "Got `Culling` data: obj_node={:?}, culling={:?}",
            obj_node_id,
            culling
        );

        trace!("Successfully loaded model node data from {:?}", obj_node_id);

        Ok(Self { shading, culling })
    }
}
