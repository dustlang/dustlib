# dustlib_core fmt module

- Source: `dustlib/sector/dustlib_core/src/fmt/mod.ds`
- Forge: `DustlibFmt`

## Procedures

### `proc K::format_debug_pair(name: UInt64, value: UInt64) -> UInt32`

- Calls:
  - `debug_open(2)`
  - `debug_field(name, value)`
  - `debug_close(2)`
- Returns `0`

### `proc K::format_display_line(text: UInt64) -> UInt32`

- Returns `display_line(text)`
