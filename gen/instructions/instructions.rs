const instructions_rs = `\
\
use crate::{
    write::ops::{ Arg, OpCode, ty, imm, at, bit, Constant::* },
    parse::lex::TokenType::{ self, * },
    token::read::TokenRef,
};

pub fn find(instruction: &TokenRef) -> Option<OpCode> {
    assert_eq!(instruction.ty(), Instruction);

    let instr_ty = instruction.first().first().ty();
    let (cb, ops) = get_instruction_info(instr_ty);

    OpCode::get_opcode(instruction, cb, ops)
}

#[cfg(test)]
pub fn get_instruction_info(tty: TokenType) -> (bool, Vec<(u8, u8, Vec<Arg>)>) {
    _get_instruction_info(tty)
}

fn get_instruction_info(tty: TokenType) -> (bool, Vec<(u8, u8, Vec<Arg>)>) {
    _get_instruction_info(tty)
}

fn _get_instruction_info(tty: TokenType) -> (bool, Vec<(u8, u8, Vec<Arg>)>) {
    // {{{ js }}}
}
`;
