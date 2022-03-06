
use crate::{
    data::{Data, Key},
    lingo::{ Lexicon, TokenType::{ self, * } },
    parse::ParsedToken,
    error::{ ErrCtx, AstErr, AstErrType },
    macros::Macros,
    process::bug,
};

use std::hash::{Hash, Hasher};

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
