# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.3](https://github.com/redbadger/crux/compare/crux_core-v0.7.2...crux_core-v0.7.3) - 2024-02-02

### Changes

- Allow bring-your-own serializer/deserializer for the `Bridge`. This is not a
  breaking change as the existing bridge interface is the same. We have
  introduced a new `BridgeWithSerializer`, where you can plug in your own
  serialization format (e.g. JSON), but be aware that you'll have to provide
  your own serialization/deserialization shell-side code (in
  TypeScript/Swift/Kotlin).

- Re-exports `crux_macros` as `crux_core::macros` in order to help keep the
  version of macros in sync with the core. Prefer importing macros as
  `crux_core::macros`, rather than `crux_macros`

## [0.7.2](https://github.com/redbadger/crux/compare/crux_core-v0.7.1...crux_core-v0.7.2) - 2024-01-26

### Fixed

- fix clippy lints

### Other

- Introduce a Compose capability which allows composition of other capabilities
- Introduce `Never` type for capabilities that don't request effects
- Effect derive macro now allows skipping variants (to support `Never`
  operations)
- Make render capability is now Clone to support composition
- remove uuid unused wasm-bindgen feature flag

## [0.7.1](https://github.com/redbadger/crux/compare/crux_core-v0.7.0...crux_core-v0.7.1) - 2024-01-11

### Other

- update deps for Rust, Web, iOS and Android
- update examples to crux_core 0.7

## [0.7.0](https://github.com/redbadger/crux/compare/crux_core-v0.6.5...crux_core-v0.7.0) - 2023-12-03

### Fixed

- fix doc tests

### Other

- improve typegen error handling

## [0.6.5](https://github.com/redbadger/crux/compare/crux_core-v0.6.4...crux_core-v0.6.5) - 2023-11-29

### Other

- root deps
- rustfmt
- full error message

## [0.6.4](https://github.com/redbadger/crux/compare/crux_core-v0.6.3...crux_core-v0.6.4) - 2023-10-25

### Other

- update deps
- update leptos examples to remove Scope
- deps + tweaks
- avoid unnecessary coercion
- Remove existing generated java files before generating the new set
- deps
- deps
