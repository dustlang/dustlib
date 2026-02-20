# dustlib_core primitive module

- Source: `dustlib/sector/dustlib_core/src/primitive/mod.ds`
- Forge: `DustlibPrimitive`

## Procedures

### `proc K::self_test() -> UInt32`

Self-test flow:

- Computes `a = checked_add_u32(10, 20)`
- Computes `b = bool_and(1, 1)`
- Returns `0` only if `a == 30` and `b == 1`
- Returns `1` otherwise

This function validates basic wiring of integer and boolean helper modules.
