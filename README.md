# fbxcel

[![Build Status](https://travis-ci.com/lo48576/fbxcel.svg?branch=develop)](https://travis-ci.com/lo48576/fbxcel)
[![Latest version](https://img.shields.io/crates/v/fbxcel.svg)](https://crates.io/crates/fbxcel)
[![Documentation](https://docs.rs/fbxcel/badge.svg)](https://docs.rs/fbxcel)
![Minimum rustc version: 1.56](https://img.shields.io/badge/rustc-1.56+-lightgray.svg)

`fbxcel` is an FBX library for Rust programming language.

`fbxcel` is relatively low-level library.
If you want to interpret and render FBX data, use
[`fbxcel-dom`](https://github.com/lo48576/fbxcel-dom) crate.

## Features

* Pull parser for FBX binary (`pull_parser` module)
    + FBX 7.4 and 7.5 is explicitly supported.
* Writer for FBX binary (`writer` module)
    + FBX 7.4 and 7.5 is explicitly supported.
    + This is optional and enabled by `writer` feature.
* Types and functions for low-level FBX tree access
    + This is optional and enabled by `tree` feature.
    + Provides arena-based tree type and read-only access to nodes.

### FBX versions

* FBX 6 or below is not supported.
* FBX 7.0 to 7.3 is not explicitly supported, but you can try FBX 7.4 feature to load them.
* FBX 7.4 and 7.5 is supported.

### FBX format

Only FBX binary format is supported.

Currently there is no plan to support FBX ASCII format.


## Rust version

Latest stable compiler (currently 1.52) is supported.

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
