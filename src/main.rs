
//! Compilation will follow these steps:
//!
//! - Source file is split into lines and words.
//!      - Words can be delimited by:
//!          - spaces
//!          - newlines
//!          - expressions characters ') ( + -'
//!          - macro parameters delimiter '.'
//!          - double quotes '"', string literals count as one word
//!      - Line comments are removed during this step.
//!      - #if directives are also treated then.
//!
//! - From words, token types are identified, and their values are parsed and stored.
//!      - e.g.  &0143 becomes a `LitHex` with a value of 0x0143
//!              A literal wich is an hexadecimal number
//!
//! - Tokens are then assembled into a tree structure based on their types.
//! 
//! - Macros are expanded.
//!
//! - The tree is checked for syntax errors.
//!
//! - Constants are calculated (defines and marker positions).
//!
//! - Nintendo logo is checked and checksums are calculated.
//!
//! - Result is encoded into the output.

/// Error types for the different compilation stages
#[macro_use]
mod error;

/// Controls the execution of the assembler.
/// It also provides tools for cli arguments and outputs.
#[macro_use]
mod program;

/// Tools for reading the source file and transforming it into tokens.
mod parse;

/// Constructs the hierarchy of tokens.
mod token;

/// Prepares the final result for the binary output.
mod write;

#[cfg(target_family="unix")]
#[cfg(test)]
mod tests;

use program::run;
use std::process::exit;

fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }
}
