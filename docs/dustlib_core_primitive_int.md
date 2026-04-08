# dustlib_core primitive int

- Source: `dustlib/sector/dustlib_core/src/primitive/int.ds`
- Forge: `DustlibPrimitiveInt`

## Constants

| Name | Type | Value |
|---|---|---|
| `U32_MAX` | `UInt32` | `4294967295` |

## Procedures

### `proc K::checked_add_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `0` on overflow risk (`lhs > U32_MAX - rhs`)
- Otherwise returns `lhs + rhs`

### `proc K::saturating_add_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `U32_MAX` on overflow risk
- Otherwise returns `lhs + rhs`

### `proc K::wrapping_add_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns wrapped result when overflow risk exists:
  - `rhs - (U32_MAX - lhs) - 1`
- Otherwise returns `lhs + rhs`

### `proc K::checked_sub_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `0` when `lhs < rhs`
- Otherwise returns `lhs - rhs`

### `proc K::min_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns smaller value

### `proc K::max_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns larger value
