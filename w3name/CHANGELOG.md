# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.8] - 2022-09-23

### Added
- add feature to build protoc from src (#22)

### Fixed
- reset version number to latest published
- quote feature name in docs.rs build flags
- use correct rustc flag in docs.rs build
- handle 404 errors, fix error handling for client.resolve (#23)
- remove optional keyword from protobuf definition
- fix invalid keyword in cargo.toml
- fix nix build (hopefully)
- fix cbor issue (need to serialize Value & Validity as byte strings)

### Other
- release (#30)
- release main (#27)
- release main (#25)
- manually bump versions, since ci is confused
- release main (#24)
- release main (#21)
- release main (#20)
- release main (#19)
- release main (#16)
- release main (#12)
- cargo fmt
- doc comments for Revision
- avoid unwrap() in doc comments
- add doc comments for WritableName, impl PartialEq
- move Hasher to its own module
- add doc comments for crate & Name struct
- add LICENSE.md & update cargo metadata
- serialize Revision to/from cbor
- aarch64 build still needs work...
- try setting PROTOC env var
- try using vendored openssl
- convert errors to error_stack reports
- working publish & create commands for cli
- resolve mostly works, but cbor errors when validating
- scaffold out cli app

## [0.1.8] - 2022-09-23

### Added
- add feature to build protoc from src (#22)

### Fixed
- quote feature name in docs.rs build flags
- use correct rustc flag in docs.rs build
- handle 404 errors, fix error handling for client.resolve (#23)
- remove optional keyword from protobuf definition
- fix invalid keyword in cargo.toml
- fix nix build (hopefully)
- fix cbor issue (need to serialize Value & Validity as byte strings)

### Other
- release main (#27)
- release main (#25)
- manually bump versions, since ci is confused
- release main (#24)
- release main (#21)
- release main (#20)
- release main (#19)
- release main (#16)
- release main (#12)
- cargo fmt
- doc comments for Revision
- avoid unwrap() in doc comments
- add doc comments for WritableName, impl PartialEq
- move Hasher to its own module
- add doc comments for crate & Name struct
- add LICENSE.md & update cargo metadata
- serialize Revision to/from cbor
- aarch64 build still needs work...
- try setting PROTOC env var
- try using vendored openssl
- convert errors to error_stack reports
- working publish & create commands for cli
- resolve mostly works, but cbor errors when validating
- scaffold out cli app
# Changelog

## [0.1.7](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.6...w3name-v0.1.7) (2022-09-23)


### Features

* add feature to build protoc from src ([#22](https://github.com/yusefnapora/w3name-rust-client/issues/22)) ([2bfb208](https://github.com/yusefnapora/w3name-rust-client/commit/2bfb20822f3c53a9198b58c57b324debec5e721d))


### Bug Fixes

* handle 404 errors, fix error handling for client.resolve ([#23](https://github.com/yusefnapora/w3name-rust-client/issues/23)) ([234ea11](https://github.com/yusefnapora/w3name-rust-client/commit/234ea118efef86c0fa454a3e9f4bbb98e7929ff9))
* quote feature name in docs.rs build flags ([b61d9eb](https://github.com/yusefnapora/w3name-rust-client/commit/b61d9eb24f4092584eb6c8f268a1661a1740cd56))
* remove optional keyword from protobuf definition ([bb6a40d](https://github.com/yusefnapora/w3name-rust-client/commit/bb6a40d9c4cca243db29c846a3655c0ef638e43a))
* use correct rustc flag in docs.rs build ([1cc8fea](https://github.com/yusefnapora/w3name-rust-client/commit/1cc8fead87dd1dbd0720dcc7ff9daee2ff938a2c))


### Miscellaneous Chores

* release 0.1.2 ([c3139bd](https://github.com/yusefnapora/w3name-rust-client/commit/c3139bd7c171400b8523a6e01662405078f63854))
* release 0.1.3 ([4f49b11](https://github.com/yusefnapora/w3name-rust-client/commit/4f49b11cccd6ea813d7643e79ea41add5fb0d88b))
* release 0.1.4 ([1b03792](https://github.com/yusefnapora/w3name-rust-client/commit/1b03792607f5b7a6ce930176c1ae1bb36336c8e1))

## [0.1.6](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.5...w3name-v0.1.6) (2022-09-23)


### Bug Fixes

* use correct rustc flag in docs.rs build ([1cc8fea](https://github.com/yusefnapora/w3name-rust-client/commit/1cc8fead87dd1dbd0720dcc7ff9daee2ff938a2c))

## [0.1.4](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.4...w3name-v0.1.4) (2022-09-22)


### Features

* add feature to build protoc from src ([#22](https://github.com/yusefnapora/w3name-rust-client/issues/22)) ([2bfb208](https://github.com/yusefnapora/w3name-rust-client/commit/2bfb20822f3c53a9198b58c57b324debec5e721d))


### Bug Fixes

* handle 404 errors, fix error handling for client.resolve ([#23](https://github.com/yusefnapora/w3name-rust-client/issues/23)) ([234ea11](https://github.com/yusefnapora/w3name-rust-client/commit/234ea118efef86c0fa454a3e9f4bbb98e7929ff9))
* remove optional keyword from protobuf definition ([bb6a40d](https://github.com/yusefnapora/w3name-rust-client/commit/bb6a40d9c4cca243db29c846a3655c0ef638e43a))


### Miscellaneous Chores

* release 0.1.2 ([c3139bd](https://github.com/yusefnapora/w3name-rust-client/commit/c3139bd7c171400b8523a6e01662405078f63854))
* release 0.1.3 ([4f49b11](https://github.com/yusefnapora/w3name-rust-client/commit/4f49b11cccd6ea813d7643e79ea41add5fb0d88b))
* release 0.1.4 ([1b03792](https://github.com/yusefnapora/w3name-rust-client/commit/1b03792607f5b7a6ce930176c1ae1bb36336c8e1))

## [0.1.4](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.4...w3name-v0.1.4) (2022-09-21)


### Features

* add feature to build protoc from src ([#22](https://github.com/yusefnapora/w3name-rust-client/issues/22)) ([2bfb208](https://github.com/yusefnapora/w3name-rust-client/commit/2bfb20822f3c53a9198b58c57b324debec5e721d))


### Bug Fixes

* remove optional keyword from protobuf definition ([bb6a40d](https://github.com/yusefnapora/w3name-rust-client/commit/bb6a40d9c4cca243db29c846a3655c0ef638e43a))


### Miscellaneous Chores

* release 0.1.2 ([c3139bd](https://github.com/yusefnapora/w3name-rust-client/commit/c3139bd7c171400b8523a6e01662405078f63854))
* release 0.1.3 ([4f49b11](https://github.com/yusefnapora/w3name-rust-client/commit/4f49b11cccd6ea813d7643e79ea41add5fb0d88b))
* release 0.1.4 ([1b03792](https://github.com/yusefnapora/w3name-rust-client/commit/1b03792607f5b7a6ce930176c1ae1bb36336c8e1))

## [0.1.4](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.3...w3name-v0.1.4) (2022-09-21)


### Miscellaneous Chores

* release 0.1.4 ([1b03792](https://github.com/yusefnapora/w3name-rust-client/commit/1b03792607f5b7a6ce930176c1ae1bb36336c8e1))

## [0.1.3](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.2...w3name-v0.1.3) (2022-09-21)


### Miscellaneous Chores

* release 0.1.3 ([4f49b11](https://github.com/yusefnapora/w3name-rust-client/commit/4f49b11cccd6ea813d7643e79ea41add5fb0d88b))

## [0.1.2](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.1...w3name-v0.1.2) (2022-09-21)


### Miscellaneous Chores

* release 0.1.2 ([c3139bd](https://github.com/yusefnapora/w3name-rust-client/commit/c3139bd7c171400b8523a6e01662405078f63854))

## [0.1.1](https://github.com/yusefnapora/w3name-rust-client/compare/w3name-v0.1.0...w3name-v0.1.1) (2022-09-21)


### Bug Fixes

* remove optional keyword from protobuf definition ([bb6a40d](https://github.com/yusefnapora/w3name-rust-client/commit/bb6a40d9c4cca243db29c846a3655c0ef638e43a))
