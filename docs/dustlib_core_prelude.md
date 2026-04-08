# dustlib_core prelude

- Source: `dustlib/sector/dustlib_core/src/prelude.ds`
- Forge: `DustlibPrelude`

## Procedures

### `proc K::init() -> UInt32`

- Emits `dustlib prelude initialized`
- Returns `0`

### `proc K::print_ok(message: UInt64) -> UInt32`

- Calls `puts(message)`
- Emits newline via `putchar(10)`
- Returns `0`

### `proc K::assert_true(value: UInt32) -> UInt32`

- Returns `0` when `value != 0`
- Emits `prelude assert failed` and returns `1` when `value == 0`
