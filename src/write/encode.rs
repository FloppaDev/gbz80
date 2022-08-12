
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
    let mut bytes: Vec<u8> = vec![];

    for child in ast.children() {
        match child.ty() {
            MacroCall => {
                //TODO
            }

            Instruction => {
                let opcode = op_map.get(child);
                let mut op_bytes = vec![];

                if opcode.cb {
                    op_bytes.push(0xCB);
                }

                op_bytes.push(opcode.code);

                if op_bytes.len() != opcode.len as usize {
                    let children = child.children();

                    if let Some(args) = children.get(1..) {
                        for arg in args {
                            match arg.get(0).ty() {
                                Lit => {
                                    //TODO handle bit values
                                    //TODO push literal 
                                }

                                Identifier => {
                                    //TODO read identifier and push value
                                }

                                _ => {}
                            }
                        }
                    }
                }

                bytes.append(&mut op_bytes);
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
