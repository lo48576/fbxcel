# Change Log

## [Unreleased]

## [0.4.2]

* A bug around `pull_parser::v7400::Parser::skip_current_node()` is fixed.

### Non-breaking change
* `pull_parser::skip_current_node()` now updates parser status correctly.
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
    + The type alias will exists for a while, but will be removed in future
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

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.4.2...develop>
[0.4.2]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.2>
[0.4.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.1>
[0.4.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.4.0>
[0.3.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.3.0>
[0.2.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.2.0>
[0.1.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
