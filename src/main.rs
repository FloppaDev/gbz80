
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
                
/// Store the data parsed from the source file.
mod data;

/// Provide all error types needed throughout the compilation.
mod error;

/// Splits the input file into words.
mod split;

/// Wrapper over token types and helpers to identify them.
mod lingo;

/// Converts words into token types and data.
mod parse;

/// Constructs the hierarchy of tokens.
mod token;

/// Macros validation and expansion.
mod macros;

/// Provides tools to work with strings and check for valid characters.
mod text;

/// Helper functions for the program.
mod process;

//TODO doc
mod instructions;

/// Writes values from defines and markers.
mod constants;

/// Tests for all the main components of the project.
#[cfg(test)]
mod tests;

//====================================================================

//TODO forbid LitStr in arithmetics.

//TODO update crate doc

//TODO parse 'Repeat's before 'Lit's and includes.

//====================================================================

use crate::{
    lingo::Lexicon,
    data::Data,
    split::Split,
    parse::parse,
    token::{Ast, TokenRef},
    process::clargs,
    macros::Macros,
    constants::Constants,
};

use std::fs;

fn main() -> Result<(), ()> {
    // Command line arguments.
    let args = std::env::args().collect::<Vec<_>>();
    let clargs = clargs(&args);

    if let Err(err) = clargs {
        eprintln!("Could not parse arguments");
        eprintln!("{:?}", err);

        return Err(())
    }

    let clargs = clargs.unwrap();

    // Get source file.
    let input = fs::read_to_string(clargs.path); 

    if let Err(err) = input {
        eprintln!("Failed compilation trying to read source file.");
        eprintln!("{:?}", err);

        return Err(())
    }

    let input = input.unwrap();
    let lexicon = Lexicon::new();
    let mut data = Data::new();

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
    let parsed_tokens = parse(&lexicon, &mut data, &split);

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
    let ast = Ast::new(&lexicon, parsed_tokens.unwrap(), &mut macros);

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
    if let Err(errors) = macros.expand(&mut ast, &lexicon, &data) {
        eprintln!(
            "Failed compilation with {} errors at stage 'macros expansion'", 
            errors.len());

        for err in errors {
            eprintln!("{:?}", err);
        }

        return Err(())
    }

    #[cfg(debug_assertions)] ast.debug();

    let ast_ref = TokenRef::new(&data, &ast);
    let constants = Constants::new(&ast_ref);

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
