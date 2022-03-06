
/// Macros validation and expansion.
pub mod macros;

pub mod read;

pub mod ast;

use crate::{
    parse::{
        lex::TokenType,
        prepare::ParsedToken,
        data::Key,
    },
};

/// Token within the tree.
#[derive(Debug)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub line_number: usize,
    pub line: &'a str,
    pub word: &'a str,
    pub data_key: Key,
    pub index: usize,
    pub parent: usize,
    pub children: Vec<usize>,
}

impl<'a> Token<'a> {

    /// Create a new `Token` from `ParsedToken`.
    const fn new(
        ParsedToken{ ty, line_number, line, word, data_key }: ParsedToken<'a>, 
        index: usize, 
        parent: usize,
    ) -> Self {
        let children = vec![];
        Self { ty, line_number, line, word, data_key, index, parent, children }
    }

}
