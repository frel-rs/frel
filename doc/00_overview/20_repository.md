# Repository Structure

```text
/doc/                  # design docs, living specs

/frel/frel/            # Public API surface

/frel/core             # Shared by the compiler and the runtime, must keep it macro-safe
/frel/core/types       # Core types shared across crates (DIP, Color, Instruction enums, errors)
/frel/core/fir         # FIR data structures + opcodes + tiny encoder/decoder helpers

/frel/compiler         # Fragment Compiler (proc-macro) → emits FIR blobs
/frel/compiler/dsl     # DSL surface (token types, pest grammar) for tests/tools only

/frel/encoding         # Event/Patch binary encodings shared by renderer<->adapter

/frel/runtime/         # 
/frel/runtime/linker   # FIR -> instance linker
/frel/runtime/store    # Reactive store implementation
/frel/runtime/render   # Instruction applier + layout engine + patch generation

/frel/adapter/browser  # Browser adapter PoC (inline mode first, worker later)

/frel/lib              # Higher level fragments (tables, trees, editors etc.)

/tests/                # Integration tests 

/utils/                # shared utilities (can be split later)
```

## Published crates

```text
frel                   # Public API surface 
frel-core              # Core types shared across crates
frel-compiler          # proc-macro for compiling DSL to FIR
frel-encoding          # Separated so adapters can depend on it without pulling in the runtime
frel-runtime           # General, adapter-independent runtime
frel-adapter-browser   # Browser adapter
frel-lib               # Higher level fragments
```