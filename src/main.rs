
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

//====================================================================

#![deny(
    // Some enum types are commonly used with '*'. it saves on redundancy but typos
    // are interpreted as the catch-all pattern '_ => ...' with a named assignment,
    // which makes the variant and all that follows unhandled.
    unreachable_patterns,
    non_snake_case,
)]

#![warn(
    missing_docs,
    clippy::pedantic,
    clippy::nursery,
)]

#![allow(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::enum_glob_use,
    clippy::cognitive_complexity,
    clippy::unnecessary_wraps,
    clippy::if_not_else,
    clippy::cast_possible_truncation,
    clippy::non_ascii_literal,
    clippy::match_wildcard_for_single_variants,
    clippy::module_name_repetitions,
)]

/// Error types for the different compilation stages
#[macro_use]
mod error;

#[macro_use]
/// Controls the execution of the assembler.
/// It also provides tools for cli arguments and outputs.
mod program;

/// Tools for reading the source file and transforming it into tokens.
mod parse;

/// Constructs the hierarchy of tokens.
mod token;

/// Prepares the final result for the binary output.
mod write;

/// Various tests to reduce the risk of a crash.
#[cfg(target_family="unix")]
#[cfg(test)]
mod tests;

use crate::program::run;
use std::process::exit;

fn main() {
    match run() {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }
}
