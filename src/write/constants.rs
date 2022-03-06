
use crate::{
    parse::{
        lex::TokenType::*,
        data::{Data, Key},
    },
    program::{
        control::bug,
        error::{ConstantsErr, ConstantsErrType},
    },
    token::{
        read::TokenRef,
    },
};

use std::collections::HashMap;

/// Holds the value of a constant or the token required to calculate it.
#[derive(Copy, Clone)]
pub enum ConstExpr<'a> {
    Nil,
    Num(usize),
    Str(Key),
    Tkn(&'a TokenRef<'a>),
}

pub struct Constants<'a>(HashMap<&'a str, ConstExpr<'a>>);

impl<'a> Constants<'a> {

    pub fn map(&self) -> &HashMap<&'a str, ConstExpr<'a>> {
        let Self(map) = self;
        map
    }

    pub fn new(
        ast: &'a TokenRef<'a>,
    ) -> Result<Self, ConstantsErr<'a>> {
        let mut fail_safe = 500;
        let mut constants = Self(
            Self::get_constants(ast, HashMap::new(), &mut fail_safe)?);

        for key in constants.map().keys() {
            println!("{}", key);
        }

        Ok(constants)
    }

    fn get_constants(
        ast: &'a TokenRef<'a>,
        mut map: HashMap<&'a str, ConstExpr<'a>>,
        fail_safe: &mut usize,
    ) -> Result<HashMap<&'a str, ConstExpr<'a>>, ConstantsErr<'a>> {
        *fail_safe -= 1;

        if *fail_safe == 0 {
            bug("Recursion limit reached while reading constants");
        }

        let data = ast.data();
        let nil = Some(ConstExpr::Nil);

        for token in ast.children() {
            let err = ConstantsErr::new(ConstantsErrType::DuplicateKey, token.into());

            match token.ty() {
                MacroCall|MacroBody => map = Self::get_constants(token, map, fail_safe)?,

                Marker => {
                    let child = token.get(0);

                    match child.ty() {
                        Label => {
                            let ident = data.get_str(child.data_key());
                            let value = ConstExpr::Tkn(child);

                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        NamedMark => {
                            let ident = data.get_str(child.data_key());
                            let value = ConstExpr::Num(
                                data.get_usize(child.get(0).get(0).data_key()));

                            map.insert(ident, value).xor(nil).ok_or(err)?;
                        }

                        _ => {}
                    }
                }

                Directive => {
                    let child = token.get(0);

                    if child.ty() == Define {
                        let ident = data.get_str(child.get(0).data_key());
                        let value = ConstExpr::Tkn(child);

                        map.insert(ident, value).xor(nil).ok_or(err)?;
                    }
                }

                _ => {}
            }
        }

        Ok(map)
    }

    fn sizeof_lit(lit: &TokenRef<'a>, data: &Data) -> usize {
        let litx = lit.get(0); 
        return match litx.ty() {
            LitDec|LitHex|LitBin => Self::size_of(data.get_usize(litx.data_key())),

            LitStr => data.get_str(litx.data_key()).len(),

            _ => bug("Unhandled literal type."),
        }
    }

    fn size_of(value: usize) -> usize {
        return match value {
            value if value <= 255 => 1,

            value if value >= 256 && value <= 65536 => 2,

            _ => bug("Exceeding number capacity.")
        }
    }

    pub fn get_defines_sizes(
        defines: &[&TokenRef<'a>],
        data: &Data,
    ) -> HashMap<TokenRef<'a>, usize> {
        let hashmap = HashMap::new();

        for define in defines {
            let token = define.get(2);

            let size = match token.ty() {
                Lit => Self::sizeof_lit(token, data),

                ty => bug(&format!("Unexpected child type in define: {:?}", ty))
            };
        }

        hashmap
    }

}
