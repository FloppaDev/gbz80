const instructions_rs = `\
\
use crate::{
    parse::lex::TokenType::{self, *},
    token::{
        read::TokenRef,
    },
    program::{
        error::{OpErr, OpErrType},
        control::bug,
    }
};

use Constant::*;

use std::collections::HashMap;

pub enum Constant {
    Byte,
    Word,
}

enum Arg {
    /// Address.
    At(Box<Arg>),

    /// Identified by a \`TokenType\`.
    Token(TokenType),

    /// Constant value.
    Const(Constant),
}

impl Arg {

    const fn cmp(token: &TokenRef) -> bool {
        todo!()//TODO? 
    }

}

pub struct OpMap<'a>(HashMap<&'a TokenRef<'a>, OpCode>);

impl<'a> OpMap<'a> {

    pub fn get(&self, token: &TokenRef<'a>) -> &OpCode {
        let Self(map) = self; 

        map.get(token).unwrap()
    }

    pub fn new(ast: &'a TokenRef<'a>) -> Result<Self, Vec<OpErr<'a>>> {
        let mut map = HashMap::new(); 
        let mut errors = vec![];

        Self::walk(ast, &mut map, &mut errors);

        if !errors.is_empty() {
            Err(errors)
        }else {
            Ok(Self(map))
        }
    }

    fn walk(
        ast: &'a TokenRef<'a>,
        map: &mut HashMap<&TokenRef<'a>, OpCode>, 
        errors: &mut Vec<OpErr<'a>>,
    ) {
        for token in ast.children() {
            match token.ty() {
                MacroCall => Self::walk(token, map, errors),

                Instruction => {
                    let opcode = OpCode::find(token);

                    if opcode.is_none() {
                        errors.push(
                            OpErr::new(OpErrType::NotFound, token.into()));

                        continue;
                    }

                    map.insert(token, opcode.unwrap());
                }

                _ => {}
            }
        }
    }

}

pub struct OpCode {
    pub cb: bool,
    pub code: u8,
    pub len: u8,
}

impl OpCode {

    fn cmp_args(
        instr_args: &[&TokenRef],
        op_args: &[Arg],
    ) -> bool {
        todo!()//TODO
    }

    fn get_opcode(
        instruction: &TokenRef, 
        cb: bool, 
        ops: Vec<(u8, u8, Vec<Arg>)>
    ) -> Option<Self> {
        let instr_children = instruction.children();

        for op in ops {
            let (len, code, op_args) = op;

            if Self::cmp_args(&instr_children[1..], &op_args) {
                let opcode = Self{ cb, code, len };

                return Some(opcode);     
            }
        }

        None
    }

    pub fn find(instruction: &TokenRef) -> Option<Self> {
        assert_eq!(instruction.ty(), Instruction);

        let instr_ty = instruction.get(0).get(0).ty();

        // {{{ js }}}
    }

}`;
