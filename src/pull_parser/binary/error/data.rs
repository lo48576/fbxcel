//! Data error.
//!
//! This is mainly syntax and low-level structure error.

use std::error;
use std::fmt;

/// Data error.
#[derive(Debug)]
pub enum DataError {}

impl error::Error for DataError {}

impl fmt::Display for DataError {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}
