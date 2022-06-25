
//TODO make sure that parents of tokens are still correct after eval.

#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]

use crate::{
    token::{
        Value,
        read::TokenRef,
    },
    parse::lex::TokenType::*,
    error::asm::{AsmErr, ExprMsg::{self, *}},
    write::constants::{ConstExpr, Constants},
};

pub struct ExprResult {
    pub updates: Vec<(String, usize)>,
}

impl ExprResult {

    fn new(updates: Vec<(String, usize)>) -> Self {
        Self{ updates }
    }

    pub fn eval<'a>(
        expr: &'a TokenRef<'a>, 
        constants: &'a Constants<'a>
    ) -> Result<Self, Vec<AsmErr<'a, ExprMsg>>> {
        match ExprCtx::new(constants).evaluate(expr) {
            Ok((_, ctx)) => Ok(Self::new(ctx.updates)),
            Err(ctx) => Err(ctx.errors)
        }
    }

}

struct ExprCtx<'a> {
    dependencies: Vec<&'a TokenRef<'a>>,
    constants: &'a Constants<'a>,
    errors: Vec<AsmErr<'a, ExprMsg>>,
    updates: Vec<(String, usize)>,
}

impl<'a> ExprCtx<'a> {

    const fn new(constants: &'a Constants<'a>) -> Self {
        Self{
            dependencies: vec![], 
            constants,
            errors: vec![],
            updates: vec![],
        }
    }

    /// Evaluates the value for an `Expr` token and its content.
    fn evaluate(mut self, expr: &'a TokenRef<'a>) -> Result<(usize, Self), Self> {
        self.dependencies.push(expr);
        let mut result;

        self = match self.eval_scope(expr) {
            Ok((value, mut s)) => {
                if value < 0 {
                    s.errors.push(err!(ExprMsg, NegativeResult, expr.into()));
                    return Err(s);
                }

                result = value as usize;
                s
            }

            Err(s) => return Err(s)
        };

        self.dependencies.pop();

        result = match expr.parent().ty() {
            DefB => result % 256,
            DefW => result % 65536,
            _ => bug!("Wrong Def type.")
        };

        let ident = expr.parent().get(0).value().as_str();
        self.updates.push((ident.to_string(), result));
        
        Ok((result, self))
    }

    fn eval_scope(mut self, scope: &'a TokenRef<'a>) -> Result<(isize, Self), Self> {
        let children = scope.children();

        match scope.ty() {
            Lit => {
                let litx = scope.get(0);            

                match litx.ty() {
                    LitDec|LitBin|LitHex => Ok((litx.value().as_usize() as isize, self)),

                    LitStr => {
                        self.errors.push(err!(ExprMsg, StrInExpr, litx.into()));
                        Err(self)
                    }

                    _ => bug!("Unhandled Lit type in Expr")
                }
            }

            Identifier => {
                let ident = scope.value().as_str();

                // Read the value in the `Constants` map.
                let const_expr = self.constants.get(ident);

                if const_expr.is_none() {
                    self.errors.push(err!(ExprMsg, ConstantNotFound, scope.into()));
                    return Err(self);
                }

                let const_expr = const_expr.unwrap();

                match const_expr {
                    ConstExpr::Value(value) => {
                        match value {
                            Value::Usize(num) => Ok((*num as isize, self)),

                            Value::Str(_) => {
                                self.errors.push(err!(ExprMsg, StrInExpr, scope.into()));
                                Err(self)
                            }

                            _ => bug!("Unhandled value type")
                        }
                    }

                    ConstExpr::Expr(expr) => {
                        for dep in &self.dependencies {
                            if **expr == **dep {
                                self.errors.push(err!(ExprMsg, CircularDependency, scope.into()));
                                return Err(self);
                            }
                        }

                        return match self.evaluate(expr) {
                            Ok((value, s)) => Ok((value as isize, s)),
                            Err(s) => Err(s)
                        };
                    }

                    _ => bug!("Invalid constant")
                }
            }

            _ => {
                if let Some(child) = children.get(0) {
                    // A value without an operator parent must be an only-child.
                    // e.g.     #db X0 10
                    //          #db X1 10 + (5)
                    // error:   #db x2 1 2 3
                    let not_op = matches!(scope.ty(), At|Expr);
                    let is_value = matches!(child.ty(), Lit|Identifier);

                    if (not_op && is_value) || child.ty() == At {
                        return self.eval_scope(child);
                    }

                    else if child.ty().parent_type() == Expr {
                        return self.eval_op(child);
                    }

                    else if scope.ty().parent_type() == Expr {
                        return self.eval_op(scope);
                    }
                }

                bug!("Unexpected token in expression.");
            }
        }
    }

    fn eval_bin(
        self,
        f: fn(isize, isize) -> isize,
        op: &'a TokenRef<'a>, 
    ) -> Result<(isize, Self), Self> {
        match self.eval_scope(op.get(0)) {
            Ok((value, s)) => {
                let lhs = value;

                match s.eval_scope(op.get(1)) {
                    Ok((value, s)) => Ok((f(lhs, value), s)),
                    Err(s) => Err(s)
                }
            }

            Err(s) => Err(s)
        }
    }

    fn eval_op(
        self, 
        op: &'a TokenRef<'a>,
    ) -> Result<(isize, Self), Self> {
        assert_eq!(op.ty().parent_type(), Expr);

        match op.ty() {
            UnNot => {
                match self.eval_scope(op.get(0)) {
                    Ok((value, s)) => {
                        let value = !(value as u16);
                        Ok((value as isize, s))
                    }
                    Err(s) => Err(s)
                }
            }

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
