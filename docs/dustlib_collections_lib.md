# dustlib_collections lib

- Source: `dustlib/sector/dustlib_collections/src/lib.ds`
- Forge: `DustlibCollections`

## Constants

| Name | Type | Value |
|---|---|---|
| `DEFAULT_VEC_CAPACITY` | `UInt32` | `16` |
| `MAX_VEC_CAPACITY` | `UInt32` | `1048576` |

## Procedures

### `proc K::vec_new() -> UInt32`

- Returns `DEFAULT_VEC_CAPACITY`

### `proc K::vec_grow(current_capacity: UInt32) -> UInt32`

- Returns `DEFAULT_VEC_CAPACITY` when `current_capacity == 0`
- Returns `MAX_VEC_CAPACITY` when `current_capacity >= MAX_VEC_CAPACITY`
- Otherwise returns `current_capacity * 2`

### `proc K::vec_push_count(current_len: UInt32, capacity: UInt32) -> UInt32`

- Returns `current_len + 1` when `current_len < capacity`
- Returns `current_len` when full

### `proc K::vec_pop_count(current_len: UInt32) -> UInt32`

- Returns `0` when empty
- Returns `current_len - 1` otherwise

### `proc K::map_bucket_index(hash: UInt32, bucket_count: UInt32) -> UInt32`

- Returns `0` when `bucket_count == 0`
- Returns `hash` when `hash < bucket_count`
- Returns `bucket_count - 1` otherwise
