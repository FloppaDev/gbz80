
/// Parses command line arguments.
pub mod clargs;

/// Formats text output in the terminal.
#[allow(dead_code)]
pub mod fmt;

use crate::{
    error::stage, 
    parse::{ source::Source, split::Split, prepare },
    token::{ ast::{ macros::Macros, Ast }, read::TokenRef, validation },
    write::{ ops::OpMap, constants::Constants, encode },
};

use std::time;

pub fn run(args: Option<Vec<String>>) -> Result<(), ()> {
    let start = time::Instant::now();

    // Command line arguments.
    let args = args.or_else(|| Some(std::env::args().collect::<Vec<_>>())).unwrap();
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

    // Identify intructions.
    let op_map = OpMap::new(&ast_ref).map_err(stage::ops)?;

    // Find and calculate all constants.
    let mut constants = Constants::new(&ast_ref, &op_map).map_err(stage::constants)?;
    let updates = constants.eval().map_err(stage::expressions)?;
    constants.update(updates);
    #[cfg(debug_assertions)] constants.debug();

    // Write output.
    encode::build(&clargs.output(), &ast_ref, &op_map, &constants).map_err(stage::encode)?;

    // Print success.
    let ms = start.elapsed().as_millis();
    let success = fmt::strip()
        .bold(&format!("\n({ms}ms) "))
        .ok("Compilation successfull!")
        .bold(" ---> ")
        .info(&format!("{}\n", clargs.output()))
        .read();

    println!("{success}");

    Ok(())
}
