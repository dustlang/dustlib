# dustlib – Core Library for the Dust Programming Language

**dustlib** is the core standard library for the Dust Programming Language
(DPL).  dustlib is authored in **Dust** itself.  It defines the
fundamental types, collections, math routines, string utilities,
error definitions and concurrency abstractions that every DPL
program can rely upon.  By staying inside the language, dustlib
demonstrates how Dust’s constraint‑first, regime‑typed semantics
enable powerful libraries while preserving determinism【709885362819930†L42-L64】.

## Project Layout

The repository follows the same structure used by the Dust compiler
and other tools:

```
dustlib/
├── README.md          # this file
├── LICENSE            # Dust Open Source License
├── Dust.toml          # manifest describing the library
├── spec/              # optional specification documents for dustlib
│   └── 01-overview.md # overview of dustlib’s design and semantics
└── sector/
    └── dustlib/
        ├── Sector.toml        # sector manifest used by the build system
        └── src/
            ├── lib.ds      # top‑level forge re‑exporting modules
            ├── collections.ds
            ├── math.ds
            ├── string.ds
            ├── error.ds
            ├── concurrency.ds
            └── effects.ds
```

Each `.ds` file defines one or more `forge` blocks containing
`shape` declarations and `process` definitions.  These are analogous
to modules and functions in other languages.  Currently the bodies
of many processes are left empty or marked as “TODO” because a
complete implementation depends on additional compiler and runtime
features being developed in tandem with the K‑regime expansion.

## Manifest (Dust.toml)

The `Dust.toml` file defines package metadata for dustlib and its
sectors tailored forthe Dust package manager (dustpkg).  It lists 
the core library as a workspace member and records dependencies, 
versions and features.

## Specification

The `spec/` directory contains draft specification documents for
dustlib.  These files describe the intent, design and semantics of
the library.  They are additive to the main DPL specification and
may evolve over time.  See `spec/01-overview.md` for an introduction.

## Contributing

Contributions are welcome!  Please consult the DPL specification and
the v0.2 roadmap for guidance on regime semantics and library design.
All additions should be accompanied by updates to the dustlib spec
where appropriate.

---

Copyright © 2026 Dust LLC