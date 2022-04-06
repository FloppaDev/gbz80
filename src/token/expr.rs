
use crate::{
    token::{
        Value,
        ast::Ast,
        read::TokenRef,
    },
    parse::lex::TokenType::{self, *},
    error::asm::{
        AsmErr, 
        AstMsg::{self, *},
        ExprMsg::{self, *},
    },
    write::constants::{ConstExpr, Constants},
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
                    match build_un_neg(ast, child) {
                        Ok(n) => is_neg = n,
                        Err(e) => return Err(e),
                    }
                }

                if !is_neg && ty == prec {
                    if prec == UnNot {
                        build_un_not(ast, child)?;
                    }
                    
                    else {
                        build_bin(ast, child)?;
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

pub struct ExprCtx<'a> {
    def_ty: TokenType,
    dependencies: Vec<&'a str>,
    constants: &'a mut Constants<'a>,
    errors: Vec<AsmErr<'a, ExprMsg>>,
}

impl<'a> ExprCtx<'a> {

    pub fn new(def_ty: TokenType, constants: &'a mut Constants<'a>) -> Self {
        Self { def_ty, dependencies: vec![], constants, errors: vec![] }
    }

    /// Evaluate the value for an `Expr` token and its content.
    pub fn evaluate(&'a mut self, ident: &'a str) -> Result<usize, ()> {
        let expr = match self.constants.get(ident).unwrap() {
            ConstExpr::Expr(e) => e,
            _ => bug!("The identifier of the expression maps to the wrong constant")
        };

        self.dependencies.push(ident);
        let mut result = self.eval_scope(expr)? as usize;
        self.dependencies.pop();

        match self.def_ty {
            DefB => Ok(result % 256),
            DefW => Ok(result % 65536),
            _ => bug!("Wrong Def type.")
        }
    }

    fn eval_scope(&mut self, scope: &TokenRef<'a>) -> Result<isize, ()> {
        if scope.children().len() != 1 {
            self.errors.push(err!(ExprMsg, TooManyChildren, scope.into()));
            return Err(());
        }

        let child = scope.children()[0];

        match scope.ty() {
            Lit => {
                let litx = scope.get(0);            

                match litx.ty() {
                    LitDec|LitBin|LitHex => {
                        return Ok(litx.value().as_usize() as isize);
                    }

                    LitStr => {
                        self.errors.push(err!(ExprMsg, StrInExpr, litx.into()));
                        return Err(());
                    }

                    _ => bug!("Unhandled Lit type in Expr")
                }
            }

            Identifier => {
                let ident = scope.value().as_str();

                //TODO check deps stack
                //if scope.value().as_str() == self.def_ident {
                    //self.errors.push(err!(ExprMsg, CircularDependency, scope.into()));
                //}

                // Read the value in the `Constants` map.
                let const_expr = self.constants.get_mut(ident)
                    .ok_or(self.errors.push(err!(ExprMsg, ConstantNotFound, scope.into())))?;

                match const_expr {
                    ConstExpr::Value(value) => {
                        match value {
                            Value::Usize(num) => return Ok(*num as isize),

                            Value::Str(_) => {
                                self.errors.push(err!(ExprMsg, StrInExpr, scope.into()));
                                return Err(());
                            }

                            _ => bug!("Unhandled value type")
                        }
                    }

                    ConstExpr::Expr(expr) => {
                        //TODO evaluate()
                        return Err(());//TODO remove
                    }

                    _ => bug!("Invalid constant")
                }
            }

            _ => {
                // A value without an operator parent must be an only-child.
                // e.g.     #db X0 10
                //          #db X1 10 + (5)
                // error:   #db x2 1 2 3
                let not_op = matches!(scope.ty(), At|Expr);
                let is_value = matches!(child.ty(), Lit|Identifier);

                //TODO check bool logic
                if not_op && is_value {
                    return self.eval_scope(child);
                }
                if child.ty() == At {
                    return self.eval_scope(child);
                }
                else if child.ty().parent_type() == Expr {
                    return self.eval_op(child);
                }
            }
        }

        todo!();//TODO remove
    }

    fn eval_bin(
        &mut self,
        f: fn(isize, isize) -> isize,
        op: &TokenRef<'a>, 
    ) -> Result<isize, ()> {
        let lhs = self.eval_scope(op.get(0))?;
        let rhs = self.eval_scope(op.get(1))?;

        Ok(f(lhs, rhs))
    }

    fn eval_op(&mut self, op: &TokenRef<'a>) -> Result<isize, ()> {
        assert_eq!(op.ty().parent_type(), Expr);

        match op.ty() {
            UnNot => Ok(!self.eval_scope(op.get(0))?),
            UnNeg => Ok(-self.eval_scope(op.get(0))?),
            BinMul => self.eval_bin(|lhs, rhs| lhs * rhs, op),
            BinDiv => self.eval_bin(|lhs, rhs| lhs / rhs, op),
            BinMod => self.eval_bin(|lhs, rhs| lhs % rhs, op),
            BinAdd => self.eval_bin(|lhs, rhs| lhs + rhs, op),
            BinSub => self.eval_bin(|lhs, rhs| lhs - rhs, op),
            BinShl => self.eval_bin(|lhs, rhs| lhs << rhs, op),
            BinShr => self.eval_bin(|lhs, rhs| lhs >> rhs, op),
            BinAnd => self.eval_bin(|lhs, rhs| lhs & rhs, op),
            BinXor => self.eval_bin(|lhs, rhs| lhs ^ rhs, op),
            BinOr => self.eval_bin(|lhs, rhs| lhs | rhs, op),
            _ => bug!("Unhandled operator type")
        }
    }

}


