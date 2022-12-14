
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

//TODO fill with zeroes to reach marks
//TODO checksums
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
                            let arg_x = arg.get(0);

                            match arg_x.ty() {
                                Lit => {
                                    if let Some(mut bytes) = arg_x.lit_to_bytes() {
                                        op_bytes.append(&mut bytes);
                                    }

                                    else {
                                        bug!("Could not read literal.");
                                    }
                                }

                                Identifier => {
                                    match arg_x.value() {
                                        //TODO put in common with code in TokenRef::lit_to_bytes(). 
                                        Value::Usize(u) => {
                                            let mut b = (*u as u16).to_be_bytes().to_vec();
                                            op_bytes.append(&mut b);
                                        }
                                        
                                        Value::Str(s) => {
                                            //TODO check encoding
                                            let mut b = s.as_bytes().to_vec();
                                            op_bytes.append(&mut b);
                                        }

                                        _ => bug!("Invalid constant.")
                                    }
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
                        match v {
                            //TODO put in common with code in TokenRef::lit_to_bytes(). 
                            Value::Usize(u) => {
                                let mut b = (*u as u16).to_be_bytes().to_vec();
                                bytes.append(&mut b);
                            }
                            
                            Value::Str(s) => {
                                //TODO check encoding
                                let mut b = s.as_bytes().to_vec();
                                bytes.append(&mut b);
                            }

                            _ => bug!("Invalid constant.")
                        }
                    }

                    _ => bug!("Could not read constant.")
                }
            }

            Lit => {
                if let Some(mut b) = child.lit_to_bytes() {
                    bytes.append(&mut b);
                }

                else {
                    bug!("Could not read literal.");
                }
            }

            Directive => {
                if child.get(0).ty() == Include {
                    let path = child.leaf().value().as_str();
                    if let Some(b) = constants.includes.get(path) {
                        let mut b = b.clone();
                        bytes.append(&mut b);
                    }

                    else {
                        bug!("Could not include file."); 
                    }
                }
            }

            _ => {

            }
        }
    }

    Ok(bytes)
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
