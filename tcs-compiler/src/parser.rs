//! Parser for TCS schema files

use lazy_static::lazy_static;
use regex::Regex;
use tcs_schema::{Definition, DefinitionKind, Field, Schema};

use crate::error::TcsError;
use crate::tokenizer::Token;
use crate::utils::{error, quote};

lazy_static! {
    static ref IDENTIFIER: Regex = Regex::new(r"^[A-Za-z_][A-Za-z0-9_]*$").unwrap();
    static ref EQUALS: Regex = Regex::new(r"^=$").unwrap();
    static ref SEMICOLON: Regex = Regex::new(r"^;$").unwrap();
    static ref INTEGER: Regex = Regex::new(r"^-?\d+$").unwrap();
    static ref LEFT_BRACE: Regex = Regex::new(r"^\{$").unwrap();
    static ref RIGHT_BRACE: Regex = Regex::new(r"^\}$").unwrap();
    static ref ARRAY_TOKEN: Regex = Regex::new(r"^\[\]$").unwrap();
    static ref FIXED_ARRAY_TOKEN: Regex = Regex::new(r"^\[(\d+)\]$").unwrap();
    static ref ENUM_KEYWORD: Regex = Regex::new(r"^enum$").unwrap();
    static ref STRUCT_KEYWORD: Regex = Regex::new(r"^struct$").unwrap();
    static ref MESSAGE_KEYWORD: Regex = Regex::new(r"^message$").unwrap();
    static ref PACKAGE_KEYWORD: Regex = Regex::new(r"^package$").unwrap();
    static ref DEPRECATED_TOKEN: Regex = Regex::new(r"^\[deprecated\]$").unwrap();
    static ref EOF: Regex = Regex::new(r"^$").unwrap();
}

/// Parse tokens into a Schema AST
pub fn parse_schema(tokens: &[Token]) -> Result<Schema, TcsError> {
    let mut definitions = Vec::new();
    let mut package_text = None;
    let mut index = 0;

    fn current_token<'a>(tokens: &'a [Token], index: usize) -> &'a Token {
        tokens.get(index).expect("Unexpected end of tokens")
    }

    fn eat(tokens: &[Token], index: &mut usize, test: &Regex) -> bool {
        if test.is_match(&current_token(tokens, *index).text) {
            *index += 1;
            true
        } else {
            false
        }
    }

    fn expect(
        tokens: &[Token],
        index: &mut usize,
        test: &Regex,
        expected: &str,
    ) -> Result<(), TcsError> {
        if !eat(tokens, index, test) {
            let tok = current_token(tokens, *index);
            return Err(error(
                &format!("Expected {} but found {}", expected, quote(&tok.text)),
                tok.line,
                tok.column,
            ));
        }
        Ok(())
    }

    fn unexpected_token(tokens: &[Token], index: &mut usize) -> TcsError {
        let tok = current_token(tokens, *index);
        error(
            &format!("Unexpected token {}", quote(&tok.text)),
            tok.line,
            tok.column,
        )
    }

    // Handle package declaration
    if eat(tokens, &mut index, &PACKAGE_KEYWORD) {
        if index >= tokens.len() {
            return Err(error("Expected identifier after package", 0, 0));
        }
        let pkg_tok = current_token(tokens, index);
        expect(tokens, &mut index, &IDENTIFIER, "identifier")?;
        package_text = Some(pkg_tok.text.clone());
        expect(tokens, &mut index, &SEMICOLON, "\";\"")?;
    }

    // Parse definitions one by one
    while index < tokens.len() && !eat(tokens, &mut index, &EOF) {
        let kind = if eat(tokens, &mut index, &ENUM_KEYWORD) {
            DefinitionKind::Enum
        } else if eat(tokens, &mut index, &STRUCT_KEYWORD) {
            DefinitionKind::Struct
        } else if eat(tokens, &mut index, &MESSAGE_KEYWORD) {
            DefinitionKind::Message
        } else {
            return Err(unexpected_token(tokens, &mut index));
        };

        // Definition name
        let name_tok = current_token(tokens, index);
        expect(tokens, &mut index, &IDENTIFIER, "identifier")?;
        expect(tokens, &mut index, &LEFT_BRACE, "\"{\"")?;

        // Collect fields
        let mut fields = Vec::new();
        while !eat(tokens, &mut index, &RIGHT_BRACE) {
            let mut type_opt = None;
            let mut is_array = false;
            let mut array_size = None;
            let mut is_deprecated = false;

            if kind != DefinitionKind::Enum {
                // Read the type token
                let t_tok = current_token(tokens, index);
                expect(tokens, &mut index, &IDENTIFIER, "identifier")?;

                // Check for array notation
                let next_tok = current_token(tokens, index);
                if eat(tokens, &mut index, &ARRAY_TOKEN) {
                    // Variable-length array: type[]
                    is_array = true;
                } else if let Some(caps) = FIXED_ARRAY_TOKEN.captures(&next_tok.text) {
                    // Fixed-size array: type[N]
                    index += 1;
                    is_array = true;
                    let size_str = caps.get(1).unwrap().as_str();
                    array_size = Some(size_str.parse::<usize>().map_err(|_| {
                        error(
                            &format!("Invalid array size {}", quote(size_str)),
                            next_tok.line,
                            next_tok.column,
                        )
                    })?);
                }
                type_opt = Some(t_tok.text.clone());
            }

            // Field name
            let f_tok = current_token(tokens, index);
            expect(tokens, &mut index, &IDENTIFIER, "identifier")?;

            // Value (either explicit or auto-increment for structs)
            let value = if kind != DefinitionKind::Struct {
                expect(tokens, &mut index, &EQUALS, "\"=\"")?;
                let v_tok = current_token(tokens, index);
                expect(tokens, &mut index, &INTEGER, "integer")?;
                v_tok.text.parse::<i32>().map_err(|_| {
                    error(
                        &format!("Invalid integer {}", quote(&v_tok.text)),
                        v_tok.line,
                        v_tok.column,
                    )
                })?
            } else {
                // For structs, assign in-order values
                fields.len() as i32 + 1
            };

            // Deprecated?
            if eat(tokens, &mut index, &DEPRECATED_TOKEN) {
                if kind != DefinitionKind::Message {
                    let deprecated = current_token(tokens, index - 1);
                    return Err(error(
                        "Cannot deprecate this field",
                        deprecated.line,
                        deprecated.column,
                    ));
                }
                is_deprecated = true;
            }

            expect(tokens, &mut index, &SEMICOLON, "\";\"")?;

            let final_value = if kind != DefinitionKind::Struct {
                value
            } else {
                fields.len() as i32 + 1
            };

            fields.push(Field {
                name: f_tok.text.clone(),
                line: f_tok.line,
                column: f_tok.column,
                type_: type_opt,
                is_array,
                array_size,
                is_deprecated,
                field_id: final_value,
            });
        }

        definitions.push(Definition {
            name: name_tok.text.clone(),
            line: name_tok.line,
            column: name_tok.column,
            kind,
            fields,
        });
    }

    Ok(Schema {
        package: package_text,
        definitions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize_schema;

    #[test]
    fn test_parse_struct_with_fixed_array() {
        let input = r#"
            struct BlockHeader {
                uint64 height;
                byte[32] prevHash;
                byte[32] merkleRoot;
            }
        "#;
        let tokens = tokenize_schema(input).unwrap();
        let schema = parse_schema(&tokens).unwrap();

        assert_eq!(schema.definitions.len(), 1);
        let def = &schema.definitions[0];
        assert_eq!(def.name, "BlockHeader");
        assert_eq!(def.fields.len(), 3);

        // height: uint64 (no array)
        assert_eq!(def.fields[0].name, "height");
        assert_eq!(def.fields[0].type_, Some("uint64".to_string()));
        assert!(!def.fields[0].is_array);
        assert_eq!(def.fields[0].array_size, None);

        // prevHash: byte[32]
        assert_eq!(def.fields[1].name, "prevHash");
        assert_eq!(def.fields[1].type_, Some("byte".to_string()));
        assert!(def.fields[1].is_array);
        assert_eq!(def.fields[1].array_size, Some(32));

        // merkleRoot: byte[32]
        assert_eq!(def.fields[2].name, "merkleRoot");
        assert_eq!(def.fields[2].type_, Some("byte".to_string()));
        assert!(def.fields[2].is_array);
        assert_eq!(def.fields[2].array_size, Some(32));
    }

    #[test]
    fn test_parse_message_with_variable_array() {
        let input = r#"
            message Transaction {
                byte[] data = 1;
                uint64 nonce = 2;
            }
        "#;
        let tokens = tokenize_schema(input).unwrap();
        let schema = parse_schema(&tokens).unwrap();

        assert_eq!(schema.definitions.len(), 1);
        let def = &schema.definitions[0];
        assert_eq!(def.kind, DefinitionKind::Message);

        // data: byte[] (variable array)
        assert_eq!(def.fields[0].name, "data");
        assert!(def.fields[0].is_array);
        assert_eq!(def.fields[0].array_size, None);
    }

    #[test]
    fn test_parse_enum() {
        let input = r#"
            enum NodeRole {
                STORAGE = 1;
                VALIDATOR = 2;
            }
        "#;
        let tokens = tokenize_schema(input).unwrap();
        let schema = parse_schema(&tokens).unwrap();

        assert_eq!(schema.definitions.len(), 1);
        let def = &schema.definitions[0];
        assert_eq!(def.kind, DefinitionKind::Enum);
        assert_eq!(def.fields.len(), 2);
        assert_eq!(def.fields[0].name, "STORAGE");
        assert_eq!(def.fields[0].field_id, 1);
    }
}
