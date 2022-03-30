
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
    let mut bin = false;
    let mut un = false;

    for prec in PRECEDENCE {
        let sel = *selection.last().unwrap();
        let children = ast.tokens[sel].children.clone();

        for (i, child) in children.iter().enumerate() {
            let token = &ast.tokens[*child];

            // A binary operator is waiting for its right operand.
            if bin {
                // Add right
                bin = false;
            }

            // A unary operator is waiting for its operand.
            else if un {
                // Add right
                un = false;
            }

            else if token.ty == BinSub && prec == UnNeg {
                let left = ast.left_of(token.index);

                if left.is_none() || ast.tokens[left.unwrap()].ty.parent_type() == Expr {
                    // This is a UnNeg. 
                    // Change ty
                    ast.tokens[sel].ty = UnNeg;
                    un = true;
                }
            }

            else if token.ty == prec {
                if prec == UnNot {
                    // This is a UnNot. 
                    un = true;
                }

                else if let Some(left) = ast.left_of(token.index) {
                    // Add left
                    bin = true; 
                }
            }

            // Enter parens.
            else if token.ty == At {
                selection.push(token.index);
            }
                
            // Last child inside selection.
            if i == children.len() - 1 {
                selection.pop();
            }
        }
    }
}
