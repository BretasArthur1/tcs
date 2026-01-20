# TCS Example

A demo showing TCS schema features including custom type arrays and nested structs.

## Schema

The example schema (`tapedrive.tcs`) demonstrates:
- Enums with explicit discriminants
- Structs with primitive fields
- Variable-length arrays of custom types (`Color[]`)
- Nested structs containing arrays (`Brush` contains `Color[]`)
- Messages with optional fields

## Build

Requires Rust toolchain with `cargo` and `rustfmt`.

```bash
# Build and run (generates code, compiles, executes)
make

# Or step by step:
make generate  # Generate src/generated.rs from schema
make build     # Compile the example
make run       # Run the demo

# Clean up
make clean
```

## Manual Build

```bash
# Build the TCS compiler (from repo root)
cargo build --release -p tcs-cli

# Generate Rust code
../target/release/tcs gen-rust --input tapedrive.tcs --output src/generated.rs

# Format (optional)
rustfmt src/generated.rs

# Build and run
cargo run
```

## Output

The demo creates sample data, serializes it to bytes, deserializes it back, and verifies the round-trip.
