
/// Parses command line arguments.
pub mod clargs;

/// Formats text output in the terminal.
#[allow(dead_code)]
pub mod fmt;

use crate::{
    error::stage, 
    parse::{ source::Source, split::Split, prepare },
    token::{ ast::{ macros::Macros, Ast }, read::TokenRef, validation },
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

    // Build the token tree.
    let mut macros = Macros::new();
    let mut ast = Ast::new(parsed_tokens, &mut macros, &source).map_err(stage::ast)?;
    macros.expand(&mut ast).map_err(stage::macros)?;
    #[cfg(debug_assertions)] ast.debug();

    // The token tree is now read-only and easier to traverse.
    let ast_ref = TokenRef::new(&ast);

    // Validate the `Ast`.
    validation::run(&ast_ref).map_err(stage::validation)?;

    Ok(())
}
