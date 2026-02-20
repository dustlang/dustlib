# dustlib_core slice

- Source: `dustlib/sector/dustlib_core/src/slice/mod.ds`
- Forge: `DustlibSlice`

## Procedures

### `proc K::len(base: UInt64, count: UInt32) -> UInt32`

- Returns `0` when `base == 0`
- Returns `count` otherwise

### `proc K::is_empty(base: UInt64, count: UInt32) -> UInt32`

- Returns `1` when `len(base, count) == 0`
- Returns `0` otherwise

### `proc K::in_bounds(index: UInt32, count: UInt32) -> UInt32`

- Returns `1` when `index < count`
- Returns `0` otherwise

### `proc K::first_index(count: UInt32) -> UInt32`

- Returns `0` when empty
- Returns `1` when non-empty

### `proc K::last_index(count: UInt32) -> UInt32`

- Returns `0` when empty
- Returns `count - 1` when non-empty

## Note

`first_index` currently uses `1` as the non-empty sentinel/index value.
