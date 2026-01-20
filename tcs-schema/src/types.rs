//! Schema AST types for TCS (Tape Canonical Serialization)

/// Represents a complete TCS schema parsed from a .tcs file.
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    /// Optional package name for the generated code module
    pub package: Option<String>,
    /// All type definitions in the schema
    pub definitions: Vec<Definition>,
}

/// The kind of a type definition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefinitionKind {
    /// Enum with named variants and explicit values
    Enum = 0,
    /// Struct with all required fields (no field IDs)
    Struct = 1,
    /// Message with optional fields (uses field IDs)
    Message = 2,
}

/// A field within a definition (enum variant, struct field, or message field)
#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    /// Field name
    pub name: String,
    /// Source line number (1-indexed)
    pub line: usize,
    /// Source column number (1-indexed)
    pub column: usize,
    /// Type name (None for enum variants)
    pub type_: Option<String>,
    /// Whether this is a variable-length array (e.g., `int[]`)
    pub is_array: bool,
    /// Fixed array size for types like `byte[32]` (None for scalars or variable arrays)
    pub array_size: Option<usize>,
    /// Whether this field is marked as deprecated
    pub is_deprecated: bool,
    /// Field index/value (auto-assigned for structs, explicit for enums/messages)
    pub field_id: i32,
}

/// A type definition (enum, struct, or message)
#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    /// Type name
    pub name: String,
    /// Source line number (1-indexed)
    pub line: usize,
    /// Source column number (1-indexed)
    pub column: usize,
    /// Kind of definition
    pub kind: DefinitionKind,
    /// Fields/variants within this definition
    pub fields: Vec<Field>,
}

impl Schema {
    /// Create a new empty schema
    pub fn new() -> Self {
        Schema {
            package: None,
            definitions: Vec::new(),
        }
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

impl Field {
    /// Check if this field is a fixed-size byte array (e.g., `byte[32]`)
    pub fn is_fixed_byte_array(&self) -> bool {
        self.array_size.is_some() && self.type_.as_deref() == Some("byte")
    }
}
