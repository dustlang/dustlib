# dustlib_core ops

- Source: `dustlib/sector/dustlib_core/src/ops/mod.ds`
- Forge: `DustlibOps`

## Procedures

### `proc K::add_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `lhs + rhs`

### `proc K::sub_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `0` when `lhs < rhs`
- Otherwise returns `lhs - rhs`

### `proc K::eq_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `1` when equal, else `0`

### `proc K::lt_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `1` when `lhs < rhs`, else `0`

### `proc K::gt_u32(lhs: UInt32, rhs: UInt32) -> UInt32`

- Returns `1` when `lhs > rhs`, else `0`
