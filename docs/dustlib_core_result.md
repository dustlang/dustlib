# dustlib_core result

- Source: `dustlib/sector/dustlib_core/src/result.ds`
- Forge: `DustlibResult`

## Constants

| Name | Type | Value |
|---|---|---|
| `TAG_OK` | `UInt32` | `0` |
| `TAG_ERR` | `UInt32` | `1` |

## Procedures

### `proc K::is_ok(tag: UInt32) -> UInt32`

- Returns `1` for `TAG_OK`, else `0`

### `proc K::is_err(tag: UInt32) -> UInt32`

- Returns `1` for `TAG_ERR`, else `0`

### `proc K::unwrap_or_u32(tag: UInt32, value: UInt32, fallback: UInt32) -> UInt32`

- Returns `value` for `Ok`
- Returns `fallback` for `Err`

### `proc K::map_ok_increment(tag: UInt32, value: UInt32) -> UInt32`

- Returns `value + 1` for `Ok`
- Returns `value` for `Err`

### `proc K::map_err_to_code(tag: UInt32, err_code: UInt32, mapped_code: UInt32) -> UInt32`

- Returns `mapped_code` for `Err`
- Returns `err_code` for `Ok`
