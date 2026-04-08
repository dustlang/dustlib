# dustlib_core primitive bool

- Source: `dustlib/sector/dustlib_core/src/primitive/bool.ds`
- Forge: `DustlibPrimitiveBool`

## Procedures

### `proc K::is_true(value: UInt32) -> UInt32`

- Returns `1` for non-zero values, else `0`

### `proc K::is_false(value: UInt32) -> UInt32`

- Returns `1` only for zero

### `proc K::bool_not(value: UInt32) -> UInt32`

- Returns `1` for zero, else `0`

### `proc K::bool_and(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `1` only when both inputs are non-zero

### `proc K::bool_or(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `0` only when both inputs are zero
- Returns `1` otherwise

### `proc K::bool_xor(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `1` when inputs differ
- Returns `0` when inputs match
