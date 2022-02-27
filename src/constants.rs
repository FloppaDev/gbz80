
//  Within expressions, markers and defines are both identifiers.
//  The size of an identifier could be 1 or 2 bytes, or anything with strings.

//  #def FOO 10                         ; size is 1
//  #def MOO "uwu~"                     ; size is 4

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

use crate::{
    lingo::TokenType::*,
    token::{Token, TokenRef},
    data::{Data, Key},
    instructions::{OpCode, OpMap},
};

use std::collections::HashMap;

pub struct Constants<'a> {
    map: HashMap<TokenRef<'a>, Key>,
}

impl<'a> Constants<'a> {

    fn insert(&mut self) {
        //TODO 
    }

    fn get_markers(
        mut self,
        ast: TokenRef<'a>,
        ops_map: &OpMap,
        data: &Data,
    ) -> HashMap<TokenRef<'a>, usize> {
        let mut hashmap = HashMap::new();
        let mut offset = 0;

        Self::walk(ast, ops_map, &mut hashmap, &mut offset, data);

        hashmap
    }

    fn walk(
        ast: TokenRef<'a>,
        ops_map: &OpMap,
        mut hashmap: &mut HashMap<TokenRef<'a>, usize>,
        mut offset: &mut usize,
        data: &Data,
    ) {
        let mut size = 0;

        for token in ast.children() {
            match token.ty() {
                MacroCall => Self::walk(*token, ops_map, hashmap, offset, data),

                Instruction => {
                    let op = ops_map.get(token);
                    size = op.len as usize;
                }

                Lit => size = Constants::sizeof_lit(*token, data),

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

    fn sizeof_lit(lit: TokenRef<'a>, data: &Data) -> usize {
        let litx = lit.get(0); 
        return match litx.ty() {
            LitDec|LitHex|LitBin => 
                if data.get_u16(litx.data_key()).is_some() { 2 }else{ 1 },

            LitStr => data.get_str(litx.data_key()).len(),

            // No other type of literal.
            _ => panic!(),
        }
    }

    fn sizeof_lit_dec(litd: &str) -> usize {
        let value = litd.parse::<usize>().unwrap();
        return match value {
            value if value <= 255 => 1,

            value if value >= 256 && value <= 65536 => 2,

            _ => panic!("Value not allowed for a decimal literal {}", litd)
        }
    }

    //TODO? Expressions always evaluate to a double
    pub fn get_defines_sizes(
        defines: &[TokenRef<'a>],
        data: &Data,
    ) -> HashMap<TokenRef<'a>, usize> {
        let hashmap = HashMap::new();

        for define in defines {
            let token = define.get(2);

            let size = match token.ty() {
                Lit => Self::sizeof_lit(*token, data),
                ty => panic!("Unexpected child type in define: {:?}", ty) 
            };
        }

        hashmap
    }

}
