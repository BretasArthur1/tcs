# TCS
(Tape Canonical Serialization)

A high-performance, schema-based binary serialization format for Tapedrive.

TCS generates Rust code from `.tcs` schema files, producing types with efficient binary serialization. The encoding is canonical (deterministic), making it suitable for cryptographic hashing and blockchain applications.

## Features

- **Schema-driven**: Define your data structures in `.tcs` files
- **Canonical encoding**: Deterministic byte output for cryptographic applications
- **High performance**: 20-60x faster than BCS in benchmarks
- **Fixed-size arrays**: Native support for `byte[32]` hash fields
- **Zero-copy deserialization**: Placement initialization for maximum speed

## Background

TCS is derived from [Kiwi](https://github.com/evanw/kiwi), a binary serialization format created by [Evan Wallace](https://github.com/evanw) for Figma's multiplayer syncing and file storage. The schema syntax is intentionally similar.

Unlike Protocol Buffers, TCS is non-self-describing—the schema is not embedded in the data stream, resulting in more compact output. This is ideal when both endpoints already know the schema (as in Tapedrive's node communication). TCS differs from Kiwi by using fixed-width little-endian integers instead of varints, ensuring canonical output for cryptographic hashing.

## Quick Start

### 1. Define a Schema

Create a `schema.tcs` file:

```proto
package chain;

enum Status {
    PENDING = 0;
    CONFIRMED = 1;
    FAILED = 2;
}

// Structs have required fields (no Option in generated code)
struct Transaction {
    byte[32] sender;
    byte[32] recipient;
    uint64 amount;
    byte[64] signature;
}

// Messages have optional fields (for schema evolution)
message Block {
    uint64 height = 1;
    byte[32] parent = 2;
    Transaction[] transactions = 3;
    Status status = 4;
}
```

### 2. Generate Rust Code

```bash
tcs gen-rust --input schema.tcs --output generated.rs
```

### 3. Use Generated Types

```rust
use chain::{Block, Transaction, Status};

// Struct fields are required - no Option wrapper
let transaction = Transaction {
    sender: [1u8; 32],
    recipient: [2u8; 32],
    amount: 1000,
    signature: [0u8; 64],
};

// Message fields are optional - wrapped in Option
let block = Block {
    height: Some(100),
    parent: Some([0u8; 32]),
    transactions: Some(vec![transaction]),
    status: Some(Status::Confirmed),
};

let bytes = block.to_bytes();
let decoded = Block::from_bytes(&bytes).unwrap();
```

**Key difference from Protocol Buffers:** Struct fields are always required and generate direct types (`u64`, `Vec<T>`), not `Option<T>`. Use `message` when you need optional fields for backwards compatibility.

## Schema Syntax

### Types

| TCS Type   | Rust Type   | Description                    |
|------------|-------------|--------------------------------|
| `bool`     | `bool`      | Boolean (1 byte)               |
| `byte`     | `u8`        | Unsigned 8-bit integer         |
| `int`      | `i32`       | Signed 32-bit integer          |
| `uint`     | `u32`       | Unsigned 32-bit integer        |
| `int64`    | `i64`       | Signed 64-bit integer          |
| `uint64`   | `u64`       | Unsigned 64-bit integer        |
| `float`    | `f32`       | 32-bit float (avoid for canonical) |
| `string`   | `String`    | UTF-8 string                   |
| `byte[N]`  | `[u8; N]`   | Fixed-size byte array          |
| `T[]`      | `Vec<T>`    | Variable-length array          |

### Definitions

**Enums** - Fixed set of values with explicit discriminants:
```
enum Role {
    VALIDATOR = 1;
    ARCHIVER = 2;
    RELAYER = 3;
}
```

**Structs** - Fixed fields, all required:
```
struct Header {
    uint64 height;
    byte[32] parent;
    byte[32] root;
    uint64 timestamp;
}
```

**Messages** - Fields with IDs, all optional (for schema evolution):
```
message Request {
    byte[32] hash = 1;
    uint64 sequence = 2;
    byte[] data = 3;
}
```

## CLI Commands

```bash
# Generate Rust code from schema
tcs gen-rust --input schema.tcs --output generated.rs

# Validate a schema file
tcs validate --input schema.tcs
```

## Performance

TCS is **20-60x faster** than BCS (Binary Canonical Serialization) used in Aptos and Sui.

| Benchmark | BCS | TCS | Speedup |
|-----------|-----|-----|---------|
| Serialize 120B | 477 ns | 19 ns | 25x |
| Serialize 4KB | 5.31 µs | 130 ns | 41x |
| Deserialize 120B | 544 ns | 27 ns | 20x |
| Deserialize 4KB | 16.4 µs | 264 ns | 62x |
| Throughput (4KB) | 238-736 MiB/s | 14-29 GiB/s | 40x |

Output size is ~2-3% larger due to fixed-width length prefixes.

## TCS vs BCS Encoding

| Aspect              | BCS                              | TCS                               |
|---------------------|----------------------------------|-----------------------------------|
| Canonical           | Yes                              | Yes                               |
| Integer encoding    | Fixed-width little-endian        | Fixed-width little-endian         |
| Sequence length     | ULEB128 (variable)               | u64 (fixed 8 bytes)               |
| Optional fields     | 0x00/0x01 prefix                 | Field ID prefix                   |
| Primary use case    | Blockchain (Aptos, Sui)          | Tapedrive                         |

Both formats produce deterministic output suitable for cryptographic hashing.

## License

MIT
