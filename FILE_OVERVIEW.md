# Minstark Codebase Overview

This document provides an overview of each file in the Minstark codebase, a minimal STARK (Scalable Transparent ARgument of Knowledge) implementation in Rust. It explains the purpose of each file, key components, underlying concepts, and how they fit into the overall STARK proof pipeline.

## Project Overview
Minstark demonstrates a simplified STARK proof for verifying the correctness of a Poseidon hash chain computation. The pipeline involves:
1. **Computation**: Generate a hash chain (sequence of Poseidon hashes).
2. **Trace Creation**: Build an execution trace from the computation.
3. **Constraint Definition**: Use AIR to define algebraic constraints the trace must satisfy.
4. **Proving**: Commit to the trace via Merkle tree and generate proofs.
5. **Verification**: Check proofs against the commitment.

This is educational and simplified (e.g., uses Merkle proofs instead of full FRI/polynomial commitments).

## File Explanations

### `Cargo.toml`
- **Purpose**: Rust package manifest defining project metadata and dependencies.
- **Key Components**: Package name, version, edition, empty dependencies.
- **Concepts**: Cargo build system; no external crates for minimalism.
- **Role**: Enables building with `cargo build`.

### `README.md`
- **Purpose**: Project documentation with overview, features, and usage.
- **Key Components**: Description, build instructions, repository structure.
- **Concepts**: Open-source documentation.
- **Role**: User guide; no code.

### `src/main.rs`
- **Purpose**: Demo binary entry point for end-to-end STARK proof.
- **Key Components**: Initializes hash chain, computes trace, proves, verifies.
- **Concepts**: STARK workflow (compute → trace → prove → verify).
- **Role**: Runnable example of the pipeline.

### `src/field.rs`
- **Purpose**: Finite field element implementation (simplified u64 wrapping).
- **Key Components**: `BaseElement` with arithmetic ops, implements `FieldElement`/`StarkField`.
- **Concepts**: Finite fields for modular arithmetic in ZKPs.
- **Role**: Mathematical foundation for all operations.

### `src/field_traits.rs`
- **Purpose**: Traits for field elements and STARK fields.
- **Key Components**: `FieldElement`, `StarkField`, `ExtensibleField`.
- **Concepts**: Polymorphism for different field implementations.
- **Role**: Interfaces for type safety and extensibility.

### `src/hash_chain.rs`
- **Purpose**: Hash chain data structure for sequential hashing.
- **Key Components**: `HashChain` with append, trace access.
- **Concepts**: Dependent sequences (like blockchain); represents computation.
- **Role**: The computation being proven.

### `src/trace.rs`
- **Purpose**: Execution trace wrapper for proof systems.
- **Key Components**: `Trace` from hash chain, row access.
- **Concepts**: Trace as intermediate states in STARKs.
- **Role**: Bridges computation to AIR/prover.

### `src/air.rs`
- **Purpose**: Algebraic Intermediate Representation for constraints.
- **Key Components**: `HashChainAir` evaluates transition constraints.
- **Concepts**: Express rules as polynomials (simplified to direct checks).
- **Role**: Enforces computational correctness.

### `src/prover.rs`
- **Purpose**: Generates STARK proofs via Merkle commitments.
- **Key Components**: `Prover` builds Merkle tree, `Proof` with root/paths.
- **Concepts**: Commitments and proofs in ZKPs (simplified).
- **Role**: Creates verifiable proofs.

### `src/verifier.rs`
- **Purpose**: Verifies proofs against commitments.
- **Key Components**: `Verifier` checks Merkle proofs.
- **Concepts**: Proof verification without full data.
- **Role**: Confirms proof validity.

### `src/merkle.rs`
- **Purpose**: Merkle tree for efficient commitments.
- **Key Components**: `MerkleTree`, `MerkleProof`, build/prove/verify.
- **Concepts**: Binary trees for data integrity proofs.
- **Role**: Provides commitments to traces.

### `src/poseidon.rs`
- **Purpose**: Poseidon hash function implementation.
- **Key Components**: `permute`, `poseidon_hash2`.
- **Concepts**: ZK-friendly cryptographic hash.
- **Role**: Hashing for Merkle and chains.

### `src/poseidon_constants.rs`
- **Purpose**: Constants for Poseidon permutation.
- **Key Components**: Rounds, alpha, MDS matrix, round constants.
- **Concepts**: Precomputed parameters for security/efficiency.
- **Role**: Parameterizes Poseidon.

## STARK Pipeline Summary
1. **Setup**: Define computation (hash chain) and constraints (AIR).
2. **Execute**: Run computation to get trace.
3. **Commit**: Build Merkle tree from trace.
4. **Prove**: Generate Merkle proofs for trace elements.
5. **Verify**: Check proofs match root and constraints hold.

This pipeline is simplified; real STARKs add polynomial interpolation, FRI, and randomness for scalability and zero-knowledge.