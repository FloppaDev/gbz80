
use crate::{
    write::{
        ops::OpMap,
        constants:: Constants,
    },
    token::{
        read::TokenRef,
    },
};

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(
    path: &str,
    ast: TokenRef, 
    op_map: &OpMap, 
    constants: &Constants
) {
    todo!();
}
