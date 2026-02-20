# dustlib_io lib

- Source: `dustlib/sector/dustlib_io/src/lib.ds`
- Forge: `DustlibIo`

## Constants

| Name | Type | Value |
|---|---|---|
| `MODE_READ` | `UInt32` | `0` |
| `MODE_WRITE` | `UInt32` | `1` |
| `MODE_APPEND` | `UInt32` | `2` |

## Procedures

### `proc K::open_read(path: UInt64) -> UInt64`

- Returns `open(path, MODE_READ)`

### `proc K::open_write(path: UInt64) -> UInt64`

- Returns `open(path, MODE_WRITE)`

### `proc K::open_append(path: UInt64) -> UInt64`

- Returns `open(path, MODE_APPEND)`

### `proc K::read_all(fd: UInt64, buffer: UInt64, max_bytes: UInt32) -> UInt32`

- Returns `read(fd, buffer, max_bytes)`

### `proc K::write_all(fd: UInt64, buffer: UInt64, bytes: UInt32) -> UInt32`

- Returns `write(fd, buffer, bytes)`

### `proc K::close_fd(fd: UInt64) -> UInt32`

- Returns `close(fd)`
