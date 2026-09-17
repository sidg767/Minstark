# MinSTARK: Implementation Flow and Concepts

## Overview
The `minstark` project is a simplified, pedagogical implementation of a STARK-like proof system. It does not implement the full complexity of a modern STARK (such as FRI-based low-degree testing or succinctness through query phases). Instead, it demonstrates the fundamental cryptographic primitives and the core concepts of Arithmetization (AIR), Trace generation, and Merkle tree commitments.

Specifically, it proves the correct execution of a **Poseidon Hash Chain**. Given a `seed` and a sequence of `inputs`, it computes a chain of hashes. The prover generates a proof that the final hash was computed correctly, and the verifier checks this proof against the public inputs.

## File-by-File Breakdown & Concepts

### 1. `field.rs` & `field_traits.rs` (Finite Field Arithmetic)
**Concept**: STARKs operate over a finite field to avoid precision issues and ensure cryptographic security.
- `field_traits.rs` defines generic traits for finite field elements (`FieldElement`, `StarkField`).
- `field.rs` implements a specific prime field (`BaseElement` or `F`) using the prime modulus $18446744073709551557$ ($2^{64} - 59$). It provides basic modular arithmetic operations (addition, multiplication, inverse, exponentiation).

### 2. `poseidon.rs` & `poseidon_constants.rs` (Cryptographic Hashing)
**Concept**: A STARK-friendly hash function used for both the computation being proved and the Merkle tree.
- Poseidon is chosen because it operates natively on field elements and can be efficiently expressed as algebraic constraints (AIR).
- `poseidon_constants.rs` provides the round constants and Maximum Distance Separable (MDS) matrix.
- `poseidon.rs` implements the `poseidon_hash2(a, b)` function, absorbing two field elements and squeezing one out after applying substitution boxes (S-boxes) and MDS mixing across multiple rounds.

### 3. `hash_chain.rs` (The State Machine / Computation)
**Concept**: The actual program we are trying to prove.
- A `HashChain` starts with an initial `seed`.
- When an `input` is appended, the new state becomes `poseidon_hash2(current_state, input)`.
- It keeps track of every intermediate state, forming the "execution trace".

### 4. `trace.rs` (Execution Trace)
**Concept**: In STARKs, the execution of a program is recorded as a 2D matrix called a trace.
- Since our state is just a single field element, the trace is a 1D vector of `F` (a single column).
- `Trace::from_hash_chain` extracts the sequence of states from the `HashChain`.

### 5. `air.rs` (Algebraic Intermediate Representation)
**Concept**: The translation of the program's logic into algebraic constraints (polynomials) that must evaluate to zero if the computation is correct.
- `HashChainAir` defines the transition constraints.
- `evaluate_transition`: For every step $i$, it enforces that `trace[i+1] == poseidon_hash2(trace[i], inputs[i])`. It does this by returning `trace[i+1] - poseidon_hash2(trace[i], inputs[i])`.
- `verify`: Checks that all transition constraints evaluate to `0`. If they do, the trace is valid.

### 6. `merkle.rs` (Cryptographic Commitments)
**Concept**: A Merkle tree allows a prover to commit to a large amount of data (the execution trace) and later reveal specific elements with succinct proofs of inclusion.
- Implements a binary Merkle tree using `poseidon_hash2`.
- `prove(leaf_index)`: Generates an authentication path (the siblings along the path to the root).
- `verify`: Validates a Merkle proof against a known root.

### 7. `prover.rs` (Proof Generation)
**Concept**: The entity that executes the computation and generates a cryptographic proof of its correctness.
- `Prover::prove` takes the `Trace` and the `inputs`.
- **Step 1 (Sanity Check)**: It uses `HashChainAir` to verify the trace internally.
- **Step 2 (Commitment)**: It builds a `MerkleTree` over all elements in the trace.
- **Step 3 (Proof Assembly)**: Unlike a true succinct STARK (which samples a few random queries), this simplified prover generates a Merkle proof for *every single step* of the trace. The final `Proof` object contains the Merkle root and the collection of all Merkle paths.

### 8. `verifier.rs` (Proof Verification)
**Concept**: The entity that checks the prover's work without having to trust the prover.
- `Verifier::verify` takes the `Proof`, the `expected_root`, the raw `trace_values`, and the `inputs`.
- **Step 1 (Commitment Check)**: It verifies the Merkle path for *every* element in the `trace_values` against the `expected_root`. This ensures the prover hasn't tampered with the trace after committing to it.
- **Step 2 (Constraint Check)**: It feeds the trace to `HashChainAir` to ensure all transitions are valid.
- If both checks pass, the computation is verified.

### 9. `main.rs` (End-to-End Execution Flow)
1. Initializes a starting `seed` and a sequence of `inputs`.
2. Computes the `HashChain` sequentially to find the `final hash`.
3. Extracts the `Trace` from the chain.
4. The `Prover` generates a `Proof` committing to this trace.
5. The `Verifier` takes the `Proof`, the raw trace, and the inputs, and verifies that the computation was valid.
