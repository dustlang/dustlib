# dustlib_core fmt display

- Source: `dustlib/sector/dustlib_core/src/fmt/display.ds`
- Forge: `DustlibFmtDisplay`

## Procedures

### `proc K::display_str(ptr: UInt64) -> UInt32`

- Calls `puts(ptr)`
- Returns `0`

### `proc K::display_line(ptr: UInt64) -> UInt32`

- Calls `display_str(ptr)` then newline (`putchar(10)`)
- Returns `0`

### `proc K::display_u32(value: UInt32) -> UInt32`

- Emits `display_u32`
- For `value == 0`, prints `'0'` (`putchar(48)`) and returns `0`
- For non-zero values, currently returns `0` without digit rendering
