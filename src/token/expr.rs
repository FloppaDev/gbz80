
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

pub fn build(ast: &mut Ast, expr_index: usize) {
    let mut selection = vec![expr_index];
    let mut unary = false;

    for prec in PRECEDENCE {
        let sel = &ast.tokens[*selection.last().unwrap()];
        let children = &sel.children;

        for (i, child) in children.iter().enumerate() {
            let token = &ast.tokens[*child];

            if token.ty == prec {
                // '-' is used for BinSub and UnNeg.
                if token.ty.paren_type() == Expr {
                    // It is unary if there is no token on the left
                    // or selection is a sub-type of Expr.
                    if i == 0 || sel.ty.parent_type() == Expr {
                        match token.ty {
                            BinSub => {}

                            UnNot => {}

                            _ => {}
                        }
                    }

                    else {
                        selection.push(token.index);
                        //TODO re-parent left
                    }
                }
            }

            else if token.ty == At {
                if i == children.len() - 1 {
                    selection.pop();
                }

                else {
                    selection.push(token.index);
                }
            }
        }
    }

    todo!();
}
