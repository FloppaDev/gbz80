
use crate::{
    write::{
        ops::OpMap,
        constants::{Constants, ConstExpr},
    },
    token::{
        read::TokenRef,
        Value,
    },
    parse::lex::TokenType::*,
};

use std::fs::File;
use std::io::prelude::*;

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn write(bytes: &[u8], path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(bytes)?;

    Ok(())
}

pub fn encode(
    ast: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants
) -> Result<Vec<u8>, ()> {
    let bytes: Vec<u8> = vec![];

    for child in ast.children() {
        match child.ty() {
            MacroCall => {
                //TODO
            }

            Instruction => {
                let opcode = op_map.get(child);
                println!("{:?}", opcode);
            }

            Identifier => {
                match constants.get(child.value().as_str()).unwrap() {
                    ConstExpr::Value(v) => {
                        println!("{:?}", v);
                    }

                    _ => {

                    }
                }
            }

            Lit => {

            }

            _ => {

            }
        }
    }

    Ok(vec![])
}

pub fn build(
    path: String,
    ast: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants
) -> Result<(), ()> {
    let bytes = encode(ast, op_map, constants)?;
    write(&bytes, &path).map_err(|_| ())?;

    Ok(())
}
