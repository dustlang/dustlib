# dustlib Workspace

- Top-level file: `dustlib/State.toml`

## Workspace Configuration

### Members

Current workspace members:

- `sector/dustlib_core`

Commented future members are noted in the manifest for additional sectors.

### Package Metadata

| Key | Value |
|---|---|
| `version` | `0.2.0` |
| `edition` | `2026` |
| `authors` | `["Dust Team"]` |
| `description` | `Core standard library for the Dust Programming Language` |
| `license` | `DOSL-1.0` |
| `repository` | `https://github.com/dustlang/dustlib` |

### Profiles

| Profile | opt-level | debug |
|---|---:|---|
| `dev` | `0` | `true` |
| `release` | `3` | `false` |

## Sector Manifests

### `sector/dustlib_core/State.toml`

- Package name: `dustlib_core`
- Entry path: `src/lib.ds`
- Artifact stanza uses `[[bin]]` with `name = "dustlib_core"`

### `sector/dustlib_io/State.toml`

- Package name: `dustlib_io`
- Entry path: `src/lib.ds`
- Depends on `dustlib_core` via relative path

### `sector/dustlib_collections/State.toml`

- Package name: `dustlib_collections`
- Entry path: `src/lib.ds`
- Depends on `dustlib_core` via relative path
