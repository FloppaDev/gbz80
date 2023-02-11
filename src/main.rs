
#![deny(unreachable_patterns, non_snake_case)]

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

fn main() {
    match program::run() {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(1)
    }
}
