# soroban-sdk-codegen

Multi-language client SDK generator CLI for Stellar Soroban contracts.

## Overview
`soroban-sdk-codegen` parses Soroban WASM contract specifications (`env.contract_spec()` or JSON) and outputs strongly-typed, ergonomic client SDKs:
- **TypeScript**: Generates interface definitions, RPC invocation methods, parameter marshalling, and Promise returns with exact bigints and Address strings.
- **Python**: Generates `@dataclass` models, IntEnum classes, and Python client classes with complete type hints for seamless integration into autonomous agent frameworks.

## CLI Usage
```bash
# Generate TypeScript and Python SDKs from contract WASM
soroban-sdk-codegen generate --wasm path/to/contract.wasm --lang all --out-dir ./sdk

# Generate only Python SDK
soroban-sdk-codegen generate --spec path/to/spec.json --lang python --out-dir ./py_client

# Inspect and print contract specification JSON
soroban-sdk-codegen inspect-spec --wasm path/to/contract.wasm
```
