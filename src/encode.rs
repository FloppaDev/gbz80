use crate::opcodes::{InstructionDef, Op};
use crate::ast::{Token, TokenType::{self, *}};

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(ast: TokenRef, instructions: &[InstructionDef]) -> Vec<u8> {
    todo!();
}
