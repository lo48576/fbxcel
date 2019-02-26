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
        let kind = match e {
            StructureError::AttributeNotFound(_, _) => LoadErrorKind::Structure,
            StructureError::NodeNotFound(_, _) => LoadErrorKind::Structure,
            StructureError::UnexpectedAttributeType(_, _, _, _) => LoadErrorKind::Value,
            StructureError::UnexpectedAttributeValue(_, _, _, _) => LoadErrorKind::Value,
        };
        Self::new(e.context(kind))
    }
}

/// Error on DOM load.
#[derive(Debug, Fail)]
pub(crate) enum StructureError {
    /// Attribute not found.
    #[fail(display = "Attribute not found: {}", _0)]
    AttributeNotFound(String, Backtrace),
    /// Node not found.
    #[fail(display = "Node not found: {}", _0)]
    NodeNotFound(String, Backtrace),
    /// Unexpected attribute type.
    #[fail(
        display = "Unexpected attribute type: {}, expected {}, got {}",
        _0, _1, _2
    )]
    UnexpectedAttributeType(String, String, String, Backtrace),
    /// Unexpected attribute value.
    #[fail(
        display = "Unexpected attribute value: {}, expected {}, got {}",
        _0, _1, _2
    )]
    UnexpectedAttributeValue(String, String, String, Backtrace),
}

impl StructureError {
    /// Creates a new `StructureError::AttributeNotFound` error.
    pub(crate) fn attribute_not_found(node_path: &[&str], attr_index: Option<usize>) -> Self {
        use std::fmt::Write;

        let mut path = node_path.iter().fold(String::new(), |mut v, component| {
            write!(&mut v, "{:?}/", component).expect("Should never fail");
            v
        });
        path.push_str("attr");
        if let Some(index) = attr_index {
            write!(&mut path, "{}", index).expect("Should never fail");
        }

        StructureError::AttributeNotFound(path, Backtrace::new())
    }

    /// Creates a new `StructureError::NodeNotfound` error.
    pub(crate) fn node_not_found(node_path: &[&str]) -> Self {
        use std::fmt::Write;

        let path = node_path
            .iter()
            .fold((String::new(), ""), |(mut v, leading_sep), component| {
                write!(&mut v, "{}{:?}", leading_sep, component).expect("Should never fail");
                (v, "/")
            })
            .0;

        StructureError::NodeNotFound(path, Backtrace::new())
    }

    /// Creates a new `StructureError::UnexpectedAttributeType` error.
    pub(crate) fn unexpected_attribute_type(
        node_path: &[&str],
        attr_index: Option<usize>,
        expected: impl Into<String>,
        got: impl Into<String>,
    ) -> Self {
        use std::fmt::Write;

        let mut path = node_path.iter().fold(String::new(), |mut v, component| {
            write!(&mut v, "{:?}/", component).expect("Should never fail");
            v
        });
        path.push_str("attr");
        if let Some(index) = attr_index {
            write!(&mut path, "{}", index).expect("Should never fail");
        }

        StructureError::UnexpectedAttributeType(path, expected.into(), got.into(), Backtrace::new())
    }

    /// Creates a new `StructureError::UnexpectedAttributeValue` error.
    pub(crate) fn unexpected_attribute_value(
        node_path: &[&str],
        attr_index: Option<usize>,
        expected: impl Into<String>,
        got: impl Into<String>,
    ) -> Self {
        use std::fmt::Write;

        let mut path = node_path.iter().fold(String::new(), |mut v, component| {
            write!(&mut v, "{:?}/", component).expect("Should never fail");
            v
        });
        path.push_str("attr");
        if let Some(index) = attr_index {
            write!(&mut path, "{}", index).expect("Should never fail");
        }

        StructureError::UnexpectedAttributeValue(
            path,
            expected.into(),
            got.into(),
            Backtrace::new(),
        )
    }
}
