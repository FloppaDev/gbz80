
use crate::program::fmt;

/// Prints an error from the assembler.
pub fn log_err<E: std::fmt::Display>(msg: &str, err: E) {
    eprintln!("{}\n{}", msg, err);
}

/// Prints the error message for a specific stage.
/// Arguments:
/// - the stage constant
macro_rules! stage_err {
    ($stage:expr) => {
        |e| crate::error::stage::log_err(&$stage(), e)
    }
}

pub const CLARGS: fn () -> String = | | fmt::strip()
    .err("Compilation Failed. ")
    .info("Invalid command line arguments.")
    .read();

pub const SPLIT: fn () -> String = | | fmt::strip()
    .err("Compilation Failed. ")
    .info("Could not recognize words in source file.")
    .read();
