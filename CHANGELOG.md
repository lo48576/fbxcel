# Change Log

## [Unreleased]

## [0.8.1]

* Add types and methods to traverse nodes in depth-first order.
* Add methods to `Tree` to modify node attributes (not only appending).
* Make many (but not all) small methods `#[inline]`.
* Make some funcitions `#[must_use]`.

### Added
* Add types and methods to traverse nodes in depth-first order.
    + `tree::v7400::DepthFirstTraversed` type is added.
    + `tree::v7400::DepthFirstTraverseSubtree` type is added.
* Add methods to `Tree` to modify node attributes (not only appending).
    + `tree::v7400::Tree` has now three new methods: `get_attribute_mut()`,
      `take_attributes_vec()`, and `set_attributes_vec()`.

### Changed (non-breaking)
* Make many (but not all) small methods `#[inline]`.
* Make some funcitions `#[must_use]`.

## [0.8.0]

* Bump minimum supported Rust version to 1.56.0.
* Iterator types returned by `tree::v7400::NodeHandle::{children, children_by_name}`
  now have a name.
* All iterator types now have `std::iter::FusedIterator` impl.
* `tree::v7400::NodeHandle::first_child_by_name()` is added.
* Now some items on docs.rs have pretty badges indicating the items is only
  enabled with some features.
    + This won't affect any builds by other users. `cargo doc --all-features`
      and other commands should still run successfully with stable toolchain.
* Bump internal dependencies.

### Added
* `tree::v7400::NodeHandle::first_child_by_name()` is added.
    + `node.first_child_by_name(name)` returns the same result as
      `node.children_by_name(name).next()`.

### Fixed
* Fixed incorrect attribute type value being written by the writer.

### Breaking changes
* Bump minimum supported Rust version to 1.56.0.

### Non-breaking changes
* Iterator types returned by `tree::v7400::NodeHandle::{children, children_by_name}`
  now have a name.
    + `NodeHandle::children()` returns `Children<'_>`.
    + `NodeHandle::children_by_name()` returns `ChildrenByName<'_>`.
    + By this change, now these iterators can be included in other types as a field.
* All iterator types defined by this crate now have `std::iter::FusedIterator` impl.

## [0.7.0]

* Bump minimum supported Rust version to 1.49.
* Bump internal dependencies.

## [0.6.0]

* Minimum supported Rust version is bumped to 1.40.0.
* Add an FBX version field to `any::AnyTree::V7400` variant
  (372a2f6e0314eed86cc2c493d2e2fc86aa226781).
* Add `any::AnyTree::fbx_version()` method (372a2f6e0314eed86cc2c493d2e2fc86aa226781).

### Breaking changes
* Add an FBX version field to `any::AnyTree::V7400` variant
  (372a2f6e0314eed86cc2c493d2e2fc86aa226781).
    + This is mainly used by newly added `any::AnyTree::fbx_version()`, but also useful for users to
      know FBX version.
        - For example, when users want to re-export the tree, they might want to know FBX version of
          the source document.

### Added
* Add `any::AnyTree::fbx_version()` method (372a2f6e0314eed86cc2c493d2e2fc86aa226781).
    + Using this, users can get FBX version of the tree even if the `AnyTree` variant is unknown for
      users.
    + By this method, users can emit meaningful error message if the tree is returned as unknown
      variant.

### Non-breaking changes
* Use `#[non_exhaustive]` instead of hidden dummy variants for enums
  (b4c0cf53fcefb2dc13850e09ac1ff15bc57a68e5).
    + Users won't affected by this internal change.

## [0.5.0]

* `pull_parser::error::{DataError, OperationError, Warning}` is now nonexhaustive.
    + This would make some of future changes non-breaking.
* Support parsing nodes with missing or extra node end markers.
    + Previously, they are ignored or causing critical errors.
      Now they are notified as warnings, and users can continue parsing.
    + Two new variants `Warning::{ExtraNodeEndMarker, MissingNodeEndMarker}` are added to
      `pull_parser::error::Warning` type.
        - Note that `Warning` have been nonexhaustive since this release.
* Deprecated items are removed.
    + `low::FbxHeader::read_fbx_header()`
    + `pull_parser::v7400::attribute::DirectAttributeValue`

### Breaking changes
* `pull_parser::error::{DataError, OperationError, Warning}` is now nonexhaustive
  (d0651118feabf842f9495da626ccb127090db331).
    + This would make some of future changes non-breaking.
* Support parsing nodes with missing or extra node end markers
  (8c3d8b7f210fe8422784ef86b468e5331bb0c2ee).
    + Previously, missing node end markers caused errors, and extra node end markers were silently
      ignored.
      Now they are notified as warnings.
      Users can choose whether to continue or abort processing.
    + Two new variants `Warning::{ExtraNodeEndMarker, MissingNodeEndMarker}` are added to
      `pull_parser::error::Warning` type.
        - Note that `Warning` have been nonexhaustive since this release.
* Deprecated items are removed (9e38b4217d33ed8bca3f7e8b11d210845a4fa8c1).
    + `low::FbxHeader::read_fbx_header()`
    + `pull_parser::v7400::attribute::DirectAttributeValue`

## [0.4.4]

* Documents are improved a little.
* Manual tree construction (without using parser) is now supported.
    + You can add nodes and attributes manually to the tree at runtime.
    + You can describe the tree using `tree_v7400!` macro at compile time.
* FBX binary writer is added.
* Tiny improvements:
    + `low::v7400::AttributeValue` implements `From<_>` for some types.
    + Strict equality check is added for trees, nodes, and attribute values.
    + `tree::v7400::Tree::debug_tree()` is added.
* Now rustc-1.34 or later is required.
    + To use `std::convert::{TryFrom, TryInto}`.

### Added
* Manual tree construction support is added (64f70b051c30, 39c4fabad119).
    + Methods to add new nodes and attributes are added.
    + Complete modification is not yet supported, for example modifying already
      added attributes or removing nodes.
    * `tree_v7400!` macro is added to construct tree easily.
      See documentation for detail.
* FBX binary writer is added (e1cb2a232d19, 33d9ac3a589c, d5dc779c0bd4,
  6cddca849a4f, 8c84359d2578).
    + `writer::v7400::binary` contains FBX binary writer stuff.
    + This can be enabled by `writer` feature.
    + `write_v7400_binary!` macro is also added.
      See the documentation for detail.
* `low::v7400::AttributeValue` implements `From<_>` for some types
  (a54226534a73, 6546d62fd38a).
    + Primitive types: `bool`, `i16`, `i32`, `i64`, `f32`, `f64`.
    + Vector types: `Vec<bool>`, `Vec<i32>`, `Vec<i64>`, `Vec<f32>`, `Vec<f64>`,
      `Vec<u8>`.
    + Slice types: `&[bool]`, `&[i32]`, `&[i64]`, `&[f32]`, `&[f64]`, `&[u8]`.
    + Special types: `String`, `&str`.
* Strict equality check is added for trees, nodes, and attribute values
  (8784d7609d8e).
    + Trees: `tree::v7400::Tree::strict_eq()`.
    + Nodes: `tree::v7400::NodeHandle::strict_eq()`.
    + Attributes: `low::v7400::AttributeValue::strict_eq()`.
    + These checks compares `f32` and `f64` bitwise.
      This means `NAN == NAN` situation is possible.
* `tree::v7400::Tree::debug_tree()` is added (4524b4dc4a99).
    * This returns pretty-printable object of the tree.
    * It dumps human-readable tree structure.
    * Default `Debug` implementation for `Tree` is hard to read because it dumps
      arena and interned string table.

### Non-breaking change
* Now rustc-1.34 or later is required.
    + To use `std::convert::TryFrom`.
    + Strictly, this is a breaking change (for users with rustc-1.33 or below),
      but not breaking for users with latest rustc.
    + Currently, only `writer` module uses `TryFrom`.
      Users not using `writer` feature won't be affected for now, but they could
      encounter compile error in future version of fbxcel.

## [0.4.3]

* Longer lifetime for iterator returned by
  `tree::v7400::NodeHandle::children_by_name()`.

### Non-breaking change
* Longer lifetime for iterator returned by
  `tree::v7400::NodeHandle::children_by_name()` (08ab27a7fc23).
    + Previously, lifetime of the returned iterator should be same as or shorter
      than the `NodeHandle` object.
      This was unnecessary restriction.
    + Now the restriction is relaxed. The iterator can live longer, and have
      the same lifetime as `Tree` object.

## [0.4.2]

* A bug around `pull_parser::v7400::Parser::skip_current_node()` is fixed.

### Non-breaking change
* `pull_parser::skip_current_node()` now updates parser status correctly
  (20f4d82d676a).
    + Previously, internal state of the parser is not updated correctly after
      `skip_current_node()`.
      This can cause parser error for correct FBX file, because the parser
      cannot track correct end position of the next node.
      This problem is now fixed.
    + This was overlooked when 5e8d3fbd97e5 was merged...

## [0.4.1]

* Docs are made more detailed, and some examples are added.
* A bug around `pull_parser::v7400::Parser::skip_current_node()` is fixed.

### Added
* `pull_parser::v7400::Parser::is_used()` is added (f55e385c745e).

### Non-breaking change
* `low::FbxHeader::read_fbx_header()` is renamed to `load()` (62f8af93a701).
  The old name is deprecated.
* `pull_parser::skip_current_node()` now updates parser status correctly
  (5e8d3fbd97e5).
    + Previously the parser status is not updated correctly after
      `skip_current_node()`.
      This can cause parser error for correct FBX file, because the parser was
      not able to determine presence of node end marker.
      This problem is now fixed.
* `tree::v7400::Loader::load()` now check parser status more precisely
  (f55e385c745e).
    + Previously, the check is loose and some of already used parser could be
      accepted.
      Now this check is fixed perfectly, and invalid parser is rejected as
      expected.

#### Deprecation
* `low::FbxHeader::read_fbx_header()` is deprecated (62f8af93a701).
    + Use `low::FbxHeader::load()` instead.

## [0.4.0]

* **`dom` module is now split to another crate,
  [`fbxcel-dom`](https://github.com/lo48576/fbxcel-dom)**.
    + If you want to interpret and render FBX data, use it.
* Object properties are supported.
    + Very basic support, but would be useful.
* Huge refactoring around attribute values.
    + Many types, modules, and functions are renamed and moved.
* `{pull_parser,tree}::any` module is added.
    + They provide mostly version-independent way to read and load the FBX data.

### Breaking change
#### Prefer "load" to "visit" for node attributes
* `pull_parser::v7400::attribute::VisitAttribute` is changed to
  `pull_parser::v7400::attribute::LoadAttribute`.
    + `VisitAttributeValue::visit_*` is renamed to `LoadAttribute::load_*`.
* `pull_parser::v7400::attribute::Attributes::visit_*` is renamed to
  `Attributes::load_*`.
* `pull_parser::ParserVersion` is now nonexhaustive.
    + By this change, it is non-breaking change to add new parser version in
      future.

### Non-breaking change
* Attributes slice returned by `tree::v7400::NodeHandle::attributes()` now have
  same lifetime as the tree.
    + The lifetime was mistakenly set too short in previous implementation, but
      this is now fixed.

#### Deprecation
* `pull_parser::v7400::attribute:DirectAttributeValue` is now deprecated.
    + It is moved to `low::v7400::AttributeValue`.
    + Now `DirectAttributeValue` is a type alias to
      `low::v7400::AttributeValue`.
    + The type alias will exist for a while, but will be removed in future
      version.

## [0.3.0]

* `dom` module is completely rewritten.
    + **No compatibility** with old (0.2.0) `dom` module.
* `tree` module is added.
    + This can be enabled with `tree` feature.

### Breaking changes
* `dom` module is completely rewritten.
    + **No compatibility** with old (0.2.0) `dom` module.

### Added
* `tree` module is added.
    + This can be enabled with `tree` feature.
    + `tree::v7400::Tree` manages raw FBX tree data, but do not touch their
      meanings.
* `pull_parser::v7400::attribute::DirectAttributeValue::get_{{types}}_or_type()`
  are added.
    + `{{types}}` are: `bool`, `i16`, `i32`, `i64`, `f32`, `f64`, `arr_bool`,
      `arr_i32`, `arr_i64`, `arr_f32`, `arr_f64`, `string`, and `binary`.
    + This enables using type info at method chain, for example
      `let val = attr.get_i64_or_type().map_err(|ty|
      Error::new("Expected i64 but got {:?}", ty))?;`.

## [0.2.0]

* Syntactic position information for pull parser is supported.
  Syntactic position contains node path, node index, attribute index, etc.
  This will make errors and warnings more detailed and useful.
* Quite basic DOM is implemented.
  This is not yet practically usable.

### Breaking changes
* `pull_parser::v7400::Parser::set_warning_handler()` now requires
  `'static + FnMut(Warning, &SyntacticPosition) -> Result<()>` as warning
  hander (note that `&SyntacticPosition` argument is added).
    + By this change, warning handler can use position information where the
      warning happened.
* `low::FbxHeader::read_fbx_header` now takes `impl std::io::Read` instead of a
  type parameter.

### Added
* `dom` module is added.
    + This can be enabled by `dom` feature, but this is not yet practically
      usable.
* `pull_parser::SyntacticPosition` is added.
* `pull_parser::error::Error::position()` is added.
* `pull_parser::v7400::Parser::skip_current_node()` is added.
* `pull_parser::v7400::attribute::Attributes::iter{,_buffered}` and
  `pull_parser::v7400::attribute::iter` module are added.
* `pull_parser::v7400::attribute::DirectAttributeValue::get_{{types}}()` are
  added.
    + `{{types}}` are: `bool`, `i16`, `i32`, `i64`, `f32`, `f64`, `arr_bool`,
      `arr_i32`, `arr_i64`, `arr_f32`, `arr_f64`, `string`, and `binary`.

## [0.1.0]

Totally rewritten.

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.8.1...develop>
[0.8.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.8.1>
[0.8.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.8.0>
[0.7.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.7.0>
[0.6.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.6.1>
[0.6.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.6.0>
[0.5.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.5.0>
[0.4.4]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.4>
[0.4.3]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.3>
[0.4.2]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.2>
[0.4.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.1>
[0.4.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.0>
[0.3.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.3.0>
[0.2.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.2.0>
[0.1.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
