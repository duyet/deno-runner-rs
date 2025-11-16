# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive API documentation with examples
- Security features:
  - Variable name validation to prevent code injection
  - JSON-based value serialization (replaces unsafe string interpolation)
  - `#![deny(unsafe_code)]` lint to prevent unsafe code in public API
- Proper error handling with `thiserror` crate
- Custom error types: `RunnerError` with detailed error variants
- Enhanced runtime.js with better error handling
- Additional console methods: `warn`, `info`, `debug`
- Circular reference handling in console output
- Examples directory with comprehensive usage examples:
  - `basic.rs` - Simple JavaScript execution
  - `rust_functions.rs` - Rust function bindings
  - `error_handling.rs` - Error handling patterns
  - `advanced.rs` - Complex use cases
- Security testing for code injection prevention
- Variable name validation tests
- `Debug` trait implementation for `Builder`
- SECURITY.md with security policy
- CONTRIBUTING.md with contribution guidelines
- This CHANGELOG.md

### Changed
- **BREAKING**: Variable binding now uses `const` instead of `let` for safety
- **BREAKING**: Values are now serialized as JSON (requires `Serialize` trait)
- **BREAKING**: Invalid variable names now return an error instead of panicking
- **BREAKING**: Removed unsafe code from the public API
- **BREAKING**: `run()` now returns `Result<String, RunnerError>` instead of `Result<String, anyhow::Error>`
- Improved runtime.js error handling with try-catch blocks
- Better console output formatting with JSON pretty-printing
- Extension builder now includes a name for better debugging

### Fixed
- **SECURITY**: Fixed critical code injection vulnerability in variable binding (CVE-TBD)
- **SECURITY**: Added validation for variable names to prevent malicious code execution
- Typo: `test_bind_numberic` → `test_bind_numeric`
- Runtime initialization error handling

### Dependencies
- Added `serde` 1.0 for safe serialization
- Added `serde_json` 1.0 for JSON handling
- Added `thiserror` 2.0 for better error types

## [0.1.0] - Previous Release

### Added
- Initial release
- Basic JavaScript execution with Deno Core
- Variable binding support
- Rust function registration via `#[op]` macro
- Console support
- Builder pattern for runtime configuration
- Basic test coverage

[Unreleased]: https://github.com/duyet/deno-runner-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/duyet/deno-runner-rs/releases/tag/v0.1.0
