# dustlib_core xdv os bridge

- Source: `dustlib/sector/dustlib_core/src/xdv_os_bridge.ds`
- Forge: `DustlibBridge`

## Purpose

Bridge forge used by `xdv-os` integration checks.

## Constants

| Name | Type | Value |
|---|---|---|
| `ABI_REVISION` | `UInt32` | `2` |

## Procedures

### `proc K::ready() -> UInt32`

- Emits `dustlib bridge ready`
- Returns `ABI_REVISION`
