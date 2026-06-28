# Vect

An experimental stack virtual machine with an unusual execution model: scripts are stored, rewound, and evaluated on demand through nested sub-VMs.

**Status: early development.** A minimal proof-of-concept VM and assembler compiler exist today. Only a three-instruction test ISA is wired up; a full-featured VM and compiler are still ahead.

## The idea

Most VMs separate *compilation* (source → bytecode) from *execution* (fetch/decode/run in a flat instruction stream). Vect blurs that line:

1. **Dual buffers** — A script lives in `dropvec`, then is **rewound** into `scriptvec` (execution stack). Items popped during evaluation are pushed back into `dropvec`, forming an execution trace.
2. **Replay by rewind** — To re-run a script, call `rewind()` again. No recompilation required. This also sets the stage for control flow (e.g. loops and jumps) without throwing away program state.
3. **Recursion as sub-programs** — Nested expressions are not a separate AST pass. They are stored as `Recursion` items (deferred sub-scripts). When evaluation needs one, it **spawns a fresh `Vectvm`**, loads the sub-script, and lets that instance evaluate itself. Instructions and operands can both be recursive; each branch resolves independently at the point of use.

The result is a lazy, stack-based evaluator where the program is both data and executable structure.

## Script format

Scripts use parenthesized, comma-separated groups (S-expression style):

```
(add, (add, 1, (add, 2, 3)), 4)
```

After compilation, the top-level group becomes a flat `Vec<Items>` with three kinds of nodes: instructions, numbers, and nested `Recursion` blocks.

## Instruction set (test ISA)

| Instruction | Operands        | Result |
|-------------|-----------------|--------|
| `add`       | `isize`, `isize` | `isize` |
| `lgr`       | `isize`, `isize` | `bool`  |
| `jump`      | `bool`, `usize`  | control flow |

Example (conceptual):

```
(jump, (lgr, (add, (add, 1, 2), 3), (add, 1, 1)), addr)
```

## Current progress

| Component | State |
|-----------|-------|
| `Vectvm` runtime | Working prototype — `add` implemented and tested |
| `lgr`, `jump` | Stubs only (`todo!`) |
| `VectAsmCompiler` | Lexer + parser for nested scripts; outputs `Vec<Items>` |
| Full ISA / tooling | Not started |

Run tests:

```bash
cargo test
```

## Project layout

```
src/
  isa.rs                  — Instructions and Items
  runtime.rs              — Vectvm (dropvec, scriptvec, evaluate, rewind)
  compiler/
    vect_asm_compiler.rs  — Vect assembly parser
```

## Roadmap

- Finish `lgr` and `jump` in the runtime
- Define jump addressing semantics (offset into `dropvec` / script position)
- Expand the instruction set and compiler
- Build a complete, production-ready VM and assembler

---

Contributions and design feedback welcome while the architecture is still taking shape.
