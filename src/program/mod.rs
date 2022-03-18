
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
        ast::Ast, 
        read::TokenRef,
        macros::Macros,
    },
    write::{
        ops::OpMap,
        constants::Constants,
    },
    error::stage::{
        CLARGS, SPLIT,
    },
};

use std::{
    fs,
};

pub fn run() -> Result<(), ()> {
    // Command line arguments.
    let args = std::env::args().collect::<Vec<_>>();
    let clargs = clargs::parse(&args).map_err(stage_err!(CLARGS))?;

    // Get source file.
    let input = fs::read_to_string(clargs.path); 

    if let Err(err) = input {
        eprintln!("Failed compilation trying to read source file.");
        eprintln!("{:?}", err);

        return Err(())
    }

    let input = input.unwrap();

    // Split source file into words.
    let split = Split::new(&input, &clargs.symbols);

    if let Err(errors) = split {
        eprintln!("Failed compilation with {} errors at stage 'split'", errors.len());

        for err in errors {
            eprintln!("{:?}", err);
        }

        return Err(())
    }

    let split = split.unwrap();
    #[cfg(debug_assertions)] split.debug();

    // Extract type information and data.
    let parsed_tokens = parse(&split);

    if let Err(errors) = parsed_tokens {
        eprintln!(
            "Failed compilation with {} errors at stage 'parse'", 
            errors.len());

        for err in errors {
            eprintln!("{}", err);
        }

        return Err(())
    }

    // Build the token tree.
    let mut macros = Macros::new();
    let ast = Ast::new(parsed_tokens.unwrap(), &mut macros);

    if let Err(errors) = ast {
        eprintln!(
            "Failed compilation with {} errors at stage 'ast build'", 
            errors.len());

        for err in errors {
            eprintln!("{:?}", err);
        }

        return Err(())
    }

    let mut ast = ast.unwrap();
    #[cfg(debug_assertions)] ast.debug();

    // Expand macro calls.
    if let Err(errors) = macros.expand(&mut ast) {
        eprintln!(
            "Failed compilation with {} errors at stage 'macros expansion'", 
            errors.len());

        for err in errors {
            eprintln!("{:?}", err);
        }

        return Err(())
    }

    #[cfg(debug_assertions)] ast.debug();

    let ast_ref = TokenRef::new(&ast);
    let op_map = OpMap::new(&ast_ref);

    if let Err(errors) = op_map {
        eprintln!(
            "Failed compilation with {} errors at stage 'ops map build'", 
            errors.len());

        for err in errors {
            eprintln!("{:?}", err);
        }

        return Err(())
    }

    let op_map = op_map.unwrap();
    let _constants = Constants::new(&ast_ref, &op_map);

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
