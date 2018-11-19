//! Parser for BX 7.4 or later.

use std::io;

use super::super::reader::{PlainSource, SeekableSource};
use super::error::OperationError;
use super::{FbxVersion, ParserSource, ParserVersion, Result};

/// FBX file header size.
const FILE_HEADER_SIZE: usize = 23 + 4;

/// Creates a new `Parser` from the given buffered reader.
///
/// Returns an error if the given FBX version in unsupported.
pub fn from_reader<R>(fbx_version: FbxVersion, reader: R) -> Result<Parser<PlainSource<R>>>
where
    R: io::Read,
{
    Parser::create(
        fbx_version,
        PlainSource::with_offset(reader, FILE_HEADER_SIZE),
    )
}

/// Creates a new `Parser` from the given seekable reader.
///
/// Returns an error if the given FBX version in unsupported.
pub fn from_seekable_reader<R>(
    fbx_version: FbxVersion,
    reader: R,
) -> Result<Parser<SeekableSource<R>>>
where
    R: io::Read + io::Seek,
{
    Parser::create(
        fbx_version,
        SeekableSource::with_offset(reader, FILE_HEADER_SIZE),
    )
}

/// Pull parser for FBX 7.4 binary or compatible later versions.
#[derive(Debug, Clone)]
pub struct Parser<R> {
    /// Parser state.
    state: State,
    /// Reader.
    reader: R,
}

impl<R: ParserSource> Parser<R> {
    /// Parser version.
    pub const PARSER_VERSION: ParserVersion = ParserVersion::V7400;

    /// Creates a new `Parser`.
    ///
    /// Returns an error if the given FBX version in unsupported.
    pub(crate) fn create(fbx_version: FbxVersion, reader: R) -> Result<Self> {
        if fbx_version.parser_version() != Some(Self::PARSER_VERSION) {
            return Err(
                OperationError::UnsupportedFbxVersion(Self::PARSER_VERSION, fbx_version).into(),
            );
        }

        Ok(Self {
            state: State::new(fbx_version),
            reader,
        })
    }

    /// Returns FBX version.
    pub fn fbx_version(&self) -> FbxVersion {
        self.state.fbx_version
    }
}

/// Parser state.
///
/// This type contains parser state especially which are independent of parser
/// source type.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    /// Target FBX version.
    fbx_version: FbxVersion,
}

impl State {
    /// Creates a new `State` for the given FBX version.
    fn new(fbx_version: FbxVersion) -> Self {
        Self { fbx_version }
    }
}
