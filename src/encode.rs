use opcodes::Instruction;
use ast::Token;

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(ast: Token, instructions: Vec<Instruction>) -> Vec<u8> {
    todo!();
}
