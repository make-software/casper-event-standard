# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2023-02-20

### Added

- Added `ces_version` named key to refer to the version of the CES in use.

### Changed

- `CLType::Any` is not supported anymore.
All parts of an event need to be `non-Any` `CLValue`s.
The check happens at the runtime in `to_bytes()` function.
`Any` causes problems to parse events in a generic
way because of the lack of a length indicator.

## [0.1.1] - 2023-02-13

### Added

- `serde` feature. It is disabled by default.
- `emit` and `init` has now `#[cfg(not(target_arch = "wasm32"))]`
implementations that panic when used. It is mostly for the easy of development.

## [0.1.0] - 2023-01-31

### Added

- `casper-event-standard` and `casper-event-standard-macro` crate.
