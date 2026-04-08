# dustlib_core option

- Source: `dustlib/sector/dustlib_core/src/option.ds`
- Forge: `DustlibOption`

## Constants

| Name | Type | Value |
|---|---|---|
| `TAG_NONE` | `UInt32` | `0` |
| `TAG_SOME` | `UInt32` | `1` |

## Procedures

### `proc K::is_some(tag: UInt32) -> UInt32`

- Returns `1` for `TAG_SOME`, else `0`

### `proc K::is_none(tag: UInt32) -> UInt32`

- Returns `1` for `TAG_NONE`, else `0`

### `proc K::unwrap_or_u32(tag: UInt32, value: UInt32, default_value: UInt32) -> UInt32`

- Returns `value` when `is_some(tag) == 1`
- Returns `default_value` otherwise

### `proc K::map_increment_u32(tag: UInt32, value: UInt32) -> UInt32`

- Returns `value + 1` for `Some`
- Returns `value` for `None`

### `proc K::expect(tag: UInt32, message: UInt64) -> UInt32`

- Returns `0` for `Some`
- For `None`, prints `message` + newline and returns `1`
