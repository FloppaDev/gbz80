
use crate::{
    parse::{
        lex::TokenType::*,
    },
    token::{
        Value,
        read::TokenRef,
    },
    error::{
        ITERATION_LIMIT,
        asm::{
            AsmErr, 
            ConstantsMsg::{self, *},
        },
    },
    write::ops::OpMap,
};

use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

/// Holds the value of a constant or the token required to calculate it.
#[derive(Copy, Clone)]
pub enum ConstExpr<'a> {
    /// Not stored anywhere, used only for convenience.
    Nil,

    /// Location needs to be calculated.
    Mark,

    /// Known value.
    Value(Value<'a>),

    /// Value needs to be calculated.
    Expr(&'a TokenRef<'a>),
}

pub struct Constants<'a> {
    constants: HashMap<&'a str, ConstExpr<'a>>,
    //TODO process includes before parse.
    includes: HashMap<&'a str, Vec<u8>>,
}

impl<'a> Constants<'a> {

    #[allow(dead_code)]
    pub fn get(&self, ident: &str) -> &ConstExpr {
        self.get(ident).unwrap() 
    }

    pub fn get_mut(&mut self, ident: &str) -> &mut ConstExpr {
        self.get_mut(ident).unwrap() 
    }

    pub fn new(
        ast: &'a TokenRef<'a>,
        op_map: &OpMap<'a>,
    ) -> Result<Self, AsmErr<'a, ConstantsMsg>> {
        let mut fail_safe = ITERATION_LIMIT;
        let mut constants = Self{ 
            constants: HashMap::new(), 
            includes: HashMap::new(),
        };

        constants = constants.get_constants(ast, &mut fail_safe)?; 

        let mut location = 0;

        // Calculate the size of labels and validate markers.
        for child in ast.children() {
            constants.set_location(op_map, child, &mut location)?; 
        }

        Ok(constants)
    }

    fn get_constants(
        mut self,
        ast: &'a TokenRef<'a>,
        fail_safe: &mut usize,
    ) -> Result<Self, AsmErr<'a, ConstantsMsg>> {
        *fail_safe -= 1;

        if *fail_safe == 0 {
            panic!("Recursion limit reached while reading constants");
        }

        let nil = Some(ConstExpr::Nil);

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

                             self.constants.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        NamedMark => {
                            let ident = child.value().as_str();
                            let value = ConstExpr::Value(*child.get(0).get(0).value());
                            self.constants.insert(ident, value).xor(nil).ok_or(err)?;
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
                            self.constants.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        DefS => {
                            let ident = child.get(0).value().as_str();
                            let str_value = child.get(0).value();
                            let value = ConstExpr::Value(*str_value);
                            self.constants.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        Include => {
                            let path = child.get(0).get(0).value().as_str();
                            
                            if self.includes.get(path).is_none() {
                                let mut buffer = vec![];
                                let mut file = File::open(path).map_err(|_| err!(
                                    ConstantsMsg, FileReadFailed, child.into()))?;
                                file.read_to_end(&mut buffer).map_err(|_| err!(
                                    ConstantsMsg, FileReadFailed, child.into()))?;
                                self.includes.insert(path, buffer);
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
        token: &'a TokenRef<'a>,
        location: &mut usize,
    ) -> Result<(), AsmErr<'a, ConstantsMsg>> {
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
                let ident = token.value().as_str();
                *location += self.size_of_ident(ident)?;
            }

            Label => {
                let value = ConstExpr::Value(Value::Usize(*location));
                *self.constants.get_mut(token.value().as_str()).unwrap() = value;
                *location += 2;
            }

            AnonMark|NamedMark => {
                let marker_location = token.get(0).get(0).value().as_usize();

                if *location == marker_location {
                    let value = ConstExpr::Value(Value::Usize(*location));
                    *self.constants.get_mut(token.value().as_str()).unwrap() = value;
                }

                else {
                    return Err(err!(ConstantsMsg, MisplacedMarker, token.into())); }

                *location += 2;
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

        Ok(())
    }
    
    fn size_of_ident(&self, ident: &'a str) -> Result<usize, AsmErr<'a, ConstantsMsg>> {
        match self.constants[ident] {
            ConstExpr::Value(value) => {
                match value {
                    Value::Usize(v) => Ok(Self::size_of_num(v)),

                    Value::Str(v) => Ok(v.len()),

                    _ => unreachable!()
                }
            }

            ConstExpr::Mark => Ok(2),

            _ => unreachable!(),
        }
    }

    fn size_of_lit(lit: &TokenRef<'a>) -> usize {
        let litx = lit.get(0); 
        return match litx.ty() {
            LitDec|LitHex|LitBin => Self::size_of_num(litx.value().as_usize()),

            LitStr => litx.value().as_str().len(),

            _ => unreachable!("Unhandled literal type."),
        }
    }

    fn size_of_num(value: usize) -> usize {
        match value {
            value if value <= 255 => 1,

            value if (256..=65535).contains(&value) => 2,

            //TODO return Err
            _ => unreachable!("Exceeding number capacity.")
        }
    }

}
