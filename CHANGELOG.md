# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2023-02-13

### Added

- `serde` feature. It is disabled by default.
- `emit` and `init` has now `#[cfg(not(target_arch = "wasm32"))]` implementations that panic when used. It is mostly for the easy of development.

## [0.1.0] - 2023-01-31

### Added

- `casper-event-standard` and `casper-event-standard-macro` crate.
