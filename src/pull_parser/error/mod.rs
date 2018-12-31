//! Errors and result types.
//!
//! Types in this module will be used among multiple versions of parsers.

use std::error;
use std::fmt;
use std::io;

pub use self::data::{Compression, DataError};
pub use self::operation::OperationError;
pub use self::warning::Warning;

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
    pub fn kind(&self) -> ErrorKind {
        self.repr.error.kind()
    }

    /// Returns a reference to the inner error container.
    pub fn get_ref(&self) -> &ErrorContainer {
        &self.repr.error
    }

    /// Returns a reference to the inner error if the type matches.
    pub fn downcast_ref<T: 'static + error::Error>(&self) -> Option<&T> {
        self.repr.error.as_error().downcast_ref::<T>()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.repr.error.fmt(f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.repr.error.source()
    }
}

impl<T> From<T> for Error
where
    T: Into<ErrorContainer>,
{
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
}

impl Repr {
    /// Creates a new `Repr`.
    fn new(error: ErrorContainer) -> Self {
        Self { error }
    }
}

/// Error kind for parsing errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// Invalid data.
    ///
    /// With this error kind, the inner error must be [`DataError`].
    Data,
    /// I/O error.
    ///
    /// With this error kind, the inner error must be [`std::io::Error`].
    Io,
    /// Invalid operation.
    ///
    /// With this error kind, the inner error must be [`OperationError`].
    Operation,
    /// Critical warning.
    ///
    /// With this error kind, the inner error must be [`Warning`].
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
    pub fn kind(&self) -> ErrorKind {
        match self {
            ErrorContainer::Data(_) => ErrorKind::Data,
            ErrorContainer::Io(_) => ErrorKind::Io,
            ErrorContainer::Operation(_) => ErrorKind::Operation,
            ErrorContainer::Warning(_) => ErrorKind::Warning,
        }
    }

    /// Returns `&dyn std::error::Error`.
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
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(self.as_error())
    }
}

impl fmt::Display for ErrorContainer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorContainer::Data(e) => write!(f, "Data error: {}", e),
            ErrorContainer::Io(e) => write!(f, "I/O error: {}", e),
            ErrorContainer::Operation(e) => write!(f, "Invalid operation: {}", e),
            ErrorContainer::Warning(e) => write!(f, "Warning considered critical: {}", e),
        }
    }
}

impl From<io::Error> for ErrorContainer {
    fn from(e: io::Error) -> Self {
        ErrorContainer::Io(e)
    }
}

impl From<DataError> for ErrorContainer {
    fn from(e: DataError) -> Self {
        ErrorContainer::Data(e)
    }
}

impl From<OperationError> for ErrorContainer {
    fn from(e: OperationError) -> Self {
        ErrorContainer::Operation(e)
    }
}

impl From<Warning> for ErrorContainer {
    fn from(e: Warning) -> Self {
        ErrorContainer::Warning(e)
    }
}
