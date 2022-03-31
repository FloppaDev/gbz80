
#![allow(unused_mut, unused_variables, dead_code)] //TODO

use crate::{
    token::ast::Ast,
    token::Token,
    parse::lex::TokenType::{self, *},
    error::asm::{AsmErr, AstMsg::{self, *}},
};

/// Precedence from strongest to weakest.
/// NOT -x
/// * / MOD
/// + -
/// SHL SHR
/// AND XOR OR
const PRECEDENCE: [TokenType; 12] = [
    UnNot, UnNeg,
    BinMul, BinDiv, BinMod,
    BinAdd, BinSub,
    BinShl, BinShr,
    BinAnd, BinXor, BinOr
];

pub fn build<'a>(ast: &mut Ast<'a>, scope: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    let children = ast.tokens[scope].children.clone();

    for prec in PRECEDENCE {
        for child in &children {
            let ty = ast.tokens[*child].ty; 

            if ty.parent_type() == Expr {
                if ty == BinSub && prec == UnNeg {
                    un_neg(ast, *child)?;
                }

                else if ty == prec {
                    if prec == UnNot {
                        un_not(ast, *child)?;
                    }
                    
                    else {
                        bin(ast, *child)?;
                    }
                }
            }

            else if ty == At {
                build(ast, *child)?;
            }
        }
    }

    Ok(())
}

fn un_neg<'a>(ast: &mut Ast<'a>, neg: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    if let Some(left) = ast.left_of(ast.tokens[neg].index) {
        let is_expr = ast.tokens[left].ty.parent_type() == Expr;
        let is_empty = ast.tokens[left].children.is_empty();

        if !is_expr || (is_expr && !is_empty) {
            return Ok(());
        }
    }

    ast.tokens[neg].ty = UnNeg;
    let right = ast.right_of(neg).ok_or(err!(
        AstMsg, UnaryWithoutRhs, ast.tokens.get(neg).unwrap().into()))?;

    ast.move_into(right, neg);

    Ok(())
}

fn un_not<'a>(ast: &mut Ast<'a>, not: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    let right = ast.right_of(not).ok_or(err!(
        AstMsg, UnaryWithoutRhs, ast.tokens.get(not).unwrap().into()))?;

    ast.move_into(right, not);

    Ok(())
}

fn bin<'a>(ast: &mut Ast<'a>, bin: usize) -> Result<(), AsmErr<'a, AstMsg>> {

    let left = ast.left_of(bin).ok_or(err!(
        AstMsg, BinaryWithoutLhs, ast.tokens.get(bin).unwrap().into()))?;
    let right = ast.right_of(bin).ok_or(err!(
        AstMsg, BinaryWithoutRhs, ast.tokens.get(bin).unwrap().into()))?;

    //TODO ast.move_into(vec![left, right] ...
    ast.move_into(left, bin);
    ast.move_into(right, bin);

    Ok(())
}
