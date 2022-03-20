
#![allow(unused_mut, unused_variables, dead_code)] //TODO

use crate::{
    token::ast::Ast,
    token::Token,
    parse::lex::TokenType::{self, *},
};

//TODO << >> for shifts, for now a single char is more convenient.

/// Precedence from strongest to weakest.
/// unary ! -
/// * / %
/// + -
/// < >
/// & ^ |
const PRECEDENCE: [TokenType; 12] = [
    UnNot, UnNeg,
    BinMul, BinDiv, BinMod,
    BinAdd, BinSub,
    BinShl, BinShr,
    BinAnd, BinXor, BinOr
];

pub fn build<'a>(ast: &'a Ast<'a>, token: &'a Token<'a>) -> &'a Token<'a> {
    // - get scopes from parens.
    // - iter all for each operator in precedence order

    for index in &token.children {
        let child = &ast.tokens[*index];

    }

    todo!();
}
