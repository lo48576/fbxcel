# Change Log

## [Unreleased]
* Syntactic position information is supported.
  Syntactic position contains node path, node index, attribute index, etc.
  This will make errors and warnings more detailed and useful.

### Breaking changes
* `pull_parser::v7400::Parser::set_warning_handler()` now requires
  `'static + FnMut(Warning, &SyntacticPosition) -> Result<()>` as warning
  hander (note that `&SyntacticPosition` argument is added).
    + By this change, warning handler can use position information where the
      warning happened.
* `low::FbxHeader::read_fbx_header` now takes `impl std::io::Read` instead of a
  type parameter.

### Added
* `pull_parser::SyntacticPosition` is added.
* `pull_parser::error::Error::position()` is added.
* `pull_parser::v7400::Parser::skip_current_node()` is added.

## [0.1.0]

Totally rewritten.

[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.1.0...develop>
[0.1.0]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
