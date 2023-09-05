# Candid Extractor

This project provides a CLI tool `candid_extractor` which extract candid definition from a canister WASM module.

## Installation

```
cargo install --git https://github.com/lwshang/candid_extractor
```

## Usage

```
candid_extractor path/to/canister.wasm
```

## Generate IC mock wat

```
cargo run --example=generate_mock_wat
```
