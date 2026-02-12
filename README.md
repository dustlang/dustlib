# dustlib

`dustlib` is the core standard library for the Dust Programming Language (DPL). It provides fundamental types, traits, functions, and macros essential for writing DPL programs, comparable to standard libraries in other systems programming languages.

## Overview

This library is written entirely in DPL and serves as the foundational layer upon which higher-level libraries and applications are built. It adheres strictly to DPL's constraint-first, regime-typed semantics.

### Key Components (in `sector/dustlib_core/src/`)

*   **Primitives:** Basic types like integers (`u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `usize`, `isize`), `bool`, `char`, `str`, and `unit`.
*   **Core Enums:** `Result<T, E>` for error handling and `Option<T>` for optional values.
*   **Formatting:** Traits `Display` and `Debug` for printing and debugging, along with the `FormatError` type.
*   **Fundamental Traits:** Traits like `Copy`, `Drop`, `Add`, which define core behaviors for types.
*   **Basic Collections:** (To be implemented in future files/directories) Core data structures like `Vec` (dynamic array) and `HashMap`.
*   **Macros:** Essential macros like `panic!`, `assert!`, `println!`.

## Structure

The `dustlib` project follows the standard DPL project layout using `sector`s:

```
Okay, the `Dust.toml` file has been removed from the directory structure as requested. The structure now relies on `State.toml` for project/workspace and sector-level configuration.

```
dustlib/
├── README.md                     # Project overview for dustlib
├── LICENSE                       # The Dust Open Source License (DOSL) text
├── State.toml                    # Top-level State workspace manifest (for DPL tooling/build scripts/tests)
├── dustpkg.lock                  # Lock file for `dustpkg`, pinning dependency versions
├── sector/                       # Root directory for DPL code sectors (analogous to 'crate/')
│   ├── dustlib_core/             # The main core library sector
│   │   ├── State.toml            # (DPL-specific) Manifest for the core sector
│   │   └── src/                  # Source files for the core library
│   │       ├── lib.ds            # Main library definition (already started)
│   │       ├── prelude.ds        # Definitions automatically imported into scopes using the library
│   │       ├── result.ds         # Detailed implementation of Result<T, E> and related items
│   │       ├── option.ds         # Detailed implementation of Option<T> and related items
│   │       ├── fmt/              # Formatting traits and utilities
│   │       │   ├── mod.ds        # Module declaration for fmt
│   │       │   ├── debug.ds      # Debug trait definition and helpers
│   │       │   └── display.ds    # Display trait definition and helpers
│   │       ├── mem/              # Memory-related utilities (if appropriate for core lib)
│   │       │   ├── mod.ds
│   │       │   └── ...
│   │       ├── ops/              # Operator traits (Add, Sub, etc.)
│   │       │   ├── mod.ds
│   │       │   └── ...
│   │       ├── primitive/        # Definitions for primitive types and their inherent methods
│   │       │   ├── mod.ds
│   │       │   ├── int.ds        # Methods for i32, u64, etc.
│   │       │   └── bool.ds
│   │       ├── slice/            # Slice type and operations
│   │       │   ├── mod.ds
│   │       │   └── ...
│   │       ├── str/              # String type and operations (if str methods are here, or in a dedicated string sector)
│   │       │   ├── mod.ds
│   │       │   └── ...
│   │       └── macros/           # Core macros (e.g., assert!, panic!, println!) if implemented in DPL
│   │           ├── mod.ds
│   │           └── ...
│   ├── dustlib_collections/      # (Future) Potential separate sector for collections like Vec, HashMap
│   │   ├── State.toml            # (DPL-specific) Manifest for the collections sector
│   │   └── src/
│   │       └── lib.ds            # Collection definitions
│   └── dustlib_io/               # (Future) Potential separate sector for I/O operations
│       ├── State.toml            # (DPL-specific) Manifest for the io sector
│       └── src/
│           └── lib.ds            # I/O definitions
├── tests/                        # Integration tests written in DPL (using .ds files)
│   ├── smoke_test.ds
│   └── ...
├── benches/                      # Benchmark files (if applicable)
├── docs/                         # Non-generated documentation, guides
│   └── getting_started_guide.md
├── build.rs                      # (Rust-specific) Build script if needed for compilation setup (Potentially replaced by a DPL equivalent or handled by State/Dust tools)
├── .gitignore                    # Standard git ignore file
└── .github/                      # GitHub-specific configs (workflows, issue templates)
    └── workflows/
        └── ci.yml                # CI configuration (e.g., using dusttest, dustdoc, etc.)
```
```

Each `.ds` file defines one or more `forge` blocks containing
`shape` declarations and `process` definitions.  These are analogous
to modules and functions in other languages.  Currently the bodies
of many processes are left empty or marked as “TODO” because a
complete implementation depends on additional compiler and runtime
features being developed in tandem with the K‑regime expansion.

## Manifest (Dust.toml)

The `Dust.toml` file defines package metadata for dustlib and its
sectors tailored forthe Dust package manager (dustpkg).  It lists 
the core library as a workspace member and records dependencies, 
versions and features.

## Specification

The `spec/` directory contains draft specification documents for
dustlib.  These files describe the intent, design and semantics of
the library.  They are additive to the main DPL specification and
may evolve over time.  See `spec/01-overview.md` for an introduction.

## Contributing

Contributions are welcome!  Please consult the DPL specification and
the v0.2 roadmap for guidance on regime semantics and library design.
All additions should be accompanied by updates to the dustlib spec
where appropriate.

---

Copyright © 2026 Dust LLC