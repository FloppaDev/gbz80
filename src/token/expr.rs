
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

struct Cursor {
    /// Index of a token.
    selection: usize,

    /// Copy of the children of the selection.
    children: Vec<usize>,

    /// Current iteration index within `children`.
    i: usize,

    /// Last iteration index within `children`.
    last: usize,
}

impl Cursor {

    fn new(selection: usize, children: Vec<usize>) -> Self {
        let last = children.len() - 1;
        Self{ selection, children, i: 0, last }
    }

}

pub fn build<'a>(ast: &mut Ast<'a>, expr_index: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    for prec in PRECEDENCE {
        let mut stack = vec![
            Cursor::new(expr_index, ast.tokens[expr_index].children.clone())];

        loop {
            let last = stack.len() - 1;
            let mut cursor = &mut stack[last];

            // This is the last child in selection.
            if cursor.i > cursor.last {
                // End of the stack, break the loop.
                if stack.len() == 1 {
                    break;
                }

                // Pop top of the stack
                else {
                    stack.pop();
                    continue;
                }
            }

            let child = cursor.children[cursor.i];
            cursor.i += 1;

            // BinSub needs to be converted if it was used as unary.
            if ast.tokens[child].ty == BinSub && prec == UnNeg {
                let left = ast.left_of(ast.tokens[child].index);

                if left.is_none() || ast.tokens[left.unwrap()].ty.parent_type() == Expr {
                    ast.tokens[child].ty = UnNeg;
                    let right = ast.right_of(child).ok_or(err!(
                        AstMsg, UnaryWithoutRhs, ast.tokens.get(child).unwrap().into()))?;
                    ast.move_into(right, child);
                }
            }

            // Is this is the operator we are currently looking for?
            else if ast.tokens[child].ty == prec {
                // Is it a a UnNot?
                if prec == UnNot {
                    let right = ast.right_of(child).ok_or(err!(
                        AstMsg, UnaryWithoutRhs, ast.tokens.get(child).unwrap().into()))?;
                    ast.move_into(right, child);
                }
                
                // It is a binary operator.
                else {
                    let left = ast.left_of(child).ok_or(err!(
                        AstMsg, BinaryWithoutLhs, ast.tokens.get(child).unwrap().into()))?;
                    let right = ast.right_of(child).ok_or(err!(
                        AstMsg, BinaryWithoutRhs, ast.tokens.get(child).unwrap().into()))?;
                    //TODO ast.move_into(vec![left, right] ...
                    ast.move_into(left, child);
                    ast.move_into(right, child);

                    println!(
                        "l{}: {:?} {:?} {:?}", 
                        ast.tokens[child].line_number, 
                        ast.tokens[left].ty, 
                        ast.tokens[child].ty, 
                        ast.tokens[right].ty);
                }
            }

            // Enter parens.
            else if ast.tokens[child].ty == At {
                let at = ast.tokens[child].index;
                stack.push(Cursor::new(at, ast.tokens[at].children.clone()));

                continue;
            }
        }
    }

    Ok(())
}
