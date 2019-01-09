//! FBX DOM document loader.

use std::collections::HashMap;

use log::warn;

use crate::dom::v7400::object::{ObjectId, ObjectMeta, ObjectNodeId};
use crate::dom::v7400::{Core, Document, IntoRawNodeId, NodeId};
use crate::dom::{AccessError, LoadError};
use crate::pull_parser::v7400::Parser;
use crate::pull_parser::ParserSource;

macro_rules! bail_if_strict {
    ($is_strict:expr, $err:expr, $loose:expr) => {
        if $is_strict {
            return Err($err.into());
        } else {
            $loose
        }
    };
    ($is_strict:expr, $err:expr) => {
        if $is_strict {
            return Err($err.into());
        }
    };
}

macro_rules! warn_noncritical {
    ($strict:expr, $format:expr) => {
        warn!(concat!("Noncritical DOM load error [strict={}] ", $format), $strict)
    };
    ($strict:expr, $format:expr, $($args:tt)*) => {
        warn!(
            concat!("Noncritical DOM load error [strict={}] ", $format),
            $strict, $($args)*
        )
    };
}

/// DOM document loader.
#[derive(Default, Debug, Clone)]
pub struct Loader {
    /// DOM core.
    core: Option<Core>,
    /// Strict mode flag.
    ///
    /// If this is `true`, non-critical errors should be `Err`.
    /// If `false`, non-critical errors are ignored.
    strict: bool,
    /// Map from object ID to node ID.
    object_ids: HashMap<ObjectId, ObjectNodeId>,
}

impl Loader {
    /// Creates a new `Loader`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the strict mode flag.
    pub fn strict(self, v: bool) -> Self {
        Self { strict: v, ..self }
    }

    /// Returns the reference to the DOM core.
    ///
    /// # Panics
    ///
    /// Panics if the DOM core is uninitialized.
    #[inline]
    fn core(&self) -> &Core {
        self.core.as_ref().expect("DOM core is not yet initialized")
    }

    /// Returns the mutable reference to the DOM core.
    ///
    /// # Panics
    ///
    /// Panics if the DOM core is uninitialized.
    #[inline]
    fn core_mut(&mut self) -> &mut Core {
        self.core.as_mut().expect("DOM core is not yet initialized")
    }

    /// Loads the DOM document from the parser.
    pub fn load_document<R>(mut self, parser: &mut Parser<R>) -> Result<Document, LoadError>
    where
        R: ParserSource,
    {
        // Load basic tree.
        self.core = Some(self.load_core(parser)?);

        // Load objects.
        self.load_objects()?;

        Ok(Document::new(
            self.core
                .expect("Should never fail: `self.core` is `Some(_)` here"),
            self.object_ids,
        ))
    }

    /// Loads DOM core.
    fn load_core<R>(&self, parser: &mut Parser<R>) -> Result<Core, LoadError>
    where
        R: ParserSource,
    {
        assert!(self.core.is_none(), "Attempt to initialize DOM core twice");
        Core::load(parser)
    }

    /// Finds a toplevel node by the name.
    fn find_toplevel(&self, target_name: &str) -> Option<NodeId> {
        let core = self.core();
        let target_sym = core.sym_opt(target_name)?;
        for toplevel_id in core.root().raw_node_id().children(&core.nodes()) {
            let toplevel = core.node(toplevel_id);
            if toplevel.data().name_sym() == target_sym {
                return Some(NodeId::new(toplevel_id));
            }
        }
        None
    }

    /// Loads objects.
    fn load_objects(&mut self) -> Result<(), LoadError> {
        assert!(
            self.object_ids.is_empty(),
            "Attempt to initialize `self.object_ids` which has been already initialized"
        );

        // Cannot use `indextree::NodeId::children()`, because it borrows arena.

        // `/Objects/*` nodes.
        if let Some(objects_node_id) = self.find_toplevel("Objects") {
            let mut next_node_id = self.core().node(objects_node_id).first_child();
            while let Some(object_node_id) = next_node_id {
                self.add_object(object_node_id)?;
                next_node_id = self.core().node(object_node_id).next_sibling();
            }
        } else {
            warn_noncritical!(self.strict, "`Objects` node not found");
            bail_if_strict!(
                self.strict,
                AccessError::NodeNotFound("`Objects`".to_owned()),
                return Ok(())
            );
        }

        // `/Documents/Document` nodes.
        if let Some(documents_node_id) = self.find_toplevel("Documents") {
            let document_sym = self.core().sym_opt("Document");
            let mut next_node_id = self.core().node(documents_node_id).first_child();
            while let Some(document_node_id) = next_node_id {
                if Some(self.core().node(document_node_id).data().name_sym()) == document_sym {
                    self.add_object(document_node_id)?;
                }
                next_node_id = self.core().node(document_node_id).next_sibling();
            }
        } else {
            warn_noncritical!(self.strict, "`Documents` node not found");
            bail_if_strict!(
                self.strict,
                AccessError::NodeNotFound("`Documents`".to_owned()),
                return Ok(())
            );
        }

        Ok(())
    }

    /// Register object node.
    fn add_object(&mut self, node_id: NodeId) -> Result<(), LoadError> {
        use std::collections::hash_map::Entry;

        let obj_meta = {
            let (node, strings) = self.core_mut().node_and_strings(node_id);
            let attrs = node.data().attributes();
            match ObjectMeta::from_attributes(attrs, strings) {
                Ok(v) => v,
                Err(e) => {
                    warn_noncritical!(self.strict, "Object load error: {}", e);
                    bail_if_strict!(self.strict, e, return Ok(()));
                }
            }
        };
        let obj_id = obj_meta.id();
        let node_id = ObjectNodeId::new(node_id);

        // Register to `object_ids`.
        match self.object_ids.entry(obj_id) {
            Entry::Occupied(entry) => {
                warn_noncritical!(
                    self.strict,
                    "Duplicate object ID: nodes with ID {:?} and {:?} have same object ID {:?}",
                    entry.get(),
                    node_id,
                    obj_id
                );
                bail_if_strict!(
                    self.strict,
                    LoadError::DuplicateId("object".to_owned(), format!("{:?}", obj_id)),
                    return Ok(())
                );
            }
            Entry::Vacant(entry) => {
                entry.insert(node_id);
            }
        }

        Ok(())
    }
}
