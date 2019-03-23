# Change Log

## [Unreleased]

Split from `fbxcel` crate, because objects-related APIs would be frequently
updated or fixed as breaking changes.

The changelog below is change from `fbxcel::dom` module as of `fbxcel-0.3.0`.

* Object properties are supported.
    + Very basic support, but would be useful.
* `any` module is added.
    + They provide mostly version-independent way to read and load the FBX data.

### Added
* `v7400::Document::objects()` is added.
* `v7400::ObjectNodeId` now implements
  `Deref<Target=fbxcel::tree::v7400::NodeId>`.
* `v7400::object::property` module and related features are added.
    + This module contains things related to object properties.
    + `v7400::ObjectHandle::direct_properties()` is added.
        * By this method, users can access object properties.
    + `v7400::ObjectHandle::properties_by_native_typename()` is added.
        * By this method, users can access object properties.
* Object-node-type-specific handle types are added.
    + Modules corresponding to node name are added under `v7400::object`.
        * `deformer`: `Deformer` and `SubDeformer` node.
        * `geometry`: `Geometry` node.
        * `material`: `Material` node.
        * `model`: `Model` node.
        * `nodeattribute`: `NodeAttribute` node.
        * `texture`: `Texture` node.
        * `video`: `Video` node.
        * Some new types of them have the same name (for example
          `model::MeshHandle` and `geometry::MeshHandle`).
          Use with care.
    + Meaningful objects traversing is partially supported, but they are
      unstable and subject to change.
      Be careful.

### Non-breaking change
* Debug (`{:?}`) formatting for `v7400::object::ObjectHandle` became much
  simpler and "usable".
    + Previously it uses default format and dumps contents of `Document` data,
      which can be very large and not so much useful.
    + Now it simply dumps object node ID and object metadata.
      Simple, small, and human-readable.

[Unreleased]: <https://github.com/lo48576/fbxcel/commits/develop>
<!--
[Unreleased]: <https://github.com/lo48576/fbxcel/compare/v0.0.1...develop>
[0.0.1]: <https://github.com/lo48576/fbxcel/releases/tag/v0.1.0>
-->
