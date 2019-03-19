# Change Log

## [Unreleased]

### Added
* `dom::v7400::ObjectNodeId` now implements `Deref<Target=tree::v7400::NodeId>`.

### Non-breaking change
* Attributes slice returned by `tree::v7400::NodeHandle::attributes()` now have same lifetime as the tree.
    + The lifetime was mistakenly set too short in previous implementation, but this is now fixed.

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

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.3.0...develop>
[0.3.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.3.0>
[0.2.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.2.0>
[0.1.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
