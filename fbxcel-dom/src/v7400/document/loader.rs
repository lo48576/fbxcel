//! FBX DOM loader.

use fbxcel::{
    pull_parser::{v7400::Parser, ParserSource},
    tree::v7400::{Loader as TreeLoader, Tree},
};
use log::trace;

use crate::v7400::{
    connection::ConnectionsCache, definition::DefinitionsCache, object::ObjectsCache, Document,
    LoadError,
};

/// FBX DOM loader.
#[derive(Default, Debug, Clone)]
pub struct Loader;

impl Loader {
    /// Creates a new `Loader`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a document from the given FBX parser.
    pub fn load_from_parser<R: ParserSource>(
        self,
        parser: &mut Parser<R>,
    ) -> Result<Document, LoadError> {
        trace!("Loading FBX DOM from a parser");
        let (tree, _) = TreeLoader::new().load(parser)?;
        self.load_from_tree(tree)
    }

    /// Loads a document from the given FBX data tree.
    pub fn load_from_tree(self, tree: Tree) -> Result<Document, LoadError> {
        trace!("Loading FBX DOM from an FBX data tree");
        let objects = ObjectsCache::from_tree(&tree)?;
        let connections = ConnectionsCache::from_tree(&tree)?;
        let definitions = DefinitionsCache::from_tree(&tree)?;
        trace!("Loaded FBX DOM successfully");
        Ok(Document {
            tree,
            objects,
            connections,
            definitions,
        })
    }
}
