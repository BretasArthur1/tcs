//! Utility functions for TCS compiler

use crate::error::TcsError;

/// Quote a string for error messages (escapes special characters)
pub fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Create a parse error
pub fn error(msg: &str, line: usize, column: usize) -> TcsError {
    TcsError::ParseError {
        msg: msg.to_string(),
        line,
        column,
    }
}

/// Converts a string to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    if s.contains('_') {
        s.split('_')
            .filter(|word| !word.is_empty())
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().to_string() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<String>()
    } else if s == s.to_uppercase() && !s.is_empty() {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().to_string() + &chars.as_str().to_lowercase(),
        }
    } else {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().to_string() + chars.as_str(),
        }
    }
}

/// Converts a string to snake_case
pub fn to_snake_case(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut snake = String::new();
    for i in 0..chars.len() {
        let c = chars[i];
        if c.is_uppercase() {
            if i > 0 {
                let prev = chars[i - 1];
                if !prev.is_uppercase() || (i + 1 < chars.len() && chars[i + 1].is_lowercase()) {
                    snake.push('_');
                }
            }
            snake.push(c.to_lowercase().next().unwrap());
        } else {
            snake.push(c);
        }
    }
    snake
}

/// Escape Rust keywords by appending an underscore
pub fn escape_rust_keyword(s: &str) -> String {
    const KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe",
        "use", "where", "while", "async", "await", "dyn",
    ];
    if KEYWORDS.contains(&s) {
        format!("{}_", s)
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("HELLO"), "Hello");
        assert_eq!(to_pascal_case("hello"), "Hello");
        assert_eq!(to_pascal_case("clientID"), "ClientID");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("clientID"), "client_id");
        assert_eq!(to_snake_case("HTTPServer"), "http_server");
    }

    #[test]
    fn test_escape_rust_keyword() {
        assert_eq!(escape_rust_keyword("type"), "type_");
        assert_eq!(escape_rust_keyword("name"), "name");
        assert_eq!(escape_rust_keyword("async"), "async_");
    }
}
