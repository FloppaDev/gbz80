
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
};

use std::collections::HashMap;

pub enum ConstExpr<'a> {
    Num(usize),
    Str(Key),
    Tkn(&'a TokenRef<'a>),
}

pub struct Constants<'a> {
    map: HashMap<&'a TokenRef<'a>, Key>,
}

impl<'a> Constants<'a> {

    fn get_constants(
        ast: &'a TokenRef<'a>,
        mut map: HashMap<&'a str, ConstExpr<'a>>,
    ) -> HashMap<&'a str, ConstExpr<'a>> {
        let data = ast.data();

        for token in ast.children() {
            match token.ty() {
                MacroCall|MacroBody => map = Self::get_constants(token, map),

                Marker => {
                    let child = token.get(0);

                    match child.ty() {
                        Label => {
                            let ident = data.get_str(child.data_key());
                            let value = ConstExpr::Tkn(child);

                            map.insert(ident, value).unwrap();
                        }

                        NamedMark => {
                            let ident = data.get_str(child.data_key());
                            let value = ConstExpr::Num(
                                data.get_usize(child.get(0).get(0).data_key()));

                            map.insert(ident, value).unwrap();
                        }

                        _ => {}
                    }
                }

                Directive => {
                    let child = token.get(0);

                    if child.ty() == Define {
                        let ident = data.get_str(child.get(0).data_key());
                        let value = ConstExpr::Tkn(child);

                        map.insert(ident, value);
                    }
                }

                _ => {}
            }
        }

        map
    }

    fn insert(&mut self) {
        //TODO 
    }

    //TODO this is obsolete.
    fn get_markers(
        mut self,
        ast: &'a TokenRef<'a>,
        ops_map: &OpMap,
        data: &Data,
    ) -> HashMap<&'a TokenRef<'a>, usize> {
        let mut hashmap = HashMap::new();
        let mut offset = 0;

        Self::_walk(ast, ops_map, &mut hashmap, &mut offset, data);

        hashmap
    }

    fn _walk(
        ast: &'a TokenRef<'a>,
        ops_map: &OpMap,
        mut hashmap: &mut HashMap<&'a TokenRef<'a>, usize>,
        mut offset: &mut usize,
        data: &Data,
    ) {
        let mut size = 0;

        for token in ast.children() {
            match token.ty() {
                MacroCall => Self::_walk(token, ops_map, hashmap, offset, data),

                Instruction => {
                    let op = ops_map.get(token);
                    size = op.len as usize;
                }

                Lit => size = Constants::sizeof_lit(token, data),

                Identifier => {
                    //? Markers are always double.
                    //? Defines can have any size.
                    //TODO need hashmap of defines and their sizes  
                }

                Marker => {
                    //TODO Write offset to hashmap
                }
                _ => {}
            }

            *offset += size;
        }
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
