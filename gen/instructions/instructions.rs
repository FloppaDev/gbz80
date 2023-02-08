const instructions_rs = `\
\
use crate::{
    write::ops::{ OpCode, ty, imm, at, bit, Constant::* },
    parse::lex::TokenType::*,
    token::read::TokenRef,
};

pub fn find(instruction: &TokenRef) -> Option<OpCode> {
    assert_eq!(instruction.ty(), Instruction);

    let instr_ty = instruction.first().first().ty();

    // {{{ js }}}
}`;
