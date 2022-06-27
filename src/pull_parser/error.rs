//! Errors and result types.
//!
//! Types in this module will be used among multiple versions of parsers.

use std::{error, fmt, io};

use crate::pull_parser::SyntacticPosition;

pub use self::{
    data::{Compression, DataError},
    operation::OperationError,
    warning::Warning,
};

mod data;
mod operation;
mod warning;

/// Parsing result.
pub type Result<T> = std::result::Result<T, Error>;

/// Parsing error.
#[derive(Debug)]
pub struct Error {
    /// The real error.
    repr: Box<Repr>,
}

impl Error {
    /// Returns the error kind.
    #[inline]
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        self.repr.error.kind()
    }

    /// Returns a reference to the inner error container.
    #[inline]
    #[must_use]
    pub fn get_ref(&self) -> &ErrorContainer {
        &self.repr.error
    }

    /// Returns a reference to the inner error if the type matches.
    #[inline]
    #[must_use]
    pub fn downcast_ref<T: 'static + error::Error>(&self) -> Option<&T> {
        self.repr.error.as_error().downcast_ref::<T>()
    }

    /// Returns the syntactic position if available.
    #[inline]
    #[must_use]
    pub fn position(&self) -> Option<&SyntacticPosition> {
        self.repr.position.as_ref()
    }

    /// Creates a new `Error` with the given syntactic position info.
    #[inline]
    #[must_use]
    pub(crate) fn with_position(error: ErrorContainer, position: SyntacticPosition) -> Self {
        Self {
            repr: Box::new(Repr::with_position(error, position)),
        }
    }

    /// Sets the syntactic position and returns the new error.
    #[inline]
    #[must_use]
    pub(crate) fn and_position(mut self, position: SyntacticPosition) -> Self {
        self.repr.position = Some(position);
        self
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.repr.error.fmt(f)
    }
}

impl error::Error for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.repr.error.source()
    }
}

impl<T> From<T> for Error
where
    T: Into<ErrorContainer>,
{
    #[inline]
    fn from(e: T) -> Self {
        Error {
            repr: Box::new(Repr::new(e.into())),
        }
    }
}

/// Internal representation of parsing error.
#[derive(Debug)]
struct Repr {
    /// Error.
    error: ErrorContainer,
    /// Syntactic position.
    position: Option<SyntacticPosition>,
}

impl Repr {
    /// Creates a new `Repr`.
    #[inline]
    #[must_use]
    pub(crate) fn new(error: ErrorContainer) -> Self {
        Self {
            error,
            position: None,
        }
    }

    /// Creates a new `Repr` with the given syntactic position info.
    #[inline]
    #[must_use]
    pub(crate) fn with_position(error: ErrorContainer, position: SyntacticPosition) -> Self {
        Self {
            error,
            position: Some(position),
        }
    }
}

/// Error kind for parsing errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Invalid data.
    ///
    /// With this error kind, the inner error must be [`DataError`].
    ///
    /// [`DataError`]: enum.DataError.html
    Data,
    /// I/O error.
    ///
    /// With this error kind, the inner error must be [`std::io::Error`].
    ///
    /// [`std::io::Error`]:
    /// https://doc.rust-lang.org/stable/std/io/struct.Error.html
    Io,
    /// Invalid operation.
    ///
    /// With this error kind, the inner error must be [`OperationError`].
    ///
    /// [`OperationError`]: enum.OperationError.html
    Operation,
    /// Critical warning.
    ///
    /// With this error kind, the inner error must be [`Warning`].
    ///
    /// [`Warning`]: enum.Warning.html
    Warning,
}

/// Parsing error container.
#[derive(Debug)]
pub enum ErrorContainer {
    /// Invalid data.
    Data(DataError),
    /// I/O error.
    Io(io::Error),
    /// Invalid operation.
    Operation(OperationError),
    /// Critical warning.
    Warning(Warning),
}

impl ErrorContainer {
    /// Returns the error kind of the error.
    #[must_use]
    pub fn kind(&self) -> ErrorKind {
        match self {
            ErrorContainer::Data(_) => ErrorKind::Data,
            ErrorContainer::Io(_) => ErrorKind::Io,
            ErrorContainer::Operation(_) => ErrorKind::Operation,
            ErrorContainer::Warning(_) => ErrorKind::Warning,
        }
    }

    /// Returns `&dyn std::error::Error`.
    #[must_use]
    pub fn as_error(&self) -> &(dyn 'static + error::Error) {
        match self {
            ErrorContainer::Data(e) => e,
            ErrorContainer::Io(e) => e,
            ErrorContainer::Operation(e) => e,
            ErrorContainer::Warning(e) => e,
        }
    }
}

impl error::Error for ErrorContainer {
    #[inline]
    #[must_use]
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.as_error())
    }
}

impl fmt::Display for ErrorContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorContainer::Data(e) => write!(f, "Data error: {}", e),
            ErrorContainer::Io(e) => write!(f, "I/O error: {}", e),
            ErrorContainer::Operation(e) => write!(f, "Invalid operation: {}", e),
            ErrorContainer::Warning(e) => write!(f, "Warning considered critical: {}", e),
        }
    }
}

impl From<io::Error> for ErrorContainer {
    #[inline]
    fn from(e: io::Error) -> Self {
        ErrorContainer::Io(e)
    }
}

impl From<DataError> for ErrorContainer {
    #[inline]
    fn from(e: DataError) -> Self {
        ErrorContainer::Data(e)
    }
}

impl From<OperationError> for ErrorContainer {
    #[inline]
    fn from(e: OperationError) -> Self {
        ErrorContainer::Operation(e)
    }
}

impl From<Warning> for ErrorContainer {
    #[inline]
    fn from(e: Warning) -> Self {
        ErrorContainer::Warning(e)
    }
}
