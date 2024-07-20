# Swiftness CairoVM Verifier

![Version](https://img.shields.io/badge/v0.0.3-green?style=flat-square&logo=git&logoColor=white&label=version)
![Continuous Integration](https://img.shields.io/github/actions/workflow/status/iosis-tech/swiftness/ci.yml?style=flat-square&logo=githubactions&logoColor=white&label=Continuous%20Integration)

This is the Rust implementation of the Cairo-VM STARK verifier with layouts. The code is inspired by StarkWare's [Cairo-verifier](https://github.com/starkware-libs/cairo-lang) implementation in Cairo0.

### Verify example proof:

```sh
cargo run --release --bin swiftness --features starknet_with_keccak,keccak --no-default-features -- --proof examples/proofs/starknet_with_keccak/cairo0_example_proof.json 
```

## Run Tests

```sh
cargo test
```

### Install wasm-pack
```sh
cargo install wasm-pack
```

### Build WASM:

```sh
cd wasm-binding && wasm-pack build --target bundler --workspace --features starknet_with_keccak,blake2s --no-default-features
```

## Features

- **Usage:**

  - [x] CLI support for easy interaction

- **Implemented Layouts:**

  - [x] dex
  - [x] recursive
  - [x] recursive_with_poseidon
  - [x] small
  - [x] starknet
  - [x] starknet_with_keccak

- **Web Support:**
  - [x] Wasm Support
  - [x] NPM package

## Contributing

Feel free to open issues or submit pull requests to help improve this project.
