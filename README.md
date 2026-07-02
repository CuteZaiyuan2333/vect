# Vect

Vect is an experimental Rust stack VM. Its core idea is to treat program
fragments as values: a script can contain numbers, instructions, labels, and
nested recursive blocks, and evaluation can spawn sub-VMs to resolve those
blocks only when an instruction needs them.

**Status: early development.** The ISA shape and runtime prototype are changing
quickly. Recent work has moved comparison into the calculation system, added
numeric/boolean calculation enums, and introduced `mainvec`/`tempvec` as part of
the execution model. The assembler parser is currently not wired in; the parser
code in `src/compiler/vect_asm_compiler.rs` is commented out while the runtime
and ISA are being redesigned.

## Design Direction

Most virtual machines run a flat stream of bytecode. Vect is exploring a more
data-like execution model:

1. **Program as vectors** - script items are stored in `dropvec`, rewound into
   `mainvec`, and evaluated from there. Items popped during evaluation are also
   recorded back into `dropvec`, so the VM keeps an execution trail that can be
   rewound again.
2. **Lazy recursive blocks** - nested expressions are represented as
   `Items::Recursion(Box<Vec<Items>>)`. When an instruction needs a value from a
   recursive block, Vect creates a fresh `Vectvm`, loads that block, rewinds it,
   and evaluates it independently.
3. **Calculations as a shared path** - arithmetic, boolean operations, and
   comparisons are all modeled as `Calculation` variants. Comparisons are no
   longer independent VM instructions; they produce `Types::Bool` through the
   calculation path.
4. **Temporary storage for variables** - `tempvec` and the `Push` instruction
   are the current direction for storing intermediate items so scripts can later
   express variable-like behavior.

## Current Data Model

The main runtime values are defined in `src/isa/isa.rs`:

```rust
pub enum Instructions {
    Cal(Calculation),
    Jump(Logic),
    Push,
}

pub enum Items {
    Element(Instructions),
    Number(Types),
    Recursion(Box<Vec<Items>>),
    Label(String),
    Empty,
    Error,
}
```

Numbers and booleans are carried by `Types` in `src/isa/numbers.rs`:

```rust
pub enum Types {
    Bool(bool),
    I32(i32),
    F32(f32),
    I64(i64),
    F64(f64),
    Error,
}
```

## Calculation ISA

The calculation layer currently has three groups:

| Group | Variants | Result |
| --- | --- | --- |
| `NumberCal` | `Add`, `Sub`, `Mul`, `Div`, `Qtt`, `Rmd` | numeric `Types` |
| `BoolCal` | `And`, `Or`, `Not1`, `Not2`, `NotSidd` | `Types::Bool` or TODO |
| `Compare` | `Lgr`, `Les`, `Eql`, `Eqlgr`, `Eqles` | `Types::Bool` |

Notes:

- `Qtt` is integer/euclidean quotient-style division.
- `Rmd` is remainder/modulo-style calculation.
- `NotSidd` is declared but still marked `todo!()` in the runtime.
- `Jump(Logic)` exists in the ISA, but jump execution is still `todo!()`.

## Script Shape

The planned assembly format is still an S-expression-like layout:

```text
(add, (add, 1, (add, 2, 3)), 4)
```

Conceptually, this becomes a `Vec<Items>` where each item is an instruction, a
typed value, a label, or a recursive sub-script. The old compiler prototype can
parse a similar shape, but it is currently commented out and still refers to the
older instruction model, so this format should be treated as the design target
rather than a working user-facing language.

## Runtime Progress

| Component | Current state |
| --- | --- |
| `Vectvm` | Prototype with `mainvec`, `dropvec`, `tempvec`, `rewind`, `evaluate`, and calculation dispatch |
| Numeric ALU | Implemented for `I32`, `I64`, `F32`, `F64`, including mixed numeric type promotion |
| Boolean ALU | `And`, `Or`, `Not1`, `Not2` implemented for `Bool`; `NotSidd` still TODO |
| Compare/condition path | `Lgr`, `Les`, `Eql`, `Eqlgr`, `Eqles` implemented as calculations returning `Bool` |
| `Push` | Added as the first step toward temp storage / variables |
| `Jump` / logic flow | Declared but not implemented |
| Assembler compiler | Old prototype is present but commented out and out of date |
| Tests | `cargo test` passes, but there are currently no active tests |

## Project Layout

```text
src/
  main.rs                         binary entry point placeholder
  lib.rs                          public module exports
  runtime.rs                      Vectvm runtime prototype
  isa/
    isa.rs                        Instructions and Items
    numbers.rs                    Types and numeric/bool/compare operations
    calculations.rs               calculation enums
    logicflow.rs                  placeholder logic-flow enum
  compiler/
    vect_asm_compiler.rs          old commented-out parser prototype
```

## Run

Build and run the placeholder binary:

```bash
cargo run
```

Run the current test suite:

```bash
cargo test
```

At the moment, `cargo test` verifies compilation only because there are no
active unit tests.

## Roadmap

- Reconnect or rewrite the assembler compiler so it targets the current
  `Instructions::Cal(Calculation)` model.
- Add active tests for `NumberCal`, `BoolCal`, `Compare`, recursion evaluation,
  `Push`, and error cases.
- Finish operand handling and error behavior in calculation dispatch.
- Define variable/temp storage semantics around `tempvec` and `Push`.
- Design and implement `Jump(Logic)` and label/address semantics.
- Decide the public syntax names for the new calculation variants.

