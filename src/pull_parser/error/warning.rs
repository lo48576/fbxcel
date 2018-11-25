//! Invalid operation.

use std::error;
use std::fmt;

/// Warning.
#[derive(Debug)]
pub enum Warning {}

impl error::Error for Warning {}

impl fmt::Display for Warning {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
