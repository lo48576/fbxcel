//! DOM load error.

use std::fmt;

use failure::{Backtrace, Context, Fail};

use crate::dom::v7400::error::CoreLoadError;
use crate::pull_parser::Error as ParserError;

/// Error kind for DOM load error.
#[derive(Debug, Clone, Fail)]
pub enum LoadErrorKind {
    /// Structure error.
    #[fail(display = "Structure error")]
    Structure,
    /// Syntax error.
    #[fail(display = "Syntax error")]
    Syntax,
    /// Value and type error.
    #[fail(display = "Value and type error")]
    Value,
}

/// Error on DOM load.
#[derive(Debug)]
pub struct LoadError {
    /// Inner error.
    inner: Context<LoadErrorKind>,
}

impl LoadError {
    /// Creates a new `LoadError` from the given error context.
    ///
    /// `From<failure::Error> for LoadError` is not used, because this
    /// functionality should not be publicly exposed (for now).
    pub(crate) fn new(inner: Context<LoadErrorKind>) -> Self {
        Self { inner }
    }
}

impl Fail for LoadError {
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl From<LoadErrorKind> for LoadError {
    fn from(kind: LoadErrorKind) -> Self {
        Self::new(Context::new(kind))
    }
}

impl From<Context<LoadErrorKind>> for LoadError {
    fn from(e: Context<LoadErrorKind>) -> Self {
        Self::new(e)
    }
}

impl From<CoreLoadError> for LoadError {
    fn from(e: CoreLoadError) -> Self {
        Self::new(e.context(LoadErrorKind::Syntax))
    }
}

impl From<ParserError> for LoadError {
    fn from(e: ParserError) -> Self {
        Self::new(e.context(LoadErrorKind::Syntax))
    }
}

impl From<StructureError> for LoadError {
    fn from(e: StructureError) -> Self {
        let kind = match e.kind() {
            StructureErrorKind::AttributeNotFound(..) => LoadErrorKind::Structure,
            StructureErrorKind::NodeNotFound(..) => LoadErrorKind::Structure,
            StructureErrorKind::UnexpectedAttributeType(..) => LoadErrorKind::Value,
            StructureErrorKind::UnexpectedAttributeValue(..) => LoadErrorKind::Value,
        };
        Self::new(e.context(kind))
    }
}

/// Error kind for `StructureError`.
#[derive(Debug, Clone, Fail)]
pub(crate) enum StructureErrorKind {
    /// Attribute not found.
    #[fail(display = "Attribute not found: index={:?}", _0)]
    AttributeNotFound(Option<usize>),
    /// Node not found.
    #[fail(display = "Node not found: {}", _0)]
    NodeNotFound(String),
    /// Unexpected attribute type.
    #[fail(
        display = "Unexpected attribute type: index={:?}, expected {}, got {}",
        _0, _1, _2
    )]
    UnexpectedAttributeType(Option<usize>, String, String),
    /// Unexpected attribute value.
    #[fail(
        display = "Unexpected attribute value: index={:?}, expected {}, got {}",
        _0, _1, _2
    )]
    UnexpectedAttributeValue(Option<usize>, String, String),
}

/// Structure error on DOM load.
#[derive(Debug, Fail)]
#[fail(display = "Structure error at node {:?}: {}", context_node, kind)]
pub(crate) struct StructureError {
    /// Error kind.
    kind: StructureErrorKind,
    /// Context node.
    context_node: Option<String>,
    /// Backtrace.
    backtrace: Backtrace,
}

impl StructureError {
    /// Creates a new `StructureError::AttributeNotFound` error.
    pub(crate) fn attribute_not_found(attr_index: Option<usize>) -> Self {
        StructureErrorKind::AttributeNotFound(attr_index).into()
    }

    /// Creates a new `StructureError::NodeNotfound` error.
    pub(crate) fn node_not_found(target: impl ToString) -> Self {
        StructureErrorKind::NodeNotFound(target.to_string()).into()
    }

    /// Creates a new `StructureError::UnexpectedAttributeType` error.
    pub(crate) fn unexpected_attribute_type(
        attr_index: Option<usize>,
        expected: impl ToString,
        got: impl ToString,
    ) -> Self {
        StructureErrorKind::UnexpectedAttributeType(
            attr_index,
            expected.to_string(),
            got.to_string(),
        )
        .into()
    }

    /// Creates a new `StructureError::UnexpectedAttributeValue` error.
    pub(crate) fn unexpected_attribute_value(
        attr_index: Option<usize>,
        expected: impl ToString,
        got: impl ToString,
    ) -> Self {
        StructureErrorKind::UnexpectedAttributeValue(
            attr_index,
            expected.to_string(),
            got.to_string(),
        )
        .into()
    }

    /// Creates the new error with given context node info.
    pub(crate) fn with_context_node(self, context_node: impl ErrorContextNode) -> Self {
        Self {
            context_node: Some(context_node.to_context_string()),
            ..self
        }
    }

    /// Returns a reference to the error kind.
    fn kind(&self) -> &StructureErrorKind {
        &self.kind
    }
}

impl From<StructureErrorKind> for StructureError {
    fn from(kind: StructureErrorKind) -> Self {
        Self {
            kind,
            context_node: None,
            backtrace: Backtrace::new(),
        }
    }
}

/// A trait for types representing error context node.
pub(crate) trait ErrorContextNode {
    /// Converts the value into string representation.
    fn to_context_string(self) -> String;
}

impl ErrorContextNode for &'_ str {
    fn to_context_string(self) -> String {
        self.into()
    }
}

impl ErrorContextNode for String {
    fn to_context_string(self) -> String {
        self
    }
}

impl<T> ErrorContextNode for (&'_ crate::dom::v7400::Core, T)
where
    T: Into<crate::dom::v7400::NodeId>,
{
    fn to_context_string(self) -> String {
        let (core, node_id) = (self.0, self.1.into());
        core.path(node_id).debug_display().to_string()
    }
}
