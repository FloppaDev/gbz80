
use crate::{
    token::{
        Value,
        read::TokenRef,
    },
    parse::lex::TokenType::{self, *},
    error::asm::{AsmErr, ExprMsg::{self, *}},
    write::constants::{ConstExpr, Constants},
};

pub fn run<'a>(
    mut constants: Constants<'a>
) -> Result<Constants<'a>, Vec<AsmErr<'a, ExprMsg>>> {
    let exprv = constants.constants.values()
        .filter(|v| matches!(v, ConstExpr::Expr(_)))
        .map(|v| match v { ConstExpr::Expr(e) => *e, _ => unreachable!() })
        .collect::<Vec<_>>();

    let mut results = vec![];
    let mut errors = vec![];

    for expr in exprv {
        match ExprResult::eval(expr, &constants) {
            Ok(result) => results.push(result),
            Err(mut e) => errors.append(&mut e)
        }
    }

    for result in results {
        for (ident, v) in result.updates {
            let value = ConstExpr::Value(Value::Usize(v));
            *constants.get_mut(&ident).unwrap() = value;
        }
    }

    todo!();
}

pub struct ExprResult {
    updates: Vec<(String, usize)>,
    value: usize,//TODO remove
}

impl ExprResult {

    fn new(updates: Vec<(String, usize)>, value: usize) -> Self {
        Self{ updates, value }
    }

    pub fn eval<'a>(
        expr: &'a TokenRef<'a>, 
        constants: &'a Constants<'a>
    ) -> Result<Self, Vec<AsmErr<'a, ExprMsg>>> {
        match ExprCtx::new(constants).evaluate(expr) {
            Ok((v, ctx)) => Ok(Self::new(ctx.updates, v)),
            Err(ctx) => Err(ctx.errors)
        }
    }

    pub fn apply<'a>(&self, constants: &mut Constants<'a>) {
        for (ident, value) in &self.updates {
            let const_expr = constants.get_mut(ident).unwrap(); 
            *const_expr = ConstExpr::Value(Value::Usize(self.value));
        }
    }

    pub fn read(self) -> usize {
        self.value
    }

}

struct ExprCtx<'a> {
    dependencies: Vec<&'a TokenRef<'a>>,
    constants: &'a Constants<'a>,
    errors: Vec<AsmErr<'a, ExprMsg>>,
    updates: Vec<(String, usize)>,
}

impl<'a> ExprCtx<'a> {

    fn new(constants: &'a Constants<'a>) -> Self {
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
        let mut result = 0;

        self = match self.eval_scope(expr) {
            Ok((value, s)) => {
                result = value as usize;
                s
            }

            Err(s) => s
        };

        self.dependencies.pop();

        result = match expr.parent().get(0).ty() {
            DefB => result % 256,
            DefW => result % 65536,
            _ => bug!("Wrong Def type.")
        };
        
        Ok((result, self))
    }

    fn eval_scope(mut self, scope: &'a TokenRef<'a>) -> Result<(isize, Self), Self> {
        if scope.children().len() != 1 {
            self.errors.push(err!(ExprMsg, TooManyChildren, scope.into()));
            return Err(self);
        }

        let child = scope.children()[0];

        match scope.ty() {
            Lit => {
                let litx = scope.get(0);            

                match litx.ty() {
                    LitDec|LitBin|LitHex => {
                        return Ok((litx.value().as_usize() as isize, self));
                    }

                    LitStr => {
                        self.errors.push(err!(ExprMsg, StrInExpr, litx.into()));
                        return Err(self);
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
                            Value::Usize(num) => return Ok((*num as isize, self)),

                            Value::Str(_) => {
                                self.errors.push(err!(ExprMsg, StrInExpr, scope.into()));
                                return Err(self);
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

                        match self.evaluate(expr) {
                            Ok((value, mut s)) => {
                                s.updates.push((ident.to_string(), value));
                                return Ok((value as isize, s));
                            }

                            Err(s) => return Err(s)
                        }
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

                if (not_op && is_value) || child.ty() == At {
                    return self.eval_scope(child);
                }

                else if child.ty().parent_type() == Expr {
                    return self.eval_op(child);
                }

                bug!("Unexpected token in expression.");
            }
        }
    }

    fn eval_bin(
        mut self,
        f: fn(isize, isize) -> isize,
        op: &'a TokenRef<'a>, 
    ) -> Result<(isize, Self), Self> {
        let mut lhs = 0;
        let mut rhs = 0;

        match self.eval_scope(op.get(0)) {
            Ok((value, s)) => {
                lhs = value;
                self = s;
            }

            Err(s) => return Err(s)
        }

        match self.eval_scope(op.get(1)) {
            Ok((value, s)) => {
                rhs = value;
                self = s;
            }

            Err(s) => return Err(s)
        }

        Ok((f(lhs, rhs), self))
    }

    fn eval_op(
        mut self, 
        op: &'a TokenRef<'a>,
    ) -> Result<(isize, Self), Self> {
        assert_eq!(op.ty().parent_type(), Expr);

        match op.ty() {
            UnNot => {
                match self.eval_scope(op.get(0)) {
                    Ok((value, s)) => Ok((!value, s)),
                    Err(s) => Err(s)
                }
            }

            UnNeg => {
                match self.eval_scope(op.get(0)) {
                    Ok((value, s)) => Ok((-value, s)),
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
