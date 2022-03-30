
#![allow(unused_mut, unused_variables, dead_code)] //TODO

use crate::{
    token::ast::Ast,
    token::Token,
    parse::lex::TokenType::{self, *},
    error::asm::{AsmErr, AstMsg::{self, *}},
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

pub fn build<'a>(ast: &mut Ast<'a>, expr_index: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    let mut bin = false;
    let mut un = false;

    for prec in PRECEDENCE {
        let mut selection = vec![expr_index];
        let mut sel = selection[selection.len()-1];

        for (i, child) in ast.tokens[sel].children.clone().iter().enumerate() {

            // A binary operator is waiting for its right operand.
            if bin {
                let op = ast.left_of(ast.tokens[*child].index).unwrap();
                let left = ast.left_of(op)
                    .ok_or(err!(
                        AstMsg, BinaryWithoutLhs, ast.tokens.get(*child).unwrap().into()))?;

                ast.move_into(left, op);
                ast.move_into(ast.tokens[*child].index, op);

                bin = false;
            }

            // A unary operator is waiting for its operand.
            else if un {
                let op = ast.left_of(ast.tokens[*child].index).unwrap();
                ast.move_into(ast.tokens[*child].index, op);
                un = false;
            }

            // BinSub needs to be converted if it was used as unary.
            else if ast.tokens[*child].ty == BinSub && prec == UnNeg {
                let left = ast.left_of(ast.tokens[*child].index);

                if left.is_none() || ast.tokens[left.unwrap()].ty.parent_type() == Expr {
                    ast.tokens[sel].ty = UnNeg;
                    un = true;
                }
            }

            // This is the operator we are currently looking for.
            else if ast.tokens[*child].ty == prec {
                if prec == UnNot {
                    un = true;
                }
                
                else {
                    bin = true; 
                }
            }

            // Enter parens.
            else if ast.tokens[*child].ty == At {
                selection.push(ast.tokens[*child].index);
                sel = selection[selection.len()-1];
            }
                
            // Last child inside selection.
            if i == ast.tokens[sel].children.len() - 1 {
                if un {
                    return Err(err!(
                        AstMsg, UnaryWithoutRhs, ast.tokens.get(*child).unwrap().into()));
                }

                if bin {
                    return Err(err!(
                        AstMsg, BinaryWithoutRhs, ast.tokens.get(*child).unwrap().into()));
                }

                if selection.len() > 1 {
                    selection.pop();
                    sel = selection[selection.len()-1];
                }
            }
        }
    }

    Ok(())
}
