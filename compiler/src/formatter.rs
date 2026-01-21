//! Formatter for TCS schema files
//!
//! Produces consistently formatted output from a parsed Schema AST.

use tcs_schema::{Definition, DefinitionKind, Schema};

/// Format a Schema AST back into a .tcs source string with consistent formatting.
pub fn format_schema(schema: &Schema) -> String {
    let mut output = String::new();

    // Package declaration
    if let Some(ref pkg) = schema.package {
        output.push_str(&format!("package {};\n", pkg));
        if !schema.definitions.is_empty() {
            output.push('\n');
        }
    }

    // Definitions
    for (i, def) in schema.definitions.iter().enumerate() {
        if i > 0 {
            output.push('\n');
        }
        format_definition(def, &mut output);
    }

    output
}

fn format_definition(def: &Definition, output: &mut String) {
    let keyword = match def.kind {
        DefinitionKind::Enum => "enum",
        DefinitionKind::Struct => "struct",
        DefinitionKind::Message => "message",
    };

    output.push_str(&format!("{} {} {{\n", keyword, def.name));

    for field in &def.fields {
        format_field(field, def.kind, output);
    }

    output.push_str("}\n");
}

fn format_field(field: &tcs_schema::Field, kind: DefinitionKind, output: &mut String) {
    output.push_str("  ");

    match kind {
        DefinitionKind::Enum => {
            // Enum variant: NAME = value;
            output.push_str(&format!("{} = {};\n", field.name, field.field_id));
        }
        DefinitionKind::Struct => {
            // Struct field: type name;
            format_typed_field(field, output);
            output.push_str(";\n");
        }
        DefinitionKind::Message => {
            // Message field: type name = id [deprecated];
            format_typed_field(field, output);
            output.push_str(&format!(" = {}", field.field_id));
            if field.is_deprecated {
                output.push_str(" [deprecated]");
            }
            output.push_str(";\n");
        }
    }
}

fn format_typed_field(field: &tcs_schema::Field, output: &mut String) {
    if let Some(ref type_name) = field.type_ {
        output.push_str(type_name);

        if field.is_array {
            if let Some(size) = field.array_size {
                output.push_str(&format!("[{}]", size));
            } else {
                output.push_str("[]");
            }
        }

        output.push(' ');
        output.push_str(&field.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_schema;
    use crate::tokenizer::tokenize_schema;

    fn parse_and_format(input: &str) -> String {
        let tokens = tokenize_schema(input).unwrap();
        let schema = parse_schema(&tokens).unwrap();
        format_schema(&schema)
    }

    #[test]
    fn test_format_enum() {
        let input = r#"
            enum  NodeRole   {
                STORAGE=1;
                VALIDATOR  =  2;
            }
        "#;
        let formatted = parse_and_format(input);
        assert_eq!(
            formatted,
            "enum NodeRole {\n  STORAGE = 1;\n  VALIDATOR = 2;\n}\n"
        );
    }

    #[test]
    fn test_format_struct() {
        let input = r#"
            struct   BlockHeader  {
                uint64    height;
                byte[32]prevHash;
                byte[32]    merkleRoot;
            }
        "#;
        let formatted = parse_and_format(input);
        assert_eq!(
            formatted,
            "struct BlockHeader {\n  uint64 height;\n  byte[32] prevHash;\n  byte[32] merkleRoot;\n}\n"
        );
    }

    #[test]
    fn test_format_message() {
        let input = r#"
            message Transaction{
                byte[32]txHash=1;
                uint64 nonce =   2;
                byte[]data= 3 [deprecated];
            }
        "#;
        let formatted = parse_and_format(input);
        assert_eq!(
            formatted,
            "message Transaction {\n  byte[32] txHash = 1;\n  uint64 nonce = 2;\n  byte[] data = 3 [deprecated];\n}\n"
        );
    }

    #[test]
    fn test_format_with_package() {
        let input = r#"
            package   mypackage  ;
            struct Foo {
                uint32 x;
            }
        "#;
        let formatted = parse_and_format(input);
        assert_eq!(
            formatted,
            "package mypackage;\n\nstruct Foo {\n  uint32 x;\n}\n"
        );
    }

    #[test]
    fn test_format_multiple_definitions() {
        let input = r#"
            enum Color { RED = 0; BLUE = 1; }
            struct Point { int32 x; int32 y; }
        "#;
        let formatted = parse_and_format(input);
        assert_eq!(
            formatted,
            "enum Color {\n  RED = 0;\n  BLUE = 1;\n}\n\nstruct Point {\n  int32 x;\n  int32 y;\n}\n"
        );
    }

    #[test]
    fn test_format_variable_array() {
        let input = r#"
            struct Data {
                int32[] values;
                string[] names;
            }
        "#;
        let formatted = parse_and_format(input);
        assert_eq!(
            formatted,
            "struct Data {\n  int32[] values;\n  string[] names;\n}\n"
        );
    }
}
