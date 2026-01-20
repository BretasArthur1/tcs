.PHONY: all generate build run clean

TCS := ../target/release/tcs
SCHEMA := tapedrive.tcs
OUTPUT := src/generated.rs

all: generate build run

# Build the TCS compiler first
$(TCS):
	cargo build --release -p tcs-cli --manifest-path ../Cargo.toml

# Generate Rust code from schema
generate: $(TCS)
	$(TCS) gen-rust --input $(SCHEMA) --output $(OUTPUT)
	rustfmt $(OUTPUT)
	@echo "Generated: $(OUTPUT)"

# Build the example
build: generate
	cargo build

# Run the example
run: build
	cargo run

clean:
	cargo clean
	rm -f $(OUTPUT)
