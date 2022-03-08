
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

    /// Expression needs to be resolved.
    Expr(&'a TokenRef<'a>),
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

        Self::resolve(&mut map, op_map, ast);

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

                    if child.ty() == Define {
                        let ident = child.get(0).value().as_str();
                        let value = ConstExpr::Expr(child);
                        map.insert(ident, value).xor(nil).ok_or(err)?;
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
    ) {
        let mut location = 0;

        // Calculate the size of labels and validate markers.
        for child in ast.children() {
            location += size_of_token(child)?; 
        }
    }

    fn size_of_token(
        const_map: &mut HashMap<&'a str, ConstExpr<'a>>,
        op_map: &OpMap<'a>,
        token: &'a TokenRef<'a>,
    ) Result<usize, ConstantsErr<'a>> {
        return match child.ty() {
            MacroCall => {}//TODO recursion.

            Instruction => location += op_map.get(child).len as usize,

            Lit => location += Self::size_of_lit(child),

            Identifier => Self::size_of_ident(child.value().as_str())?,

            Label => {
                let value = ConstExpr::Value(Value::Usize(location));
                *const_map.get_mut(child.value().as_str()).unwrap() = value;
                location += 2;
            }

            AnonMark|NamedMark => {
                let marker_location = child.get(0).get(0).value().as_usize();

                if location == marker_location {
                    let value = ConstExpr::Value(Value::Usize(location));
                    *const_map.get_mut(child.value().as_str()).unwrap() = value;
                }

                else {
                    return ConstantsErr::new(
                        child.into(), ConstantsErrType::MisplacedMarker));
                }

                location += 2;
            }

            _ => 0 
        }
    }
    
    fn size_of_ident(
        const_map: &mut HashMap<&'a str, ConstExpr<'a>>,
        op_map: &OpMap<'a>,
        ident: &'a str,
    ) Result<usize, ConstantsErr<'a>> {
        match const_map[ident] {
            ConstExpr::Value(value) => {
                match value {
                    Value::Usize(v) => location += Self::size_of_num(v),

                    Value::Str(v) => location += v.len(),

                    _ => unreachable!()
                }
            },

            ConstExpr::Expr(expr) => {
                //TODO
                // Try to find size of the expr, 
                // or its dependencies, then the expr.
            },

            ConstExpr::Mark => location += 2,

            _ => unreachable!(),
        }
    }

    fn size_of_expr(lit: &TokenRef<'a>) -> Option<usize> {

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

            //TODO return Option?
            _ => unreachable!("Exceeding number capacity.")
        }
    }

}
