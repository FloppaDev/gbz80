
//! Compilation will follow these steps:
//!
//! - File is split into lines and words.
//!      - Words can be delimited by:
//!          - spaces
//!          - newlines
//!          - expressions characters ') ( + -'
//!          - macro parameters delimiter '.'
//!          - double quotes '"', string literals count as one word
//!      - Line comments are ignored during this step.
//!      - #if directives are also treated then.
//!
//! - From words, token types can be identified, and their values can be parsed and stored.
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
//! - Markers are validated. 
//!      .i.e marker &0104: code before the marker cannot exceed &0104.
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
)]

#![allow(unused_mut, unused_variables, dead_code)] //TODO

//====================================================================
            
#[macro_use]
mod program;

mod parse;

/// Constructs the hierarchy of tokens.
mod token;

mod write;

#[cfg(test)]
mod tests;

//====================================================================

//TODO memory banks.

//TODO math expressions.
//TODO forbid LitStr in arithmetics.

//TODO update crate doc

//====================================================================

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
    program::{
        clargs,
        error::stage::*,
    },
};

use std::{
    fs,
    fmt,
};

fn main() {
    match run() {
        Ok(_) => (),
        Err(_) => ()
    }
}

pub fn run() -> Result<(), ()> {
    // Command line arguments.
    let args = std::env::args().collect::<Vec<_>>();
    let clargs = clargs(&args).map_err(stage_err!(CLARGS))?;

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
            eprintln!("{:?}", err);
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
    let constants = Constants::new(&ast_ref, &op_map);

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
