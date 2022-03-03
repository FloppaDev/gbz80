
//  Within expressions, markers and defines are both identifiers.
//  The size of an identifier could be 1 or 2 bytes, or anything with strings.

//  #def FOO 10                         ; size is 1
//  #def MOO "uwu~"                     ; size is 4
//  #def BAZ FOO
//  #def ABC FOO + BAZ
//  #def UGH :Label2 + 1                ; size is 2

//  add a                   ; Instruction size: 1 byte
//  &01                     ; 1 byte
//  &2938                   ; 2 bytes
//  MOO                     ; 4 (no \0)
//  Label1:                 ; @8
//  10                      ; 1
//  1000                    ; 2
//  FOO                     ; 1 
//  "Hello"                 ; 5
//  BAR                     ; 1? 2?
//  :Label2                 ; @?
//  BAR                     ; 1? 2?
//  BAR                     ; 1? 2?
//  &00FF:Marker            ; @256

//  The first thing to do is to determine which identifiers are markers/labels because
//      their size will always be 2 bytes.
//
//  Create a dictionnary of all defines and check for undefined identifiers.
//
//  ABC depends on BAZ and FOO: try to calculate them before calculating ABC.
//      There's a possiblity for circular dependencies.



//  - Get labels values
//  loop {
//      - iterate through all constants and try to calculate their values.
//      if count of unknown values remains the same {
//          return error
//      }
//      if all values are known {
//          break
//      }
//  }

use crate::{
    lingo::TokenType::*,
    token::{Token, TokenRef},
    data::{Data, Key},
    instructions::{OpCode, OpMap},
    process::bug,
    error::{ConstantsErr, ConstantsErrType},
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

    //TODO? Expressions always evaluate to a double
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
