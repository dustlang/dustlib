# dustlib Core Library v0.2 Specification (Draft)

**Document Type:** Library Specification Draft  
**Status:** Draft for Review  
**Target:** dustlib v0.2  
**Section:** Core Library Architecture  
**Date:** 2026-02-12  
**Copyright:** © 2026 Dust LLC

---

## Overview

dustlib is the foundational standard library for the Dust Programming Language, providing core types, traits, and functions essential for DPL programming. This specification outlines the complete architecture and implementation plan for dustlib v0.2.

---

## Design Principles

### 1.1 DPL Compliance

- **Constraint-first:** All operations respect DPL's constraint model
- **Regime-aware:** Core functionality works across all DPL regimes
- **Deterministic:** No undefined behavior or hidden side effects
- **Verifiable:** All implementations suitable for formal verification

### 1.2 Minimal Foundation

- **Essential only:** Include only what's needed for basic programming
- **Extensible:** Foundation for higher-level libraries
- **Zero-overhead:** No unnecessary runtime cost
- **Memory-safe:** No buffer overflows, use-after-free, or data races

---

## Library Structure

### 2.1 Top-Level Organization

```
dustlib/
├── README.md                    # Project overview
├── LICENSE                      # Dust Open Source License
├── State.toml                   # DPL workspace manifest
├── dustpkg.lock                 # Dependency lock file
├── spec/                        # dustlib-specific specifications
└── sector/                      # DPL code sectors
    ├── dustlib_core/           # Core library sector
    ├── dustlib_memory/         # Memory management sector
    ├── dustlib_fmt/            # Formatting sector
    └── dustlib_prelude/        # Prelude sector
```

### 2.2 Sector Definitions

Each sector represents a focused area of functionality:

- **dustlib_core:** Essential types, traits, and basic operations
- **dustlib_memory:** Memory allocation, pointers, and buffer operations
- **dustlib_fmt:** Text formatting and string operations
- **dustlib_prelude:** Commonly imported items

---

## Core Library Sectors

### 3.1 dustlib_core

**Purpose:** Fundamental types and traits

**Key Components:**
- Primitive types (integers, booleans, characters)
- Core enums (Result, Option)
- Essential traits (Copy, Drop, Debug, Display)
- Basic operators and arithmetic

**File Structure:**
```
dustlib_core/
├── State.toml
└── src/
    ├── lib.ds                   # Main library definition
    ├── prelude.ds               # Prelude definitions
    ├── primitive/              # Primitive type definitions
    │   ├── mod.ds
    │   ├── integer.ds          # Integer types and operations
    │   ├── boolean.ds          # Boolean operations
    │   └── character.ds         # Character operations
    ├── enum/                   # Core enums
    │   ├── mod.ds
    │   ├── result.ds           # Result<T, E> type
    │   └── option.ds           # Option<T> type
    ├── trait/                  # Core traits
    │   ├── mod.ds
    │   ├── copy.ds             # Copy trait
    │   ├── drop.ds             # Drop trait
    │   └── ops.ds              # Operator traits
    └── container/              # Basic containers
        ├── mod.ds
        ├── array.ds            # Fixed-size arrays
        └── slice.ds            # Slice operations
```

**Core Type Definitions:**
```dust
// Primitive integers
type K[Int8], K[Int16], K[Int32], K[Int64]
type K[UInt8], K[UInt16], K[UInt32], K[UInt64]
type K[Bool]
type K[Char]

// Core enums
enum K[Result[T, E]] = K[Ok[T]] | K[Err[E]]
enum K[Option[T]] = K[Some[T]] | K[None]

// Essential traits
trait K[Copy] { }
trait K[Drop] { fn drop(self) -> K[Unit] }
trait K[Debug] { fn fmt(self, f: K[Ptr[K[Formatter]]]) -> K[Result[K[Unit], K[FormatError]]] }
```

### 3.2 dustlib_memory

**Purpose:** Memory management and pointer operations

**Key Components:**
- Memory allocation/deallocation
- Safe pointer operations
- Buffer management
- Memory layout utilities

**File Structure:**
```
dustlib_memory/
├── State.toml
└── src/
    ├── lib.ds                   # Memory library entry point
    ├── alloc.ds                # Allocation operations
    ├── ptr.ds                  # Pointer operations
    ├── buf.ds                  # Buffer operations
    └── layout.ds               # Memory layout utilities
```

**Memory Operations:**
```dust
// Basic allocation
K alloc(K[Size] size) -> K[Ptr[K[Byte]]]
K dealloc(K[Ptr[K[Byte]]] ptr, K[Size] size) -> K[Unit]

// Pointer operations
K ptr_read[K[T]](K[Ptr[T]] ptr) -> K[T]
K ptr_write[K[T]](K[Ptr[T]] ptr, K[T] value) -> K[Unit]
K ptr_offset(K[Ptr[T]] ptr, K[Int] offset) -> K[Ptr[T]]

// Buffer operations
K buf_copy(K[Ptr[K[Byte]]] dst, K[Ptr[K[Byte]]] src, K[Size] len) -> K[Unit]
K buf_compare(K[Ptr[K[Byte]]] a, K[Ptr[K[Byte]]] b, K[Size] len) -> K[Int]
```

### 3.3 dustlib_fmt

**Purpose:** Text formatting and string operations

**Key Components:**
- String formatting
- Display and Debug formatting
- String manipulation utilities
- UTF-8 handling

**File Structure:**
```
dustlib_fmt/
├── State.toml
└── src/
    ├── lib.ds                   # Formatting library entry point
    ├── display.ds              # Display trait and formatting
    ├── debug.ds                # Debug trait and formatting
    ├── string.ds               # String operations
    └── utf8.ds                 # UTF-8 encoding/decoding
```

**Formatting Operations:**
```dust
// Formatting traits
trait K[Display] { 
    fn fmt(self, f: K[Ptr[K[Formatter]]]) -> K[Result[K[Unit], K[FormatError]]] 
}

// Basic string operations
K str_len(K[Ptr[K[Char]]] s) -> K[Size]
K str_compare(K[Ptr[K[Char]]] a, K[Ptr[K[Char]]] b) -> K[Int]
K str_copy(K[Ptr[K[Char]]] dst, K[Ptr[K[Char]]] src, K[Size] len) -> K[Unit]
```

### 3.4 dustlib_prelude

**Purpose:** Common imports for convenience

**Components:**
- Most used types and traits
- Common operations
- Convenience re-exports

**Prelude Contents:**
```dust
// Re-export core types
use dustlib_core::{Result, Option, K[Int8], K[Int32], K[UInt8], K[UInt32], K[Bool]}

// Re-export essential traits
use dustlib_core::{Copy, Drop, Debug, Display}

// Re-export memory operations
use dustlib_memory::{alloc, dealloc, ptr_read, ptr_write}

// Re-export formatting
use dustlib_fmt::{Display, Debug}
```

---

## Implementation Guidelines

### 4.1 K Regime Compliance

All dustlib implementations must:
- Use K Regime for all operations
- Maintain deterministic behavior
- Ensure memory safety
- Avoid undefined behavior

### 4.2 Error Handling Strategy

**Result Type Usage:**
- All fallible operations return `Result<T, E>`
- Error types must be descriptive and actionable
- No panic behavior in library code
- Graceful degradation when possible

### 4.3 Performance Considerations

**Zero-Cost Abstractions:**
- No runtime overhead for abstractions
- Compile-time optimization
- Efficient memory layout
- Minimal function call overhead

---

## Integration with DPL Ecosystem

### 5.1 Compiler Integration

dustlib must integrate with:
- Type system and inference
- Compile-time checking
- Code generation
- Link-time optimization

### 5.2 Toolchain Support

Required tooling:
- **dustfmt:** Formatting for dustlib code
- **dusttest:** Testing framework
- **dustdoc:** Documentation generation
- **dustpkg:** Package management

### 5.3 Extension Points

For other libraries:
- Well-defined trait interfaces
- Extensible type system
- Clear separation of concerns
- Backward compatibility guarantees

---

## Migration Path

### 6.1 Current State Analysis

The existing dustlib structure provides:
- Basic type definitions
- Core trait interfaces
- Partial implementation of Result and Option
- Foundation for memory operations

### 6.2 v0.2 Implementation Steps

1. **Complete Core Types**
   - Finish primitive type implementations
   - Complete Result and Option with all methods
   - Implement essential trait functionality

2. **Memory Management**
   - Implement safe pointer operations
   - Add memory allocation utilities
   - Create buffer manipulation functions

3. **Formatting System**
   - Implement Display and Debug traits
   - Add string manipulation utilities
   - Support UTF-8 encoding/decoding

4. **Testing and Validation**
   - Comprehensive unit tests
   - Integration tests with compiler
   - Performance benchmarks

---

## Examples

### 7.1 Core Library Usage

```dust
// Basic operations using dustlib
use dustlib_prelude::*;

K compute_factorial(K[Int32] n) -> K[Result[K[Int32], K[String]]] {
    if n < 0 {
        K[Err["Negative input"]]
    } else if n <= 1 {
        K[Ok[1]]
    } else {
        match compute_factorial(n - 1) {
            K[Ok[prev]] => K[Ok[n * prev]],
            K[Err[msg]] => K[Err[msg]]
        }
    }
}

K main {
    let result: K[Result[K[Int32], K[String]]] = compute_factorial(5);
    
    match result {
        K[Ok[value]] => emit "Factorial: {value}",
        K[Err[msg]] => emit "Error: {msg}"
    }
}
```

### 7.2 Memory Management

```dust
use dustlib_prelude::*;

K create_buffer(K[Size] size) -> K[Result[K[Ptr[K[Int32]]], K[String]]] {
    let ptr: K[Ptr[K[Byte]]] = alloc(size * 4);
    
    if ptr == null {
        K[Err["Allocation failed"]]
    } else {
        K[Ok[ptr as K[Ptr[K[Int32]]]]
    }
}

K main {
    match create_buffer(10) {
        K[Ok[buf]] => {
            // Use buffer
            ptr_write(buf, 42);
            emit "First element: {ptr_read(buf)}";
            dealloc(buf as K[Ptr[K[Byte]]], 40);
        },
        K[Err[msg]] => emit "Error: {msg}"
    }
}
```

---

## Quality Assurance

### 8.1 Testing Strategy

**Unit Tests:**
- Every public function tested
- Edge cases covered
- Error conditions verified
- Performance benchmarks

**Integration Tests:**
- Compiler integration
- Cross-platform compatibility
- Memory safety validation
- Deterministic behavior verification

### 8.2 Documentation Standards

**API Documentation:**
- Every public item documented
- Usage examples included
- Behavior guarantees specified
- Performance characteristics noted

**Implementation Notes:**
- Algorithm explanations
- Complexity analysis
- Safety invariants
- Memory layout details

---

## Conclusion

dustlib provides the essential foundation for DPL programming while maintaining the language's core principles of constraint-first design, deterministic behavior, and verifiability. The v0.2 implementation establishes a solid base for building higher-level libraries and applications, including the XDV kernel.

The modular sector-based design allows for focused development, clear maintenance boundaries, and extensibility for future DPL versions.

---

© 2026 Dust LLC