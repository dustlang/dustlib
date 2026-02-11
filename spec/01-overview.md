# dustlib Specification – Overview

## Purpose

`dustlib` is the core library for the Dust Programming Language (DPL).
It provides fundamental shapes, processes and effects used by
every Dust program.  Being written entirely in Dust, the library
serves as a reference for idiomatic DPL code and demonstrates how
the language’s constraint‑first semantics enable expressive yet
deterministic abstractions【709885362819930†L42-L64】.  This
document outlines the design of dustlib and the semantics of its
components.

## Modules

`dustlib` is organised into several **forges**, each contained in a
separate source file:

| Forge          | Description                                                |
|---------------|------------------------------------------------------------|
| `collections` | Defines shapes for arrays and lists and processes to
                 create and manipulate them.  Arrays are modelled as a
                 length and a memory handle (`Mem`), reflecting the K‑regime
                 memory semantics.                                           |
| `math`        | Provides numerical processes such as `sum`, `max` and
                 `min`.  These processes operate on arrays of numbers and
                 return deterministic results.                               |
| `string`      | Contains string utilities including concatenation and
                 splitting.  Strings are immutable sequences of Unicode
                 scalar values.                                              |
| `error`       | Defines a minimal `Error` shape with a message field and
                 processes for constructing and inspecting errors.           |
| `concurrency` | Declares a `Thread<T>` shape and `spawn`/`join` processes.
                 In the v0.2 prototype these are placeholders; future
                 implementations will wrap deterministic K‑regime concurrency
                 primitives【548468680421956†L121-L129】.                     |
| `effects`     | Exposes processes representing side effects.  The current
                 prototype includes a no‑op effect and an example failure
                 effect.                                                     |

## Shapes and Processes

The shapes defined in dustlib mirror those described in the main DPL
specification but are re‑expressed as user‑defined constructs.  For
instance, `collections.Array<T>` has fields `length: Int` and
`data: Mem`, where `Mem` is a K‑regime resource type.  Processes
such as `collections.new<T>(n: Int) → Array<T>` allocate memory and
return a new array.  All allocations and frees must adhere to the
memory effects and constraints defined in the DPL spec【709885362819930†L42-L64】.

Most processes in the prototype are stubs; their bodies are empty or
contain TODO comments.  Implementing them requires additional
compiler and runtime support (e.g. deterministic allocators and
schedulers) that will be developed in subsequent milestones.

## Regime Neutrality

Although dustlib depends on K‑regime primitives such as `Mem` and
`Thread`, it strives to remain regime‑neutral by not performing
quantum (`Q`) or phase (`Φ`) operations.  Cross‑regime isolation
rules still apply【709885362819930†L174-L202】; dustlib functions do
not call into other regimes directly.  Future regime‑specific
libraries (`dustlib_k`, `dustlib_q`, etc.) will extend dustlib for
regime‑specific operations.

## Evolution

This document is a **draft** and will evolve as dustlib is
implemented and refined.  All additions will follow the DPL
versioning rules for minor releases: new features are additive and
must not break existing programs【587782563170635†L29-L34】.  A
comprehensive specification for each forge and process will be
provided once implementations are stable.