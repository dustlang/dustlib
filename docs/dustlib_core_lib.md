# dustlib_core lib

- Source: `dustlib/sector/dustlib_core/src/lib.ds`
- Forge: `DustlibCore`

## Constants

| Name | Type | Value |
|---|---|---|
| `ABI_REVISION` | `UInt32` | `3` |
| `MAX_DEFAULT_CAPACITY` | `UInt32` | `1024` |
| `ERR_OK` | `UInt32` | `0` |
| `ERR_INVALID` | `UInt32` | `1` |
| `ERR_DOMAIN_NOT_AVAILABLE` | `UInt32` | `100` |

## Procedures

### `proc K::init() -> UInt32`

- Emits `dustlib_core initialized`
- Returns `ABI_REVISION`

### `proc K::min_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns the smaller input

### `proc K::max_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns the larger input

### `proc K::clamp_u32(value: UInt32, low: UInt32, high: UInt32) -> UInt32`

- Returns:
  - `low` when `value < low`
  - `high` when `value > high`
  - `value` otherwise

### `proc K::bool_to_u32(value: UInt32) -> UInt32`

- Returns `0` when input is `0`
- Returns `1` for non-zero input

### `proc K::capacity_default() -> UInt32`

- Returns `MAX_DEFAULT_CAPACITY`
