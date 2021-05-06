//! FBX data tree.
//!
//! This module is enabled by `tree` feature.
//!
//! # Creating tree
//!
//! There are two ways to load a tree: easy setup and manual setup.
//!
//! ## Easy setup (recommended)
//!
//! If you don't care about precise FBX version (e.g. difference between FBX 7.4
//! and 7.5), or warnings handling, you can use easy setup using [`any`] module.
//!
//! `AnyTree` loader prints all parser warnings using `log` crate, but treats
//! them non-critical, and the processing will continue.
//!
//! ```no_run
//! use fbxcel::tree::any::AnyTree;
//!
//! let file = std::fs::File::open("sample.fbx").expect("Failed to open file");
//! // You can also use raw `file`, but do buffering for better efficiency.
//! let reader = std::io::BufReader::new(file);
//!
//! // Use `from_seekable_reader` for readers implementing `std::io::Seek`.
//! // To use readers without `std::io::Seek` implementation, use `from_reader`
//! // instead.
//! match AnyTree::from_seekable_reader(reader).expect("Failed to load tree") {
//!     AnyTree::V7400(fbx_version, tree, footer) => {
//!         // You got a tree (and footer)! Do what you want!
//!     }
//!     // `AnyTree` is nonexhaustive.
//!     // You should handle new unknown tree version case.
//!     _ => panic!("Got FBX tree of unsupported version"),
//! }
//! ```
//!
//! ## Manual setup
//!
//! In this way you have full control, but usual users don't need this.
//!
//! 1. Get FBX parser.
//! 2. Pass the parser to appropriate tree loader.
//!
//! About tree loaders, see [`v7400::Loader`].
//! (Currently, only `v7400` tree is supported.)

pub mod any;
pub mod v7400;
