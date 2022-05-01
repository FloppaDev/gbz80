
pub mod read;

pub mod ast;

pub mod expr;

pub mod validation;

use crate::{
    parse::{
        lex::TokenType,
        prepare::ParsedToken,
    },
};

#[derive(Debug, Copy, Clone)]
pub enum Value<'a> {
    Void,
    Usize(usize),
    Str(&'a str),
}

impl<'a> Value<'a> {

    /// Returns the contained `usize` value.
    /// Panics:
    /// self is not a `Value::Usize`
    pub fn as_usize(&self) -> usize {
        if let Value::Usize(v) = *self {
            return v;
        }

        panic!("Wrong value type");
    }

    /// Returns the contained `str` value.
    /// Panics:
    /// self is not a `Value::Str`
    pub fn as_str(&self) -> &'a str {
        if let Value::Str(v) = *self {
            return v;
        }

        panic!("Wrong value type");
    }

}

/// Token within the tree.
#[derive(Debug)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub file: &'a str,
    pub line_number: usize,
    pub line: &'a str,
    pub word: &'a str,
    pub value: Value<'a>,
    pub index: usize,
    pub parent: usize,
    pub children: Vec<usize>,
}

impl<'a> Token<'a> {

    /// Create a new `Token` from `ParsedToken`.
    const fn new(
        ParsedToken{ ty, line_number, line, word, value }: ParsedToken<'a>, 
        index: usize, 
        parent: usize,
    ) -> Self {
        let children = vec![];
        Self { ty, line_number, line, word, value, index, parent, children }
    }

}
