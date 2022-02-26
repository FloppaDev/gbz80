const instructions_rs = `\
\
use crate::{
    lingo::TokenType::{self, *},
    token::TokenRef,
    error::{ErrCtx, OpErr, OpErrType},
};

use Constant::*;

use std::collections::HashMap;

enum Arg {
    /// Address.
    At(Box<Arg>),

    /// Identified by a \`TokenType\`.
    Token(TokenType),

    /// Constant value.
    Const(Constant),
}

impl Arg {

    fn cmp(token: Option<&TokenRef>) -> bool {
        
    }

}

pub enum Constant {
    Byte,
    Word,
}

pub struct OpCode {
    pub cb: bool,
    pub code: u8,
    pub len: u8,
}

impl OpCode {

    fn get_opcode(cb: bool, ops: Vec<(usize, usize, Vec<Arg>)>) -> Option<OpCode> {

    }

    pub fn find(instruction: &TokenRef) -> Option<OpCode> {
        assert_eq!(instruction.ty(), Instruction);

        let instr_ty = instruction.get(0).get(0).ty();

        // {{{ js }}}
    }

}

pub struct OpMap<'a>(HashMap<&'a TokenRef<'a>, OpCode>);

impl<'a> OpMap<'a> {
    
    pub fn get(&self, token: &TokenRef<'a>) -> &OpCode {
        let Self(map) = self; 

        map.get(token).unwrap()
    }

}

impl<'a> OpMap<'a> {

    pub fn new(ast: &'a TokenRef<'a>) -> Result<Self, Vec<OpErr<'a>>> {
        let mut map = HashMap::new(); 
        let mut errors = vec![];

        fn walk<'a>(
            ast: &'a TokenRef<'a>,
            map: &mut HashMap<&'a TokenRef<'a>, OpCode>, 
            errors: &mut Vec<OpErr<'a>>,
        ) {
            for token in ast.children() {
                match token.ty() {
                    MacroCall => walk(&token, map, errors),

                    Instruction => {
                        let opcode = OpCode::find(&token);

                        if opcode.is_none() {
                            errors.push(
                                OpErr::new(OpErrType::NotFound, (&token).into()));

                            continue;
                        }

                        map.insert(&token, opcode.unwrap());
                    }

                    _ => {}
                }
            }
        }

        walk(ast, &mut map, &mut errors);

        if !errors.is_empty() {
            Err(errors)
        }else {
            Ok(Self(map))
        }
    }

}`;
