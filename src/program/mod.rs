
/// Parses command line arguments.
pub mod clargs;

/// Formats text output in the terminal.
pub mod fmt;

use crate::{
    parse::{
        split::Split,
        prepare::parse,
    },
    token::{
        ast::{
            Ast, 
            macros::Macros,
        },
        read::TokenRef,
        expr,
    },
    write::{
        ops::OpMap,
        constants::Constants,
    },
    error::stage,
};

use std::{
    fs,
};

pub fn run() -> Result<(), ()> {
    // Command line arguments.
    let args = std::env::args().collect::<Vec<_>>();
    let clargs = clargs::parse(&args).map_err(stage::clargs)?;

    // Get source file.
    let input = fs::read_to_string(clargs.path).map_err(stage::source)?; 

    // Split source file into words.
    let split = Split::new(&input, &clargs.symbols).map_err(stage::split)?;
    #[cfg(debug_assertions)] split.debug();

    // Extract type information and data.
    let parsed_tokens = parse(&split).map_err(stage::parse)?;

    // Build the token tree.
    let mut macros = Macros::new();
    let mut ast = Ast::new(parsed_tokens, &mut macros).map_err(stage::ast)?;
    macros.expand(&mut ast).map_err(stage::macros)?;
    #[cfg(debug_assertions)] ast.debug();

    let ast_ref = TokenRef::new(&ast);
    let op_map = OpMap::new(&ast_ref).map_err(stage::ops)?;

    // Find and calculate all constants.
    let mut constants = Constants::new(&ast_ref, &op_map).map_err(stage::constants)?;
    let updates = constants.eval().unwrap();
    constants.update(updates);
    #[cfg(debug_assertions)] constants.debug();

    // let instructions = opcodes::get_instructions();
    // let int_ast = ast::Token::make_ast(split, &instructions);

    // let err = validation::check(&int_ast.root);
    // if err != 0 {
        // eprintln!(
            // "\x1b[0;31mCompilation failed at syntax validation with {} errors.\x1b[0m",
            // err
        // );
        // std::process::exit(1);
    // }
    
    // let ops_map = encode::instruction_ops(&int_ast.root, &instructions);
    // let markers = encode::get_markers(&int_ast.root, &ops_map);

    //encode::build(int_ast.root, instructions);

    Ok(())
}
