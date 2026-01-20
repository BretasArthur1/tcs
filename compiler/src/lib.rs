//! TCS Compiler - Schema compiler and code generator for Tape Canonical Serialization
//!
//! This crate provides the compiler pipeline for TCS:
//! - Tokenization of `.tcs` schema files
//! - Parsing into an AST
//! - Schema verification
//! - Rust code generation with wincode derives

pub mod error;
pub mod gen_rust;
pub mod parser;
pub mod tokenizer;
pub mod utils;
pub mod verifier;

pub use error::TcsError;
pub use gen_rust::compile_schema_to_rust;
pub use parser::parse_schema;
pub use tokenizer::tokenize_schema;
pub use verifier::verify_schema;

/// Compile a TCS schema string to Rust code
///
/// This is the main entry point for the compiler. It performs:
/// 1. Tokenization
/// 2. Parsing
/// 3. Verification
/// 4. Code generation
pub fn compile(source: &str) -> Result<String, TcsError> {
    let tokens = tokenize_schema(source)?;
    let schema = parse_schema(&tokens)?;
    verify_schema(&schema)?;
    Ok(compile_schema_to_rust(&schema))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_pipeline() {
        let input = r#"
            package tapedrive;

            enum NodeRole {
                STORAGE = 1;
                VALIDATOR = 2;
            }

            struct BlockHeader {
                uint64 height;
                byte[32] prevHash;
                byte[32] merkleRoot;
                uint64 timestamp;
            }

            message Transaction {
                byte[32] txHash = 1;
                uint64 nonce = 2;
                byte[] data = 3;
                NodeRole senderRole = 4;
            }
        "#;

        let result = compile(input);
        assert!(result.is_ok(), "Compilation failed: {:?}", result.err());

        let code = result.unwrap();

        // Check module wrapper
        assert!(code.contains("pub mod tapedrive {"));

        // Check enum
        assert!(code.contains("pub enum NodeRole"));
        assert!(code.contains("#[repr(u32)]"));

        // Check struct with fixed arrays
        assert!(code.contains("pub struct BlockHeader"));
        assert!(code.contains("pub prev_hash: [u8; 32]"));

        // Check message with optional fields
        assert!(code.contains("pub struct Transaction"));
        assert!(code.contains("pub tx_hash: Option<[u8; 32]>"));
        assert!(code.contains("pub sender_role: Option<NodeRole>"));

        // Check wincode derives
        assert!(code.contains("SchemaRead, SchemaWrite"));
    }

    #[test]
    fn test_error_on_undefined_type() {
        let input = r#"
            struct Bad {
                Unknown field;
            }
        "#;

        let result = compile(input);
        assert!(result.is_err());
    }
}
