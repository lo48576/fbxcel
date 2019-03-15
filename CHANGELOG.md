# Change Log

## [Unreleased]

* Add `domcast` module.
    + This is a new DOM feature.
    + This can be enabled with `domcast` feature.
* Add `tree` module.
    + This can be enabled with `tree` feature.
* Non-strict DOM load support is removed.
* DOM load error handling are changed.
    + Internal representation of `dom::error::LoadError` got hidden to reduce
      incompatibility due to internal changes.
    + New error handling is still not perfect, and might change in future.
* DOM object connections handling are changed.
    + This is mainly internal changes, but some of exposed APIs are changed.
    + For example, connections are now iterable in same order as raw FBX data.
* New object node is supported:
    + `Model`, `Scene`.

### Breaking changes
* Non-strict DOM load support is removed.
  `dom::v7400::Loader::strict` is removed, and the loader will always interpret
  data strictly.
* `dom::error::LoadError` is now `struct` and internal representation is hidden.
  Users should not expect any guarantee except that it implements
  `std::error::Error` trait.
    + Currently some other interfaces are public (including
      `impl From<CoreLoadError> for LoadError` and `impl Fail for LoadError`),
      but they might (or might not) be changed or removed in future.
* `dom::v7400::Core::load()` now uses dedicated error type.
  That is, it now returns `Result<Core, dom::v7400::error::CoreLoadError>`
  instead of `Result<Core, dom::error::LoadError>`.
* `dom::v7400::object::connection::{ConnectionRef, ConnectionEdge}` is removed.
    + Use `dom::v7400::object::connection::Connection` instead.
    + Functions returning these types are renamed and have different return
      types now.
* `dom::v7400::object::ObjectId::{sources,destinations}_undirected` are replaced
  by `ObjectId::{sources,destinations}`.
    + Now connections iterated by the returned iterators have guaranteed orders.
    + Now they return `impl Iterator<Item = &Connection>`.

### Added
* `domcast` module is added.
    + This is a new DOM feature.
    + `dom` feature and module will be replaced with this.
* `dom::v7400::error::CoreLoadError` type is added.
* `dom::v7400::object::scene` module and related types are added.
    + By these types, users can get scenes and their root object IDs.
* `dom::v7400::object::model` module and related types are added.
* Add `tree` module.
    + This can be enabled with `tree` feature.
    + `tree::v7400::Tree` manages raw FBX tree data, but do not touch their
      meanings.
* DOM node ID types (such as `NodeId`, `ObjectNodeId`, `SceneNodeId`) now
  implements type conversion traits (`From`, `Into`, `DowncastId`, and `Deref`).
* `pull_parser::v7400::attribute::DirectAttributeValue::get_{{types}}_or_type()`
  are added.
    + `{{types}}` are: `bool`, `i16`, `i32`, `i64`, `f32`, `f64`, `arr_bool`,
      `arr_i32`, `arr_i64`, `arr_f32`, `arr_f64`, `string`, and `binary`.
    + This enables using type info at method chain, for example
      `let val = attr.get_i64_or_type().map_err(|ty|
      Error::new("Expected i64 but got {:?}", ty))?;`.

### Fixed
#### `dom::v7400`
* Objects graph structure is now correctly loadable.
    + Previously, multiple nodes are created in internal graph for single
      objects, and this prevents objects connections from correctly being
      tracked.
      Now this is fixed.
    + Previously, multiple connections are considered same with same src/dest
      pair and different label.
      This prevented some (correct) connections from loaded.
      Now this is fixed.

### Others
* `dom` module has little more detailed logging (mainly `trace` level).

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

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.2.0...develop>
[0.2.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.2.0>
[0.1.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
