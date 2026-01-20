//! TCS CLI - Command-line interface for Tape Canonical Serialization
//!
//! Commands:
//! - gen-rust: Generate Rust code from a .tcs schema
//! - validate: Validate a .tcs schema
//! - format: Format a .tcs schema (placeholder)

use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use tcs_compiler::{compile, TcsError};

#[derive(Parser)]
#[command(name = "tcs")]
#[command(author, version, about = "TCS (Tape Canonical Serialization) compiler", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate Rust code from a .tcs schema file
    GenRust {
        /// Input .tcs schema file
        #[arg(short, long)]
        input: PathBuf,

        /// Output .rs file (defaults to stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Validate a .tcs schema file
    Validate {
        /// Input .tcs schema file
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Format a .tcs schema file (placeholder - not yet implemented)
    Format {
        /// Input .tcs schema file
        #[arg(short, long)]
        input: PathBuf,

        /// Check only, don't modify the file
        #[arg(long)]
        check: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::GenRust { input, output } => gen_rust(input, output),
        Commands::Validate { input } => validate(input),
        Commands::Format { input, check } => format_schema(input, check),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn gen_rust(input: PathBuf, output: Option<PathBuf>) -> Result<(), TcsError> {
    let source = fs::read_to_string(&input)?;
    let rust_code = compile(&source)?;

    match output {
        Some(path) => {
            fs::write(&path, &rust_code)?;
            eprintln!("Generated: {}", path.display());
        }
        None => {
            println!("{}", rust_code);
        }
    }

    Ok(())
}

fn validate(input: PathBuf) -> Result<(), TcsError> {
    let source = fs::read_to_string(&input)?;

    let tokens = tcs_compiler::tokenize_schema(&source)?;
    let schema = tcs_compiler::parse_schema(&tokens)?;
    tcs_compiler::verify_schema(&schema)?;

    eprintln!("Schema is valid: {}", input.display());
    eprintln!(
        "  {} definition(s)",
        schema.definitions.len()
    );

    for def in &schema.definitions {
        eprintln!(
            "    - {} ({:?}, {} field(s))",
            def.name,
            def.kind,
            def.fields.len()
        );
    }

    Ok(())
}

fn format_schema(input: PathBuf, check: bool) -> Result<(), TcsError> {
    let _ = (input, check);
    eprintln!("Warning: format command is not yet implemented");
    Ok(())
}
