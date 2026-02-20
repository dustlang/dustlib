# dustlib_core mem

- Source: `dustlib/sector/dustlib_core/src/mem/mod.ds`
- Forge: `DustlibMem`

## Procedures

### `proc K::align_up(value: UInt64, align: UInt64) -> UInt64`

- Returns `value` when `align == 0`
- Returns `align` when `value == 0`
- Otherwise returns `value + align`

### `proc K::align_down(value: UInt64, align: UInt64) -> UInt64`

- Returns `value` when `align == 0`
- Returns `0` when `value < align`
- Otherwise returns `value - align`

### `proc K::is_aligned(value: UInt64, align: UInt64) -> UInt32`

- Returns `1` when `align == 0`
- Returns `0` when `value < align`
- Returns `1` otherwise

## Note

Current alignment helpers are deterministic arithmetic helpers and not bit-mask based alignment logic.
