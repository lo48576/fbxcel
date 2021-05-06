//! Pull parser for FBX binary.
//!
//! # FBX versions and types
//!
//! Some types are version-agnostic, and some aren't.
//!
//! These modules are common among all supported FBX versions:
//!
//! * Error types (defined in [`error`] module).
//! * [`AnyParser`][`any::AnyParser`] feature (defined in [`any`] module).
//! * Parser source traits and wrappers (defined in [`reader`] module).
//!
//! # Using pull parser
//!
//! There are two ways to set up a parser: easy setup and manual setup.
//!
//! ## Easy setup (recommended)
//!
//! If you don't care about precise FBX version (e.g. difference between FBX 7.4
//! and 7.5), you can use easy setup using [`any`] module.
//!
//! ```no_run
//! use fbxcel::pull_parser::any::{from_seekable_reader, AnyParser};
//!
//! let file = std::fs::File::open("sample.fbx").expect("Failed to open file");
//! // You can also use raw `file`, but do buffering for better efficiency.
//! let reader = std::io::BufReader::new(file);
//!
//! // Use `from_seekable_reader` for readers implementing `std::io::Seek`.
//! // To use readers without `std::io::Seek` implementation, use `from_reader`
//! // instead.
//! match from_seekable_reader(reader).expect("Failed to setup FBX parser") {
//!     // Use v7400 parser (implemented in `v7400` module).
//!     AnyParser::V7400(mut parser) => {
//!         // You got a parser! Do what you want!
//!     },
//!     // `AnyParser` is nonexhaustive.
//!     // You should handle new unknown parser version case.
//!     _ => panic!("Unsupported FBX parser is required"),
//! }
//! ```
//!
//! ## Manual setup
//!
//! In this way you have full control, but usual users don't need this.
//!
//! 1. Get FBX header.
//! 2. Decide which version of parser to use.
//! 3. Create parser with source reader.
//!
//! ```no_run
//! use fbxcel::{low::FbxHeader, pull_parser::ParserVersion};
//!
//! let file = std::fs::File::open("sample.fbx").expect("Failed to open file");
//! // You can also use raw `file`, but do buffering for better efficiency.
//! let mut reader = std::io::BufReader::new(file);
//!
//! // 1. Get FBX header.
//! let header = FbxHeader::load(&mut reader)
//!     .expect("Failed to load FBX header");
//! // 2. Decide which version of parser to use.
//! match header.parser_version() {
//!     // Use v7400 parser (implemented in `v7400` module).
//!     Some(ParserVersion::V7400) => {
//!         // 3. Create parser with source reader.
//!         // Pass both header and reader.
//!         // Use `from_seekable_reader` for readers implementing `std::io::Seek`.
//!         // To use readers without `std::io::Seek` implementation, use
//!         // `from_reader` instead.
//!         let mut parser = fbxcel::pull_parser::v7400::from_seekable_reader(header, reader)
//!             .expect("Failed to setup parser");
//!         // You got a parser! Do what you want!
//!     },
//!     // `ParserVersion` is nonexhaustive.
//!     // You should handle new unknown parser version case.
//!     Some(v) => panic!("Parser version {:?} is not yet supported", v),
//!     // No appropriate parser found
//!     None => panic!(
//!         "FBX version {:?} is not supported by backend library",
//!         header.version()
//!     ),
//! }
//! ```

pub use self::{
    error::{Error, Result, Warning},
    position::SyntacticPosition,
    reader::ParserSource,
    version::ParserVersion,
};

pub mod any;
pub mod error;
mod position;
pub mod reader;
pub mod v7400;
mod version;
