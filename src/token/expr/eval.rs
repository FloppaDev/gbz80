
use crate::{
    token::{
        Value,
        read::TokenRef,
    },
    parse::lex::TokenType::{self, *},
    error::asm::{AsmErr, ExprMsg::{self, *}},
    write::constants::{ConstExpr, Constants},
};

pub struct ExprCtx<'a> {
    def_ty: TokenType,
    dependencies: Vec<&'a TokenRef<'a>>,
    constants: &'a mut Constants<'a>,
    errors: Vec<AsmErr<'a, ExprMsg>>,
}

impl<'a> ExprCtx<'a> {

    pub fn new(def_ty: TokenType, constants: &'a mut Constants<'a>) -> Self {
        Self { def_ty, dependencies: vec![], constants, errors: vec![] }
    }

    /// Evaluate the value for an `Expr` token and its content.
    pub fn evaluate(&'a mut self, expr: &'a TokenRef<'a>) -> Result<usize, ()> {
        self.dependencies.push(expr);
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
