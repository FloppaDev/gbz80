
use crate::{
    parse::{
        lex::TokenType::*,
    },
    token::{
        Value,
        read::TokenRef,
        expr::eval::ExprResult,
    },
    error::{
        ITERATION_LIMIT,
        asm::{
            AsmErr, 
            ConstantsMsg::{self, *},
            ExprMsg,
        },
    },
    write::ops::OpMap,
};

#[cfg(debug_assertions)]
use crate::program::fmt::title;

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

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

pub struct Constants<'a> {
    pub constants: Vec<(&'a str, ConstExpr<'a>)>,//TODO rename
    //TODO include data
    includes: HashMap<&'a str, Vec<u8>>,
}

impl<'a> Constants<'a> {

    pub fn get(&self, ident: &str) -> Option<&ConstExpr<'a>> {
        for (key, value) in &self.constants {
            if *key == ident {
                return Some(value)
            }
        }

        None
    }

    pub fn get_mut(&mut self, ident: &str) -> Option<&mut ConstExpr<'a>> {
        for (key, value) in &mut self.constants {
            if *key == ident {
                return Some(value)
            }
        }

        None
    }

    fn insert(&mut self, ident: &'a str, const_expr: ConstExpr<'a>) -> Result<(), ()> {
        for (key, _) in &self.constants {
            if *key == ident {
                return Err(());
            }
        }

        self.constants.push((ident, const_expr));

        Ok(()) 
    }

    fn entries(&self) -> &[(&'a str, ConstExpr<'a>)] {
        &self.constants
    }

    pub fn new(
        ast: &'a TokenRef<'a>,
        op_map: &OpMap<'a>,
    ) -> Result<Self, AsmErr<'a, ConstantsMsg>> {
        let mut fail_safe = ITERATION_LIMIT;
        let mut result = Self{ 
            constants: vec![],
            includes: HashMap::new(),
        };

        result = result.get_constants(ast, &mut fail_safe)?; 

        let mut location = 0;
        result.set_location(op_map, ast, &mut location)?; 

        //TODO remove debug code.
        println!();
        for (key, value) in &result.constants {
            let ty = match value {
                ConstExpr::Mark => "Mark",
                ConstExpr::Value(_) => "Value",
                ConstExpr::Expr(_) => "Expr",
            };

            println!("{}: {}", key, ty);
        }
        println!();

        Ok(result)
    }

    pub fn eval(&'a self) -> Result<Vec<(String, usize)>, Vec<AsmErr<'a, ExprMsg>>> {
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

    pub fn update(&mut self, updates: Vec<(String, usize)>) {
        for (ident, v) in updates {
            let value = ConstExpr::Value(Value::Usize(v));
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
                    let child = token.get(0);

                    match child.ty() {
                        Label => {
                            let ident = child.value().as_str();
                            let value = ConstExpr::Mark;

                             self.insert(ident, value).map_err(|_| err)?;
                        }

                        NamedMark => {
                            let ident = child.value().as_str();
                            let value = ConstExpr::Value(*child.get(0).get(0).value());
                            self.insert(ident, value).map_err(|_| err)?;
                        }

                        _ => {}
                    }
                }

                Directive => {
                    let child = token.get(0);

                    match child.ty() {
                        DefB|DefW => {
                            let ident = child.get(0).value().as_str();
                            let value = ConstExpr::Expr(child.get(1));
                            self.insert(ident, value).map_err(|_| err)?;
                        }

                        Include => {
                            let local = child.get(0).get(0).value().as_str();

                            if self.includes.get(local).is_none() {
                                //TODO put this code as a function in `Input`
                                let source = child.ast().source.main().path();

                                let path = match source.parent() {
                                    //TODO Path must be validated for to_str when building `Source`
                                    Some(dir) => format!("{}/{}", dir.to_str().unwrap(), local),
                                    None => local.into() 
                                };

                                let mut buffer = vec![];

                                let mut file = File::open(path).map_err(|_| err!(
                                    ConstantsMsg, FileReadFailed, child.into()))?;

                                file.read_to_end(&mut buffer).map_err(|_| err!(
                                    ConstantsMsg, FileReadFailed, child.into()))?;

                                self.includes.insert(local, buffer);
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

                Instruction => {
                    println!("{}", token.line_number());//TODO remove
                    *location += op_map.get(token).len as usize;
                }

                Lit => *location += Self::size_of_lit(token),

                Identifier => {
                    let ident = token.value().as_str();
                    *location += self.size_of_ident(ident)?;
                }

                Label => {
                    let value = ConstExpr::Value(Value::Usize(*location));
                    let key = token.value().as_str();
                    *self.get_mut(key).unwrap() = value;
                    *location += 2;
                }

                Marker => {
                    self.set_location(op_map, token.get(0), location)?;
                }

                AnonMark => {
                    let marker_location = token.get(0).get(0).value().as_usize();

                    if *location <= marker_location {
                        *location = marker_location;
                    }

                    else {
                        println!("{} != {}", *location, marker_location);//TODO remove
                        return Err(err!(ConstantsMsg, MisplacedMarker, token.into())); 
                    }
                }

                NamedMark => {
                    let marker_location = token.get(0).get(0).value().as_usize();

                    if *location <= marker_location {
                        let value = ConstExpr::Value(Value::Usize(*location));
                        let key = token.value().as_str();
                        *self.get_mut(key).unwrap() = value;
                        *location = marker_location;
                    }

                    else {
                        return Err(err!(ConstantsMsg, MisplacedMarker, token.into())); 
                    }
                }

                Directive => {
                    let dir = token.get(0);
                    
                    if dir.ty() == Include {
                        let path = dir.get(0).get(0).value().as_str();
                        *location += self.includes.get(path).unwrap().len();
                    }
                }

                _ => {}
            }
        }

        Ok(())
    }
    
    fn size_of_ident(&self, ident: &'a str) -> Result<usize, AsmErr<'a, ConstantsMsg>> {
        match self.get(ident).unwrap() {
            ConstExpr::Value(value) => {
                match value {
                    Value::Usize(v) => Ok(Self::size_of_num(*v)),

                    Value::Str(v) => Ok(v.len()),

                    _ => bug!("Unhandled `Value` type.")
                }
            }

            ConstExpr::Mark => Ok(2),

            ConstExpr::Expr(token) => {
                match token.parent().ty() {
                    DefB => Ok(1),
                    DefW => Ok(2),
                    _ => bug!("Unexpected token type")
                }
            }
        }
    }

    fn size_of_lit(lit: &TokenRef<'a>) -> usize {
        let litx = lit.get(0); 
        return match litx.ty() {
            LitDec|LitHex|LitBin => Self::size_of_num(litx.value().as_usize()),

            LitStr => litx.value().as_str().len(),

            _ => bug!("Unhandled literal type."),
        }
    }

    fn size_of_num(value: usize) -> usize {
        match value {
            value if value <= 255 => 1,

            value if (256..=65535).contains(&value) => 2,

            //TODO return Err
            _ => bug!("Exceeding number capacity.")
        }
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        title("Constant values");

        for (key, value) in &self.constants {
            let value_str = if let ConstExpr::Value(v) = value {
                match v {
                    Value::Usize(v) => v.to_string(),
                    Value::Str(v) => (*v).to_string(),
                    _ => bug!("Unexpected Value type")
                }
            }else {
                bug!("Unexpected ConstExpr type")
            };

            println!("{}: {}", key, value_str);
        }
    }

}
