# Change Log

## [Unreleased]
* Object properties are supported.
    + Very basic support, but would be useful.
* Huge refactoring around attribute values.
    + Many types, modules, and functions are renamed and moved.

### Breaking change
#### Prefer "load" to "visit" for node attributes
* `pull_parser::v7400::attribute::VisitAttribute` is changed to
  `pull_parser::v7400::attribute::LoadAttribute`.
    + `VisitAttributeValue::visit_*` is renamed to `LoadAttribute::load_*`.
* `pull_parser::v7400::attribute::Attributes::visit_*` is renamed to
  `Attributes::load_*`.

### Added
* `dom::v7400::Document::objects()` is added.
* `dom::v7400::ObjectNodeId` now implements `Deref<Target=tree::v7400::NodeId>`.
* `dom::v7400::object::property` module and related features are added.
    + This module contains things related to object properties.
    + `dom::v7400::ObjectHandle::direct_properties()` is added.
        * By this method, users can access object properties.
    + `dom::v7400::ObjectHandle::properties_by_native_typename()` is added.
        * By this method, users can access object properties.
* Object-node-type-specific handle types are added.
    + Modules corresponding to node name are added under `dom::v7400::object`.
        * `deformer`: `Deformer` and `SubDeformer` node.
        * `geometry`: `Geometry` node.
        * `material`: `Material` node.
        * `model`: `Model` node.
        * `texture`: `Texture` node.
        * `video`: `Video` node.
        * Some new types of them have the same name (for example
          `model::MeshHandle` and `geometry::MeshHandle`).
          Use with care.

### Non-breaking change
* Attributes slice returned by `tree::v7400::NodeHandle::attributes()` now have
  same lifetime as the tree.
    + The lifetime was mistakenly set too short in previous implementation, but
      this is now fixed.
* Debug (`{:?}`) formatting for `dom::v7400::object::ObjectHandle` became much
  simpler and "usable".
    + Previously it uses default format and dumps contents of `Document` data,
      which can be very large and not so much useful.
    + Now it simply dumps object node ID and object metadata.
      Simple, small, and human-readable.

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

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.3.0...develop>
[0.3.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.3.0>
[0.2.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.2.0>
[0.1.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
