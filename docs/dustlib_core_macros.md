# dustlib_core macros

- Source: `dustlib/sector/dustlib_core/src/macros/mod.rs`

## Purpose

Prototype macro definitions and design notes for core macro behavior.

## Current State

The file is currently written in Rust-style `macro_rules!` syntax as a placeholder/design artifact rather than stable Dust `.ds` macro syntax.

Defined macro names:

- `panic!`
- `assert!`
- `assert_eq!`
- `println!`
- `format!`
- `todo!`
- `unreachable!`

## Notes

- The file itself states macro syntax is hypothetical pending finalized DPL macro system details.
- Treat this as implementation guidance and intent, not finalized runtime/compiler behavior.
