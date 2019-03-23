//! Objects cache.

use std::collections::HashMap;

use fbxcel::tree::v7400::{NodeHandle, Tree};
use log::{debug, trace};
use string_interner::StringInterner;

use crate::v7400::{
    error::{
        load::{LoadError, StructureError},
        object::ObjectMetaError,
    },
    object::{ObjectClassSym, ObjectId, ObjectMeta, ObjectNodeId},
};

/// Objects cache.
#[derive(Debug, Clone)]
pub(crate) struct ObjectsCache {
    /// A map from object ID to node ID.
    obj_id_to_node_id: HashMap<ObjectId, ObjectNodeId>,
    /// Object metadata store.
    meta: HashMap<ObjectNodeId, ObjectMeta>,
    /// Interned object classes and subclasses.
    class_strings: StringInterner<ObjectClassSym>,
    /// `Document` nodes.
    document_nodes: Vec<ObjectNodeId>,
}

impl ObjectsCache {
    /// Returns object node ID corresponding to the given object ID.
    pub(crate) fn node_id(&self, obj_id: ObjectId) -> Option<ObjectNodeId> {
        self.obj_id_to_node_id.get(&obj_id).cloned()
    }

    /// Returns a reference to the object metadata.
    pub(crate) fn meta_from_node_id(&self, node_id: ObjectNodeId) -> Option<&ObjectMeta> {
        self.meta.get(&node_id)
    }

    /// Creates a new `ObjectsCache` from the given FBX data tree.
    pub(crate) fn from_tree(tree: &Tree) -> Result<Self, LoadError> {
        debug!("Loading objects cache");
        let objects_cache = ObjectsCacheBuilder::default().load(tree)?;
        debug!(
            "Loaded objects cache successfully: {} objects",
            objects_cache.obj_id_to_node_id.len()
        );
        Ok(objects_cache)
    }

    /// Resolves object class and subclass to string.
    ///
    /// # Panics
    ///
    /// Panics if the given symbol is not registered in the internal table.
    pub(crate) fn resolve_class_string(&self, sym: ObjectClassSym) -> &str {
        self.class_strings
            .resolve(sym)
            .unwrap_or_else(|| panic!("Unresolvable class name symbol: sym={:?}", sym))
    }

    /// Returns document node IDs.
    pub(crate) fn document_nodes(&self) -> &[ObjectNodeId] {
        &self.document_nodes
    }

    /// Returns an iterator of object IDs.
    pub(crate) fn object_node_ids<'a>(&'a self) -> impl Iterator<Item = ObjectNodeId> + 'a {
        self.meta.keys().cloned()
    }
}

/// Objects cache builder.
#[derive(Debug)]
struct ObjectsCacheBuilder {
    /// A map from object ID to node ID.
    obj_id_to_node_id: HashMap<ObjectId, ObjectNodeId>,
    /// Object metadata store.
    meta: HashMap<ObjectNodeId, ObjectMeta>,
    /// Interned object classes and subclasses.
    class_strings: StringInterner<ObjectClassSym>,
    /// `Document` nodes.
    document_nodes: Vec<ObjectNodeId>,
}

impl ObjectsCacheBuilder {
    /// Loads the data from the tree.
    fn load(mut self, tree: &Tree) -> Result<ObjectsCache, LoadError> {
        self.load_objects(tree)?;
        self.load_documents(tree)?;
        Ok(self.build())
    }

    /// Creates an `ObjectsCache` from the builder.
    fn build(self) -> ObjectsCache {
        ObjectsCache {
            obj_id_to_node_id: self.obj_id_to_node_id,
            meta: self.meta,
            class_strings: self.class_strings,
            document_nodes: self.document_nodes,
        }
    }

    /// Loads object nodes under toplevel `Objects` node.
    fn load_objects(&mut self, tree: &Tree) -> Result<(), LoadError> {
        let objects_node = tree
            .root()
            .children_by_name("Objects")
            .next()
            .ok_or(StructureError::MissingObjectsNode)?;
        for object_node in objects_node.children() {
            self.load_object(object_node)?;
        }

        Ok(())
    }

    /// Loads `Document` object nodes under toplevel `Documents` node.
    fn load_documents(&mut self, tree: &Tree) -> Result<(), LoadError> {
        let documents_node = tree
            .root()
            .children_by_name("Documents")
            .next()
            .ok_or(StructureError::MissingDocumentsNode)?;
        for object_node in documents_node.children_by_name("Document") {
            let obj_node_id = self.load_object(object_node)?;
            self.document_nodes.push(obj_node_id);
        }

        Ok(())
    }

    /// Loads an object from the node handle and caches it.
    fn load_object(&mut self, node: NodeHandle<'_>) -> Result<ObjectNodeId, ObjectMetaError> {
        trace!("Loading object metadata, node_id={:?}", node.node_id());
        assert!(
            !self.meta.contains_key(&ObjectNodeId::new(node.node_id())),
            "The node is already loaded: node_id={:?}",
            node.node_id()
        );

        let obj_id = self.load_object_id(node)?;
        let (name, class_sym) = self.load_name_class(node, obj_id)?;
        let subclass_sym = self.load_subclass(node, obj_id)?;

        let obj_node_id = ObjectNodeId::new(node.node_id());
        let meta = ObjectMeta::new(obj_id, name, class_sym, subclass_sym);
        trace!(
            "Successfully loaded object metadata: node={:?}, metadata={:?}",
            obj_node_id,
            meta,
        );

        self.obj_id_to_node_id.insert(obj_id, obj_node_id);
        self.meta.insert(obj_node_id, meta);

        Ok(obj_node_id)
    }

    /// Loads an object ID from the given object node.
    fn load_object_id(&self, node: NodeHandle<'_>) -> Result<ObjectId, ObjectMetaError> {
        let attrs = node.attributes();
        let obj_id = attrs
            .get(0)
            .ok_or_else(|| ObjectMetaError::MissingId(node.node_id()))?
            .get_i64_or_type()
            .map(ObjectId::new)
            .map_err(|ty| ObjectMetaError::InvalidIdType(node.node_id(), ty))?;
        trace!("Got object id: {:?}", obj_id);
        if let Some(&alt_node_id) = self.obj_id_to_node_id.get(&obj_id) {
            return Err(ObjectMetaError::DuplicateObjectId(
                obj_id,
                node.node_id(),
                alt_node_id.into(),
            ));
        }
        Ok(obj_id)
    }

    /// Loads name and class from the given object node.
    fn load_name_class(
        &mut self,
        node: NodeHandle<'_>,
        obj_id: ObjectId,
    ) -> Result<(Option<String>, ObjectClassSym), ObjectMetaError> {
        let attrs = node.attributes();
        let (name, class) = attrs
            .get(1)
            .ok_or_else(|| ObjectMetaError::MissingNameClass(node.node_id(), obj_id))?
            .get_string_or_type()
            .map(|name_class| {
                name_class.find("\u{0}\u{1}").map_or_else(
                    || (None, ""),
                    |sep_pos| {
                        (
                            Some(name_class[0..sep_pos].to_owned()),
                            &name_class[sep_pos + 2..],
                        )
                    },
                )
            })
            .map_err(|ty| ObjectMetaError::InvalidNameClassType(node.node_id(), obj_id, ty))?;
        let class_sym = self.class_strings.get_or_intern(class);
        trace!(
            "Got name and class: object_id={:?}, name={:?}, class={:?}, class_sym={:?}",
            obj_id,
            name,
            class,
            class_sym
        );

        Ok((name, class_sym))
    }

    /// Loads subclass from the given object node.
    fn load_subclass(
        &mut self,
        node: NodeHandle<'_>,
        obj_id: ObjectId,
    ) -> Result<ObjectClassSym, ObjectMetaError> {
        let attrs = node.attributes();
        let subclass = attrs
            .get(2)
            .ok_or_else(|| ObjectMetaError::MissingSubclass(node.node_id(), obj_id))?
            .get_string_or_type()
            .map_err(|ty| ObjectMetaError::InvalidSubclassType(node.node_id(), obj_id, ty))?;
        let subclass_sym = self.class_strings.get_or_intern(subclass);
        trace!(
            "Got subclass: object_id={:?}, subclass={:?}, subclass_sym={:?}",
            obj_id,
            subclass,
            subclass_sym
        );

        Ok(subclass_sym)
    }
}

impl Default for ObjectsCacheBuilder {
    fn default() -> Self {
        Self {
            obj_id_to_node_id: Default::default(),
            meta: Default::default(),
            class_strings: StringInterner::new(),
            document_nodes: Default::default(),
        }
    }
}
