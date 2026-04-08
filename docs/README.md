# dustlib Documentation

This directory contains complete markdown documentation for the `dustlib` library.

## Scope

- Workspace and sector configuration
- Full API documentation for `dustlib_core` modules
- API documentation for `dustlib_io` and `dustlib_collections`
- Bridge and macro notes used by `xdv-os` integration

## Document Index

### Library-Level

- `dustlib_overview.md`
- `dustlib_workspace.md`
- `getting_started_guide.md`

### dustlib_core

- `dustlib_core_lib.md`
- `dustlib_core_prelude.md`
- `dustlib_core_option.md`
- `dustlib_core_result.md`
- `dustlib_core_fmt.md`
- `dustlib_core_fmt_debug.md`
- `dustlib_core_fmt_display.md`
- `dustlib_core_mem.md`
- `dustlib_core_ops.md`
- `dustlib_core_primitive.md`
- `dustlib_core_primitive_int.md`
- `dustlib_core_primitive_bool.md`
- `dustlib_core_slice.md`
- `dustlib_core_str.md`
- `dustlib_core_xdv_os_bridge.md`
- `dustlib_core_macros.md`

### Other Sectors

- `dustlib_io_lib.md`
- `dustlib_collections_lib.md`

## Source Mapping

| Source File | Forge | Documentation |
|---|---|---|
| `sector/dustlib_core/src/lib.ds` | `DustlibCore` | `dustlib_core_lib.md` |
| `sector/dustlib_core/src/prelude.ds` | `DustlibPrelude` | `dustlib_core_prelude.md` |
| `sector/dustlib_core/src/option.ds` | `DustlibOption` | `dustlib_core_option.md` |
| `sector/dustlib_core/src/result.ds` | `DustlibResult` | `dustlib_core_result.md` |
| `sector/dustlib_core/src/fmt/mod.ds` | `DustlibFmt` | `dustlib_core_fmt.md` |
| `sector/dustlib_core/src/fmt/debug.ds` | `DustlibFmtDebug` | `dustlib_core_fmt_debug.md` |
| `sector/dustlib_core/src/fmt/display.ds` | `DustlibFmtDisplay` | `dustlib_core_fmt_display.md` |
| `sector/dustlib_core/src/mem/mod.ds` | `DustlibMem` | `dustlib_core_mem.md` |
| `sector/dustlib_core/src/ops/mod.ds` | `DustlibOps` | `dustlib_core_ops.md` |
| `sector/dustlib_core/src/primitive/mod.ds` | `DustlibPrimitive` | `dustlib_core_primitive.md` |
| `sector/dustlib_core/src/primitive/int.ds` | `DustlibPrimitiveInt` | `dustlib_core_primitive_int.md` |
| `sector/dustlib_core/src/primitive/bool.ds` | `DustlibPrimitiveBool` | `dustlib_core_primitive_bool.md` |
| `sector/dustlib_core/src/slice/mod.ds` | `DustlibSlice` | `dustlib_core_slice.md` |
| `sector/dustlib_core/src/str/mod.ds` | `DustlibStr` | `dustlib_core_str.md` |
| `sector/dustlib_core/src/xdv_os_bridge.ds` | `DustlibBridge` | `dustlib_core_xdv_os_bridge.md` |
| `sector/dustlib_core/src/macros/mod.rs` | macro definitions | `dustlib_core_macros.md` |
| `sector/dustlib_io/src/lib.ds` | `DustlibIo` | `dustlib_io_lib.md` |
| `sector/dustlib_collections/src/lib.ds` | `DustlibCollections` | `dustlib_collections_lib.md` |
