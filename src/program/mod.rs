
/// Provide all error types needed throughout the compilation.
#[macro_use]
pub mod error;

use crate::{
    program::error::{ ClargsErr, ClargsErrType::* },
};

pub const RECURSION_LIMIT: usize = 1000;

fn log_err<E: fmt::Display>(msg: &str, err: E) {
    eprintln!("{}\n{}", msg, err);
}

macro_rules! stage_err {
    ($stage:expr) => {
        |e| log_err(&$stage(), e)
    }
}
