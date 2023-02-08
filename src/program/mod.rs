
/// Parses command line arguments.
pub mod clargs;

/// Formats text output in the terminal.
#[allow(dead_code)]
pub mod fmt;

use crate::{
    error::stage, 
    parse::{ source::Source, split::Split, prepare },
};

pub fn run() -> Result<(), ()> {
    // Command line arguments.
    let args = std::env::args().collect::<Vec<_>>();
    let clargs = clargs::parse(&args).map_err(stage::clargs)?;

    // Get source file.
    let source = Source::new(clargs.path).map_err(stage::source)?;

    // Split source files into words.
    let split = Split::new(source.main(), &clargs.symbols).map_err(stage::split)?;
    #[cfg(debug_assertions)] split.debug();

    // Extract type information and data.
    let parsed_tokens = prepare::parse(&split).map_err(stage::parse)?;

    Ok(())
}
