
pub mod eval;

use crate::{
    token::{ast::Ast},
    parse::lex::TokenType::{self, *},
    error::asm::{AsmErr, AstMsg::{self, *}},
};

use Arity::*;

enum Arity {
    Unary,
    Binary,
}

struct Prec {
    arity: Arity,
    operators: &'static [TokenType],
}

impl Prec {

    const fn new(arity: Arity, operators: &'static [TokenType]) -> Self {
        Self{ arity, operators }
    }

}

/// Precedence from strongest to weakest.
const PRECEDENCE: &'static [Prec] = &[
    Prec::new(Unary, &[UnNot]),
    Prec::new(Binary, &[BinMul, BinDiv, BinMod]),
    Prec::new(Binary, &[BinAdd, BinSub]),
    Prec::new(Binary, &[BinShl, BinShr]),
    Prec::new(Binary, &[BinAnd, BinXor, BinOr]),
];

/// Builds an `Expr` token from a `DefB` or `DefW`.
pub fn build<'a>(ast: &mut Ast<'a>, scope: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    if cfg!(test) {
        return Ok(());
    }

    // Iterate recursively through parens.
    for i in 0..ast.tokens[scope].children.len() {
        let child = ast.tokens[scope].children[i];
        let ty = ast.tokens[child].ty; 

        if ty == At {
            build(ast, child)?;
        }
    }

    // Build the tree of operator, following the precedence order.
    for prec in PRECEDENCE {
        let mut i = 0;

        loop {
            let child = ast.tokens[scope].children[i];
            let ty = ast.tokens[child].ty; 
            let mut binary = false;

            if ty.parent_type() == Expr {
                for op in prec.operators {
                    if *op == ty {
                        match prec.arity {
                            Binary => {
                                binary = true;
                                build_bin(ast, child)?;
                            }

                            Unary => {
                                build_un(ast, child)?;
                            }
                        }
                    }
                }
            }

            // if binary, 2 tokens were removed. Next index is the same index.
            if !binary {
                i += 1;
            }

            if i == ast.tokens[scope].children.len() {
                break;
            }
        }
    }

    Ok(())
}

/// Attempts to move right operand into a unary 'not' operator.
fn build_un<'a>(ast: &mut Ast<'a>, un: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    let right = ast.right_of(un)
        .ok_or(err!(AstMsg, UnaryWithoutRhs, (&ast.tokens[un]).into()))?;

    ast.move_into(right, un);

    Ok(())
}

/// Attempts to move left and right operands into a binary operator.
fn build_bin<'a>(ast: &mut Ast<'a>, bin: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    let err_ctx = (&ast.tokens[bin]).into();

    let left = ast.left_of(bin).ok_or(err!(AstMsg, BinaryWithoutLhs, err_ctx))?;
    let right = ast.right_of(bin).ok_or(err!(AstMsg, BinaryWithoutRhs, err_ctx))?;

    ast.move_into(left, bin);
    ast.move_into(right, bin);

    Ok(())
}
