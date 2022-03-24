
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

fn expr<'a>(token: &Token<'a>) -> Token<'a> {
    let Token{ line_number, line, word, value, .. } = *token;

    Token{ 
        ty: Expr, 
        line_number, 
        line, 
        word, 
        value, 
        index: 0, 
        parent: 0, 
        children: vec![] 
    }
}

fn op_token<'a>(ty: TokenType, index: usize, parent: &Token<'a>) -> Token<'a> {
    let Token{ line_number, line, word, value, .. } = *parent;
    let parent = parent.index;

    Token{ ty, line_number, line, word, value, index, parent, children: vec![] }
}

pub fn build<'a>(ast: &'a Ast<'a>, token: &'a Token<'a>) -> &'a Token<'a> {
    let mut tokens = vec![expr(token)];
    let mut selection = vec![0];

    for index in &token.children {
        let child = &ast.tokens[*index];

        match child.ty {
            At0 => {
                let index = tokens.len();
                selection.push(index);
                let at = op_token(At, index, &tokens[*selection.last().unwrap()]);
                tokens.push(at);
            }

            At1 => {
                selection.pop();
            }

            _ => {}
        }
    }

    todo!();
}
