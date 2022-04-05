
use crate::{
    token::{
        Value,
        ast::Ast,
        read::TokenRef,
    },
    parse::lex::TokenType::{self, *},
    error::asm::{
        AsmErr, AstMsg::{self, *}
        ExprErr, ExprMsg::{self, *}
    },
    write::constants::ConstExpr,
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

/// Builds an `Expr` token from a `DefB` or `DefW`.
pub fn build<'a>(ast: &mut Ast<'a>, scope: usize) -> Result<(), AsmErr<'a, AstMsg>> {
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
            let mut is_bin = false;
            let mut is_neg = false;

            if ty.parent_type() == Expr {
                if ty == BinSub && prec == UnNeg {
                    match un_neg(ast, child) {
                        Ok(n) => is_neg = n,
                        Err(e) => return Err(e),
                    }
                }

                if !is_neg && ty == prec {
                    if prec == UnNot {
                        un_not(ast, child)?;
                    }
                    
                    else {
                        bin(ast, child)?;
                        is_bin = true;
                    }
                }
            }

            // if is it was a bin, 2 tokens were removed. Next index is the same index.
            if !is_bin {
                i += 1;
            }

            if i == ast.tokens[scope].children.len() {
                break;
            }
        }
    }

    Ok(())
}

/// Check if a '-' is a unary operator and attempts to move right operand into it.
fn build_un_neg<'a>(ast: &mut Ast<'a>, neg: usize) -> Result<bool, AsmErr<'a, AstMsg>> {
    if let Some(left) = ast.left_of(ast.tokens[neg].index) {
        let left = &ast.tokens[left];
        let is_expr = left.ty.parent_type() == Expr;

        if !is_expr || (is_expr && !left.children.is_empty()) {
            return Ok(false);
        }
    }

    ast.tokens[neg].ty = UnNeg;
    let right = ast.right_of(neg)
        .ok_or(err!(AstMsg, UnaryWithoutRhs, (&ast.tokens[neg]).into()))?;

    ast.move_into(right, neg);

    Ok(true)
}

/// Attempts to move right operand into a unary 'not' operator.
fn build_un_not<'a>(ast: &mut Ast<'a>, not: usize) -> Result<(), AsmErr<'a, AstMsg>> {
    let right = ast.right_of(not)
        .ok_or(err!(AstMsg, UnaryWithoutRhs, (&ast.tokens[not]).into()))?;

    ast.move_into(right, not);

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

/// Evaluate the value for an `Expr` token and its content.
pub fn evaluate(expr: &TokenRef) -> usize {
    assert_eq!(expr.ty(), Expr);

    // - Evaluate op if Lit or Identifier (not LitStr).
    // - Do not allow ident of this expr within the Expr itself.
    // - Check for circular depencies.

    let mut errors = vec![];
    let def_ident = expr.parent().get(0).value().as_str();

    todo!()
}

struct ExprCtx<'a> {
    def_ident: &'a str, 
    def_ty: TokenType,
    constants: &'a mut Constants,
    errors: &'a mut Vec<AsmErr<'a, ExprMsg>>,
}

fn eval_scope(scope: &TokenRef, ctx: &mut ExprCtx) -> Result<isize, ()> {
    match scope.ty() {
        Lit => {
            let litx = scope.get(0);            

            match litx.ty() {
                LitDec|LitBin|LitHex => {
                    return Ok(litx.value().as_usize());
                }

                LitStr => {
                    errors.push(err!(ExprMsg, LitStrInExpr, litx.into()));
                    return Err(());
                }

                _ => bug!("Unhandled Lit type in Expr")
            }
        }

        Identifier => {
            let ident = child.value().ast_str();

            // Does this expression depend on itself? 
            if child.value().as_str() == def_ident {
                errors.push(err!(ExprMsg, CircularDependency, child.into()));
            }

            // Read the value in the `Constants` map.
            let const_expr = constants.get_mut(ident)
                .map_err(|e| errors.push(err!(ExprMsg, ConstantNotFound, child.into())));

            match const_expr {
                Value(value) => match value {
                    Value::Usize(num) => return Ok(num as isize),

                    Value::Str(s) => {
                        // #db X "Hello" is allowed.
                        if scope.ty() == Expr && scope.children().len() == 1 {

                        }

                        // #db X ("Hello") * 10 is not allowed.
                        else {
                            errors.push(err!(ExprMsg, LitStrInExpr, litx.into()));
                            return Err(());
                        }
                    }
                }

                ConstExpr::Expr(expr) => {
                    //TODO
                }

                _ => bug!("Invalid constant")
            }
        }

        _ => {
            for child in scope.children() {
                // A value without an operator parent must be an only-child.
                // e.g.     #db X0 10
                //          #db X1 10 + (5)
                // error:   #db x2 1 2 3
                let not_op = matches!(scope.ty(), At|Expr);
                let is_value = matches!(child.ty(), Lit|Identifier);

                if not_op && is_value {
                    if scope.children().len() == 1 {
                        return eval_scope(child, ctx);
                    }

                    else {
                        ctx.errors.push();
                        return Err(());
                    }
                }

                if child.ty() == At {
                    return eval_scope(child, ctx);
                }

                else if child.ty().parent_type() == Expr {
                    return eval_op(child, ctx);
                }
            }
        }
    }
}

fn eval_bin<'a>(
    f: fn(isize, isize) -> isize,
    op: &TokenRef<'a>, 
    ctx: &mut ExprCtx,
) -> Result<isize, AsmErr<'a, ExprMsg>> {
    let lhs = eval_scope(op.get(0), ctx)?;
    let rhs = eval_scope(op.get(1), ctx)?;

    Ok(f(lhs, rhs))
}

fn eval_op<'a>(op: &TokenRef, ctx: &mut ExprCtx) -> Result<isize, ()> {
    assert_eq!(op.ty().parent_type(), Expr);

    let result = match op.ty() {
        UnNot => Ok(~eval_scope(op.get(0), ctx)?),
        UnNeg => Ok(-eval_scope(op.get(0), ctx)?),
        BinMul => eval_bin(|lhs, rhs| lhs * rhs, op, ctx),
        BinDiv => eval_bin(|lhs, rhs| lhs / rhs, op, ctx),
        BinMod => eval_bin(|lhs, rhs| lhs % rhs, op, ctx),
        BinAdd => eval_bin(|lhs, rhs| lhs + rhs, op, ctx),
        BinSub => eval_bin(|lhs, rhs| lhs - rhs, op, ctx),
        BinShl => eval_bin(|lhs, rhs| lhs << rhs, op, ctx),
        BinShr => eval_bin(|lhs, rhs| lhs >> rhs, op, ctx),
        BinAnd => eval_bin(|lhs, rhs| lhs & rhs, op, ctx),
        BinXor => eval_bin(|lhs, rhs| lhs ^ rhs, op, ctx),
        BinOr => eval_bin(|lhs, rhs| lhs | rhs, op, ctx),
        _ => bug!("Unhandled operator type")
    }?;

    match ctx.def_ty {
        DefB => Ok(result % 256),
        DefW => Ok(result % 65536),
        _ => bug!("Wrong Def type.")
    }
}
