
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
                todo!()
            }

            // Fill empty space with zeroes to reach markers.
            Marker => {
                let marker_kind = child.get(0);

                let location = match marker_kind.ty() {
                    NamedMark | AnonMark => marker_kind.leaf().value().as_usize(),
                    Label => continue,
                    _ => bug!("Invalid Marker type."),
                };

                let diff = location - bytes.len();
                let mut fill = vec![0; diff];

                bytes.append(&mut fill);
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
                                    arg_x.lit_to_bytes().map_or_else(|| {
                                        bug!("Could not read literal.");
                                    }, |mut bytes| {
                                        op_bytes.append(&mut bytes);
                                    });
                                }

                                Identifier => {
                                    match arg_x.value() {
                                        
                                        
                                        Value::Str(s) => {
                                            let const_expr = constants.get(s).unwrap();

                                            if let ConstExpr::Value(value) = const_expr {
                                                match value {
                                                    Value::Usize(u) => {
                                                        //TODO was it always u16?
                                                        let mut b = (*u as u16).to_be_bytes().to_vec();
                                                        op_bytes.append(&mut b);
                                                    }            

                                                    Value::Str(s) => {
                                                        //TODO check encoding
                                                        let mut b = s.as_bytes().to_vec();
                                                        op_bytes.append(&mut b);
                                                    }

                                                    _ => bug!("Invalid Value.")
                                                }
                                            }

                                            else {
                                                bug!("Invalid constant.");
                                            }
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
                if let ConstExpr::Value(v) = constants.get(child.value().as_str()).unwrap() {
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

                else {
                    bug!("Could not read constant.");
                }
            }

            Lit => {
                child.lit_to_bytes().map_or_else(|| {
                    bug!("Could not read literal.");
                }, |mut b| {
                    bytes.append(&mut b);
                });
            }

            Directive => {
                if child.get(0).ty() == Include {
                    let path = child.leaf().value().as_str();

                    constants.includes.get(path).map_or_else(|| {
                        bug!("Could not include file."); 
                    }, |b| {
                        let mut b = b.clone();
                        bytes.append(&mut b);
                    });
                }
            }

            _ => {

            }
        }
    }

    Ok(bytes)
}

pub fn build(
    path: &str,
    ast: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants
) -> Result<(), ()> {
    let bytes = encode(ast, op_map, constants)?;
    write(&bytes, path).map_err(|_| ())?;

    Ok(())
}
