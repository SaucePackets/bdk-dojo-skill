## Setup

- Update `src/lib.rs` to ensure the BDK bridge notes module is already imported:
  - (No new module needed — this lesson inspects the real miniscript crate and updates bridge notes.)

- Work in the cumulative reference crate: `examples/bdk-dojo-wallet/`, or in your own learner repo/fork. Do not assume the repo is named `bdk-dojo`.
- This lesson does not add new code. It is a research, comparison, and writing lesson.

## Goal

Bridge the toy policy AST to the real `miniscript` crate — compare concepts, read real docs, and produce a bridge note.

## Builds on

- Policy AST (6.1)
- Policy composition (6.2)
- Spending satisfaction (6.3)
- BDK bridge notes (5.x)

## Tasks

This lesson has three tasks. Complete them in order.

### Task 1: Read the miniscript crate docs

Read the official `miniscript` crate documentation or source:

- What is the `Miniscript<Pk>` type and how does it relate to the toy `Policy` enum?
- What is the `Descriptor` type and how does it wrap miniscript policies?
- What are the `Satisfier` and `Satisfaction` types?
- What restrictions does miniscript's type system enforce (cost, malleability, etc.)?

### Task 2: Map toy concepts to real concepts

| Toy Concept | Real Miniscript Concept | What the Toy Hides |
|-------------|------------------------|-------------------|
| `Policy::Key` | `pk()` / `pkh()` | Public key hashing, compressed vs uncompressed |
| `Policy::Older` | `older()` | Relative timelock semantics, CSV opcode |
| `Policy::After` | `after()` | Absolute timelock semantics, CLTV opcode |
| `Policy::And` | `and()` | Witness order, malleability guarantees |
| `Policy::Or` | `or()` with probabilities | Satisfaction cost estimation, branch selection |
| `Policy::Thresh` | `thresh()` | Type unification across branches |
| `Policy::HashLock` | `sha256()` / `hash256()` | Hash function choice (SHA256, HASH256, RIPEMD160) |
| `SatisfactionSummary` | `miniscript::Satisfaction` | Witness stack, script size, resource limits |

### Task 3: Write a bridge note

Create or update `bdk_bridge.rs` with a miniscript bridge note:

```rust
pub fn miniscript_bridge_note() -> BdkBridgeNote {
    BdkBridgeNote {
        toy_concept: "Policy enum with Key, Older, After, And, Or, Thresh, HashLock",
        bdk_concept: "miniscript::Miniscript<Pk> with typed fragments, satisfaction costs, and malleability analysis",
        what_toy_hides: "Script compilation, witness stack construction, type system constraints, resource limits, key hashing",
    }
}
```

## Reference implementation

- `examples/bdk-dojo-wallet/src/bdk_bridge.rs` (extended with `miniscript_bridge_note`)
- The real `miniscript` crate: <https://docs.rs/miniscript/latest/miniscript/>

## Done when

- All three tasks are completed
- The bridge note is added to `src/bdk_bridge.rs`
- `cargo test` passes
- The learner can explain the real miniscript type system in one or two sentences
