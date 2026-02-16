# Minstark

> A minimal STARK (Scalable Transparent ARgument of Knowledge) implementation in Rust.

This repository implements core components for building and verifying STARK proofs, including field arithmetic, Poseidon hash constants, Merkle trees, and basic prover/verifier logic.

## Features

- Field arithmetic and traits
- Poseidon permutation and constants
- Merkle tree utilities and hash chains
- Basic prover and verifier modules for constructing/verifying STARK proofs

## Repository structure

- `src/` — main Rust source files (field.rs, poseidon.rs, merkle.rs, prover.rs, verifier.rs, etc.)
- `Cargo.toml` — crate manifest

## Prerequisites

- Rust toolchain (stable). Install from https://rustup.rs.

## Build

Build the project with Cargo:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

## Usage

This crate contains library and binary components for experimenting with STARK constructions. Typical workflows:

- Import as a Rust crate and use the field, Poseidon, and Merkle utilities in your project.
- Explore `prover.rs` and `verifier.rs` for example proof-generation and verification flows.

If you want specific examples or a small demo harness added to the repository, open an issue or request it via a PR.

## Contributing

Contributions, bug reports, and improvements are welcome. Please open issues or submit pull requests on the repository.

## License

No license file is included in this repository. If you intend to open-source this project, add a `LICENSE` file (for example, MIT or Apache-2.0) or update the README with the chosen license.

## Maintainer

Repository owner: sidg767
