# dustlib_core fmt debug

- Source: `dustlib/sector/dustlib_core/src/fmt/debug.ds`
- Forge: `DustlibFmtDebug`

## Constants

| Name | Type | Value |
|---|---|---|
| `KIND_SCALAR` | `UInt32` | `1` |
| `KIND_PAIR` | `UInt32` | `2` |
| `KIND_LIST` | `UInt32` | `3` |

## Procedures

### `proc K::debug_open(kind: UInt32) -> UInt32`

- Emits opening delimiter via `putchar`:
  - scalar: `<` (60)
  - pair: `(` (40)
  - list/default: `[` (91)
- Returns `0`

### `proc K::debug_field(name: UInt64, value: UInt64) -> UInt32`

- Emits `name`, then `: `, then `value`
- Returns `0`

### `proc K::debug_close(kind: UInt32) -> UInt32`

- Emits closing delimiter via `putchar`:
  - scalar: `>` (62)
  - pair: `)` (41)
  - list/default: `]` (93)
- Returns `0`
