const instructions_rs = `\
\
use crate::{
    write::ops::{OpCode, Arg, Constant::*},
    parse::lex::TokenType::*,
    token::{
        read::TokenRef,
    },
};

pub fn find(instruction: &TokenRef) -> Option<OpCode> {
    assert_eq!(instruction.ty(), Instruction);

    let instr_ty = instruction.get(0).get(0).ty();

    // {{{ js }}}
}`;
