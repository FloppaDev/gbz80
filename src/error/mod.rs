
/// Contains errors messages for the main compilation stages.
#[macro_use]
pub mod stage;

/// Defines errors for the preparation stage.
pub mod init;

/// Defines errors for the assembler stages.
#[macro_use]
pub mod asm;

use crate::{
    parse::{
        prepare::ParsedToken,
        lex::TokenType,
    },
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
    ty: TokenType,
    file: &'a str,
    line_number: usize,
    line: &'a str,
    word: &'a str,
}

impl<'a> ErrCtx<'a> {

    pub const fn new(
        ty: TokenType,
        file: &'a str,
        line_number: usize,
        line: &'a str,
        word: &'a str,
    ) -> Self {
        Self{ ty, file, line_number, line, word }
    }

    pub fn word_start(self) -> Option<usize> {
        if self.line.is_empty() || self.word.is_empty() {
            return None;
        }

        let line = self.line.as_ptr() as usize;
        let word = self.word.as_ptr() as usize;

        if line > word {
            return None;
        }

        let start_byte = word - line;
        let mut bytes = 0;

        for (i, ch) in self.line.chars().enumerate() {
            bytes += ch.len_utf8();

            if bytes >= start_byte {
                return Some(i);
            }
        }

        None
    }

}

impl<'a> From<&ParsedToken<'a>> for ErrCtx<'a> {

    fn from(token: &ParsedToken<'a>) -> Self {
        let ParsedToken{ ty, file, line_number, line, word, .. } = *token;
        Self { ty, file, line_number, line, word }
    }

}

impl<'a> From<&Token<'a>> for ErrCtx<'a> {

    fn from(token: &Token<'a>) -> Self {
        let Token{ ty, file, line_number, line, word, .. } = *token;
        Self { ty, file, line_number, line, word }
    }

}

impl<'a> From<&TokenRef<'a>> for ErrCtx<'a> {

    fn from(token_ref: &TokenRef<'a>) -> Self {
        let Token{ ty, file, line_number, line, word, .. } = *token_ref.token();
        Self { ty, file, line_number, line, word }
    }

}

/// Generates an `unreachable` macro call with the source context included.
/// e.g.    bug!("Oopsie! Assembler no worky...")
macro_rules! bug {
    ($s:literal) => {
        {
            println!("Internal error at '{}':\n{}\nThis is a bug.", source!(), $s);
            std::process::exit(1);
        }
    }
}
