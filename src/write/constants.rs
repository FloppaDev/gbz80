
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
    }
};

use std::collections::HashMap;

/// Holds the value of a constant or the token required to calculate it.
#[derive(Copy, Clone)]
pub enum ConstExpr<'a> {
    /// Not stored anywhere, used only for convenience.
    Nil,

    /// Location needs to be calculated.
    Label,

    /// Known value.
    Value(&'a Value<'a>),

    /// Expression needs to be resolved.
    Expr(&'a TokenRef<'a>),
}

pub struct Constants<'a> {
    map: HashMap<&'a str, ConstExpr<'a>>,
}

impl<'a> Constants<'a> {

    pub fn new(ast: &'a TokenRef<'a>) -> Result<Self, ConstantsErr<'a>> {
        let mut fail_safe = RECURSION_LIMIT;
        let mut map = Self::get_constants(ast, HashMap::new(), &mut fail_safe)?; 
        Self::resolve(&mut map);

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
                            let value = ConstExpr::Label;

                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        NamedMark => {
                            let ident = child.value().as_str();
                            let value = ConstExpr::Value(child.get(0).get(0).value());
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

    fn resolve(map: &mut HashMap<&'a str, ConstExpr<'a>>) {
        use ConstExpr::*;

        for (key, value) in map.iter_mut() {
            match value {
                Expr(token) => {
                    //TODO resolve math expressions
                }

                Label => {
                    //TODO *value = Num(x)
                }

                _ => {}
            }
        }
    }

    fn sizeof_lit(lit: &TokenRef<'a>) -> usize {
        let litx = lit.get(0); 
        return match litx.ty() {
            LitDec|LitHex|LitBin => Self::size_of(litx.value().as_usize()),

            LitStr => litx.value().as_str().len(),

            _ => unreachable!("Unhandled literal type."),
        }
    }

    fn size_of(value: usize) -> usize {
        match value {
            value if value <= 255 => 1,

            value if (256..=65536).contains(&value) => 2,

            //TODO return Option?
            _ => unreachable!("Exceeding number capacity.")
        }
    }

    pub fn get_defines_sizes(
        defines: &[&TokenRef<'a>],
    ) -> HashMap<TokenRef<'a>, usize> {
        let hashmap = HashMap::new();

        for define in defines {
            let token = define.get(2);

            let size = match token.ty() {
                Lit => Self::sizeof_lit(token),

                ty => unreachable!(
                    &format!("Unexpected child type in define: {:?}", ty))
            };
        }

        hashmap
    }

}
