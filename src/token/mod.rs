
/// Build the token hierarchy.
pub mod ast;

/// Evaluate constant expressions.
pub mod expr;

/// Tools to make reading the token tree easier.
pub mod read;

/// Ensures that tokens respect their intended use.
pub mod validation;

use crate::{
    parse::{ lex::TokenType, prepare::ParsedToken },
};

#[derive(Debug, Copy, Clone)]
pub enum Value<'a> {
    Void,
    U8(u8),
    U16(u16),
    Str(&'a str),
}

impl<'a> Value<'a> {

    /// Returns the contained `u8` value.
    pub fn as_u8(&self) -> Result<u8, ()> {
        return if let Value::U8(v) = *self { Ok(v) }else{ Err(()) };

    }

    /// Returns the contained `u16` value.
    pub fn as_u16(&self) -> Result<u16, ()> {
        return if let Value::U16(v) = *self { Ok(v) }else{ Err(()) };
    }

    /// Returns the contained `str` value.
    pub fn as_str(&self) -> Result<&'a str, ()> {
        return if let Value::Str(v) = *self { Ok(v) }else{ Err(()) };
    }

    /*
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::U8(v) => vec![v],
            Self::U16(v) => encode::u16_to_bytes(v).unwrap(),
            Self::Str(v) => encode::str_to_bytes(v).unwrap(),
            _ => bug!("Wrong value type")
        }
    }
    */

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
        ParsedToken{ ty, file, line_number, line, word, value }: ParsedToken<'a>, 
        index: usize, 
        parent: usize,
    ) -> Self {
        Self { ty, file, line_number, line, word, value, index, parent, children: vec![] }
    }

}
