# fbxcel-dom

[![Build Status](https://travis-ci.org/lo48576/fbxcel-dom.svg?branch=develop)](https://travis-ci.org/lo48576/fbxcel-dom)
[![Latest version](https://img.shields.io/crates/v/fbxcel-dom.svg)](https://crates.io/crates/fbxcel-dom)
[![Documentation](https://docs.rs/fbxcel-dom/badge.svg)](https://docs.rs/fbxcel-dom)

`fbxcel-dom` is an FBX DOM library for Rust programming language.

For low-level features, use [`fbxcel`](https://github.com/lo48576/fbxcel) crate.

Note that this crate is highly experimental and updated frequently with breaking
changes (especially for objects-related APIs).

## Features

* Only read-only operations are supported.
* Currently, quite basic tree and objects traversal functions are available.

### FBX versions

* FBX 6 or below is not supported.
* FBX 7.0 to 7.3 is not explicitly supported, but you can try FBX 7.4 feature to
  load them.
* FBX 7.4 and 7.5 is supported.

### FBX format

Only FBX binary format is supported.

Currently there is no plan to support FBX ASCII format.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE.txt](LICENSE-APACHE.txt) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT.txt](LICENSE-MIT.txt) or
  <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
