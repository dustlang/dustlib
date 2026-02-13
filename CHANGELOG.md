# Changelog - dustlib (DPL Core Library)

All notable changes to dustlib are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - Unreleased (DPL v0.2)

### Added

#### Core Types
- Complete primitive type definitions (Int8-64, UInt8-64, Float32/64, Bool, Char)
- Result and Option enum implementations with full method sets
- String and slice type definitions

#### Traits
- Copy trait implementation for all primitive types
- Drop trait with proper cleanup semantics
- Display and Debug formatting traits
- Operator traits (Add, Sub, Mul, Div, etc.)

#### Memory Operations (dustlib_memory sector)
- Memory allocation and deallocation
- Safe pointer operations (read, write, offset)
- Buffer operations (copy, set, compare)
- Memory layout utilities

#### Formatting System (dustlib_fmt sector)
- Display trait implementation for all types
- Debug trait for debugging output
- String manipulation utilities
- UTF-8 handling

#### Prelude
- Standard prelude with commonly used types and traits
- Import convenience for core functionality

### Changed

- Sector structure reorganized for better modularity
- Implementation complete for all stubbed functions
- Error handling standardized across all modules

### Fixed

- Type safety issues in pointer operations
- Memory safety in buffer operations
- UTF-8 validation in string handling

### Removed

- Deprecated stub functions without implementations

## [0.1.0] - 2026-02-12

### Added

- Initial dustlib implementation
- Basic type definitions (partial)
- Result and Option enum stubs
- Core trait interfaces (partial)
- Basic formatting trait stubs

### Known Issues

- Many functions left as TODO stubs
- Memory operations require v0.2 expansion
- Incomplete trait implementations

---

Copyright © 2026 Dust LLC