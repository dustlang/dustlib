# dustlib Overview

- Project path: `dustlib/`
- Workspace manifest: `dustlib/State.toml`
- Specification draft: `dustlib/spec/dustlib-v0.2-specification.md`

## Purpose

`dustlib` is the foundational standard library for Dust projects. It provides core helpers used by runtime and OS layers.

## Active Sectors

| Sector | Status | Manifest | Source Root |
|---|---|---|---|
| `dustlib_core` | Active workspace member | `sector/dustlib_core/State.toml` | `sector/dustlib_core/src/` |
| `dustlib_io` | Present in repo | `sector/dustlib_io/State.toml` | `sector/dustlib_io/src/` |
| `dustlib_collections` | Present in repo | `sector/dustlib_collections/State.toml` | `sector/dustlib_collections/src/` |

## Design Notes

- K-domain helper functions are the dominant implemented API surface.
- Most procedures return explicit status/integer values.
- Higher-level abstractions in the spec are partially represented by practical helper forges in source.

## Integration

`dustlib` is consumed by `xdv-os` and related components via path dependencies in workspace manifests.
