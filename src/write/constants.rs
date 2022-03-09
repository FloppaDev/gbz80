
use crate::{
    parse::{
        lex::TokenType::*,
    },
    token::{
        Value,
        read::TokenRef,
    },
    program::{
        error::{ConstantsErr, ConstantsErrType},
        RECURSION_LIMIT,
    },
    write::instructions::OpMap,
};

use std::collections::HashMap;

/// Holds the value of a constant or the token required to calculate it.
#[derive(Copy, Clone)]
pub enum ConstExpr<'a> {
    /// Not stored anywhere, used only for convenience.
    Nil,

    /// Location needs to be calculated.
    Mark,

    /// Known value.
    Value(Value<'a>),
}

pub struct Constants<'a> {
    map: HashMap<&'a str, ConstExpr<'a>>,
}

impl<'a> Constants<'a> {

    pub fn new(
        ast: &'a TokenRef<'a>,
        op_map: &OpMap<'a>,
    ) -> Result<Self, ConstantsErr<'a>> {
        let mut fail_safe = RECURSION_LIMIT;
        let mut map = Self::get_constants(ast, HashMap::new(), &mut fail_safe)?; 

        Self::resolve(&mut map, op_map, ast)?;

        Ok(Self{ map })
    }

    fn get_constants(
        ast: &'a TokenRef<'a>,
        mut map: HashMap<&'a str, ConstExpr<'a>>,
        fail_safe: &mut usize,
    ) -> Result<HashMap<&'a str, ConstExpr<'a>>, ConstantsErr<'a>> {
        *fail_safe -= 1;

        if *fail_safe == 0 {
            panic!("Recursion limit reached while reading constants");
        }

        let nil = Some(ConstExpr::Nil);

        for token in ast.children() {
            let err = ConstantsErr::new(ConstantsErrType::DuplicateKey, token.into());

            match token.ty() {
                MacroCall|MacroBody => map = Self::get_constants(token, map, fail_safe)?,

                Marker => {
                    let child = token.get(0);

                    match child.ty() {
                        Label => {
                            let ident = child.value().as_str();
                            let value = ConstExpr::Mark;

                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        NamedMark => {
                            let ident = child.value().as_str();
                            let value = ConstExpr::Value(*child.get(0).get(0).value());
                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        _ => {}
                    }
                }

                Directive => {
                    let child = token.get(0);
                    let ident = child.get(0).value().as_str();

                    match child.ty() {
                        DefB => {
                            let value = ConstExpr::Value(Value::Usize(1));
                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        DefW => {
                            let value = ConstExpr::Value(Value::Usize(2));
                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        DefS => {
                            let len = child.get(0).value().as_str().len();
                            let value = ConstExpr::Value(Value::Usize(len));
                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        Include => {
                            //TODO
                        }

                        _ => {}
                    }
                }

                _ => {}
            }
        }

        Ok(map)
    }

    /// Calculate values in expressions and labels.
    fn resolve(
        const_map: &mut HashMap<&'a str, ConstExpr<'a>>,
        op_map: &OpMap<'a>,
        ast: &'a TokenRef<'a>,
    ) -> Result<(), ConstantsErr<'a>> {
        let mut location = 0;

        // Calculate the size of labels and validate markers.
        for child in ast.children() {
            Self::size_of_token(const_map, op_map, child, &mut location)?; 
        }

        Ok(())
    }

    /// Increases the current location by the size in bytes of a token.
    //TODO rename, it assigns to location.
    fn size_of_token(
        const_map: &mut HashMap<&'a str, ConstExpr<'a>>,
        op_map: &OpMap<'a>,
        token: &'a TokenRef<'a>,
        location: &mut usize,
    ) -> Result<(), ConstantsErr<'a>> {
        match token.ty() {
            MacroCall => {
                for child in token.children() {
                    if child.ty() == MacroBody {
                        Self::size_of_token(const_map, op_map, child, location)?;
                        break;
                    }
                }
            }

            Instruction => *location += op_map.get(token).len as usize,

            Lit => *location += Self::size_of_lit(token),

            Identifier => {
                let ident = token.value().as_str();
                *location += Self::size_of_ident(const_map, op_map, ident)?;
            }

            Label => {
                let value = ConstExpr::Value(Value::Usize(*location));
                *const_map.get_mut(token.value().as_str()).unwrap() = value;
                *location += 2;
            }

            AnonMark|NamedMark => {
                let marker_location = token.get(0).get(0).value().as_usize();

                if *location == marker_location {
                    let value = ConstExpr::Value(Value::Usize(*location));
                    *const_map.get_mut(token.value().as_str()).unwrap() = value;
                }

                else {
                    return Err(ConstantsErr::new(
                        ConstantsErrType::MisplacedMarker, token.into()));
                }

                *location += 2;
            }

            _ => {}
        }

        Ok(())
    }
    
    fn size_of_ident(
        const_map: &mut HashMap<&'a str, ConstExpr<'a>>,
        op_map: &OpMap<'a>,
        ident: &'a str,
    ) -> Result<usize, ConstantsErr<'a>> {
        match const_map[ident] {
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

            value if (256..=65536).contains(&value) => 2,

            //TODO return Err
            _ => unreachable!("Exceeding number capacity.")
        }
    }

}
