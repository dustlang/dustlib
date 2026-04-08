# dustlib_core str

- Source: `dustlib/sector/dustlib_core/src/str/mod.ds`
- Forge: `DustlibStr`

## Procedures

### `proc K::len(ptr: UInt64, known_len: UInt32) -> UInt32`

- Returns `0` when `ptr == 0`
- Returns `known_len` otherwise

### `proc K::is_empty(ptr: UInt64, known_len: UInt32) -> UInt32`

- Returns `1` when computed length is `0`
- Returns `0` otherwise

### `proc K::starts_with(text_len: UInt32, prefix_len: UInt32) -> UInt32`

- Returns `0` when `prefix_len > text_len`
- Returns `1` otherwise

### `proc K::ends_with(text_len: UInt32, suffix_len: UInt32) -> UInt32`

- Returns `0` when `suffix_len > text_len`
- Returns `1` otherwise

### `proc K::contains(text_len: UInt32, needle_len: UInt32) -> UInt32`

- Returns `1` when `needle_len == 0`
- Returns `0` when `needle_len > text_len`
- Returns `1` otherwise

### `proc K::split_at(text_len: UInt32, mid: UInt32) -> UInt32`

- Returns `1` when `mid > text_len` (invalid split)
- Returns `0` otherwise
