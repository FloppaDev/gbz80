
/// Contains errors messages for the main compilation stages.
#[macro_use]
pub mod stage;

pub mod init;

#[macro_use]
pub mod asm;

use crate::{
    parse::prepare::ParsedToken,
    token::{
        Token,
        read::TokenRef,
    }
};

/// Used mostly in recursion fail-safes.
pub const ITERATION_LIMIT: usize = 1000;

/// Creates a `SourceCtx` containing its location in the source code.
/// No arguments.
macro_rules! source {
    () => {
        crate::error::SourceCtx{ 
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }
}

/// Stores a location in the source code.
#[derive(Debug)]
pub struct SourceCtx {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for SourceCtx {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self{ file, line, column } = self;
        write!(f, "{}", format!("{}:{}:{}", file, line, column))
    }

}

/// Provides context for an error in the parsed source file.
#[derive(Debug, Copy, Clone)]
pub struct ErrCtx<'a> {
    line_number: usize,
    line: &'a str,
    word: &'a str,
}

impl<'a> ErrCtx<'a> {

    pub const fn new(
        line_number: usize,
        line: &'a str,
        word: &'a str,
    ) -> Self {
        Self{ line_number, line, word }
    }

}

impl<'a> From<&ParsedToken<'a>> for ErrCtx<'a> {

    fn from(token: &ParsedToken<'a>) -> Self {
        let ParsedToken{ line_number, line, word, .. } = *token;
        Self { line_number, line, word }
    }

}

impl<'a> From<&Token<'a>> for ErrCtx<'a> {

    fn from(token: &Token<'a>) -> Self {
        let Token{ line_number, line, word, .. } = *token;
        Self { line_number, line, word }
    }

}

impl<'a> From<&TokenRef<'a>> for ErrCtx<'a> {

    fn from(token_ref: &TokenRef<'a>) -> Self {
        let Token{ line_number, line, word, .. } = *token_ref.token();
        Self { line_number, line, word }
    }

}
