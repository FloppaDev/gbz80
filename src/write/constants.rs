
use crate::{
    parse::{ lex::TokenType::* },
    token::{ Value, read::TokenRef, expr::eval::{ExprResult, ExprValue } },
    error::{ ITERATION_LIMIT, asm::{ AsmErr, ConstantsMsg::{self, *}, ExprMsg } },
    write::ops::OpMap,
};

#[cfg(debug_assertions)]
use crate::program::fmt::title;

use std::collections::HashMap;

/// Holds the value of a constant or the token required to calculate it.
#[derive(Copy, Clone)]
pub enum ConstExpr<'a> {
    /// Location needs to be calculated.
    Mark,
    /// Known value.
    Value(Value<'a>),
    /// Value needs to be calculated.
    Expr(&'a TokenRef<'a>),
}

impl<'a> ConstExpr<'a> {

    pub fn as_value(&self) -> Result<Value<'a>, ()> {
        return if let ConstExpr::Value(value) = self {
            return Ok(*value);
        }else {
            Err(())
        };
    }

}

pub struct Constants<'a> {
    pub const_exprs: Vec<(&'a str, ConstExpr<'a>)>,
    pub includes: HashMap<&'a str, Vec<u8>>,
}

impl<'a> Constants<'a> {

    pub fn get(&self, ident: &str) -> Option<&ConstExpr<'a>> {
        for (key, value) in &self.const_exprs {
            if *key == ident {
                return Some(value)
            }
        }

        None
    }

    pub fn get_mut(&mut self, ident: &str) -> Option<&mut ConstExpr<'a>> {
        for (key, value) in &mut self.const_exprs {
            if *key == ident {
                return Some(value)
            }
        }

        None
    }

    fn insert(&mut self, ident: &'a str, const_expr: ConstExpr<'a>) -> Result<(), ()> {
        for (key, _) in &self.const_exprs {
            if *key == ident {
                return Err(());
            }
        }

        self.const_exprs.push((ident, const_expr));
        Ok(()) 
    }

    fn entries(&self) -> &[(&'a str, ConstExpr<'a>)] {
        &self.const_exprs
    }

    pub fn new(
        ast: &'a TokenRef<'a>,
        op_map: &OpMap<'a>,
    ) -> Result<Self, AsmErr<'a, ConstantsMsg>> {
        let mut fail_safe = ITERATION_LIMIT;
        let mut result = Self{ 
            const_exprs: vec![],
            includes: HashMap::new(),
        };

        result = result.get_constants(ast, &mut fail_safe)?; 

        let mut location = 0;
        result.set_location(op_map, ast, &mut location)?; 

        Ok(result)
    }

    /// Evaluates all constant expressions and returns the results.
    pub fn eval(&'a self) -> Result<Vec<(String, ExprValue)>, Vec<AsmErr<'a, ExprMsg>>> {
        let exprv = self.entries().iter()
            .filter(|(_, v)| matches!(v, ConstExpr::Expr(_)))
            .map(|(_, v)| match v { ConstExpr::Expr(e) => *e, _ => unreachable!() })
            .collect::<Vec<_>>();

        let mut updates = vec![];
        let mut errors = vec![];

        for expr in exprv {
            match ExprResult::eval(expr, self) {
                Ok(mut result) => updates.append(&mut result.updates),
                Err(mut e) => errors.append(&mut e)
            }
        }

        if errors.is_empty() {
            Ok(updates)
        }else {
            Err(errors)
        }
    }

    /// Applies updates on the constants' values.
    pub fn update(&mut self, updates: Vec<(String, ExprValue)>) {
        for (ident, v) in updates {
            let value = match v {
                ExprValue::U8(v) => ConstExpr::Value(Value::U8(v)),
                ExprValue::U16(v) => ConstExpr::Value(Value::U16(v)),
            };

            *self.get_mut(&ident).unwrap() = value;
        }
    }

    fn get_constants(
        mut self,
        ast: &'a TokenRef<'a>,
        fail_safe: &mut usize,
    ) -> Result<Self, AsmErr<'a, ConstantsMsg>> {
        *fail_safe -= 1;

        assert!(*fail_safe != 0, 
            "Recursion limit reached while reading constants");

        for token in ast.children() {
            let err = err!(ConstantsMsg, DuplicateKey, token.into());

            match token.ty() {
                MacroCall|MacroBody => self = self.get_constants(token, fail_safe)?,

                Marker => {
                    let child = token.first();

                    match child.ty() {
                        Label => {
                            let ident = child.value().as_str().unwrap();
                            let value = ConstExpr::Mark;
                             self.insert(ident, value).map_err(|_| err)?;
                        }

                        NamedMark => {
                            let ident = child.value().as_str().unwrap();
                            let value = ConstExpr::Value(*child.first().first().value());
                            self.insert(ident, value).map_err(|_| err)?;
                        }

                        _ => {}
                    }
                }

                Directive => {
                    let child = token.first();

                    match child.ty() {
                        DefB|DefW => {
                            let ident = child.first().value().as_str().unwrap();
                            let value = ConstExpr::Expr(child.get(1));
                            self.insert(ident, value).map_err(|_| err)?;
                        }

                        Include => {
                            let local = child.first().first().value().as_str().unwrap();

                            if self.includes.get(local).is_none() {
                                let data = child.ast().source.main().read_local(local).map_err(|_|
                                    err!(ConstantsMsg, FileReadFailed, token.into()))?;

                                self.includes.insert(local, data);
                            }
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        Ok(self)
    }

    /// Increases the current location by the size in bytes of a token.
    fn set_location(
        &mut self,
        op_map: &OpMap<'a>,
        root: &'a TokenRef<'a>,
        location: &mut usize,
    ) -> Result<(), AsmErr<'a, ConstantsMsg>> {
        for token in root.children() {
            match token.ty() {
                MacroCall => {
                    for child in token.children() {
                        if child.ty() == MacroBody {
                            self.set_location(op_map, child, location)?;
                            break;
                        }
                    }
                }

                Instruction => *location += op_map.get(token).len as usize,
                Lit => *location += Self::size_of_lit(token),

                Identifier => {
                    let ident = token.value().as_str().unwrap();
                    *location += self.size_of_ident(ident);
                }

                Label => {
                    let value = ConstExpr::Value(Value::U16(*location as u16));
                    let key = token.value().as_str().unwrap();
                    *self.get_mut(key).unwrap() = value;
                    //*location += 2; TODO commented out, make sure it was an error
                }

                Marker => self.set_location(op_map, token, location)?,

                AnonMark|NamedMark => {
                    let marker_location = token.first().first().value().as_num().unwrap();

                    if *location <= marker_location {
                        *location = marker_location;
                    }

                    else {
                        return Err(err!(ConstantsMsg, MisplacedMarker, token.into())); 
                    }
                }

                Directive => {
                    let dir = token.first();
                    
                    if dir.ty() == Include {
                        let path = dir.first().first().value().as_str().unwrap();
                        *location += self.includes.get(path).unwrap().len();
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }
    
    fn size_of_ident(&self, ident: &'a str) -> usize {
        match self.get(ident).unwrap() {
            ConstExpr::Value(value) => {
                match value {
                    Value::U8(_) => 1,
                    Value::U16(_) => 2,
                    Value::Str(v) => v.len(),
                    _ => bug!("Unhandled `Value` type.")
                }
            }

            ConstExpr::Mark => 2,

            ConstExpr::Expr(token) => {
                match token.parent().ty() {
                    DefB => 1,
                    DefW => 2,
                    _ => bug!("Unexpected token type")
                }
            }
        }
    }

    fn size_of_lit(lit: &TokenRef<'a>) -> usize {
        match lit.first().value() {
            Value::U8(_) => 1,    
            Value::U16(_) => 2,    
            Value::Str(v) => v.len(),    
            _ => bug!("Unhandled literal type."),
        }
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        title("Constant values");

        for (key, value) in &self.const_exprs {
            let value_str;
            let ty_str;

            if let ConstExpr::Value(v) = value {
                match v {
                    Value::U8(v) => {
                        value_str = v.to_string();
                        ty_str = "BYTE";
                    }
                    Value::U16(v) => {
                        value_str = v.to_string();
                        ty_str = "WORD";
                    }
                    Value::Str(v) => {
                        value_str = (*v).to_string();
                        ty_str = "STRG";
                    }
                    _ => bug!("Unexpected Value type")
                }
            }else {
                bug!("Unexpected ConstExpr type");
            };

            let len = 48usize.saturating_sub(key.len());
            let bar = "─".repeat(len);
            let hex = value_str.parse::<usize>().unwrap();

            println!("{ty_str} {key} {bar} 0x{hex:X} {value_str}");
        }
    }

}
