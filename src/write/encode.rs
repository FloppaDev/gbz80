
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

pub fn str_to_bytes(s: &str) -> Result<Vec<u8>, ()> {
    let mut bytes = vec![];

    for c in s.chars() {
        if !c.is_ascii() {
            return Err(());
        }

        bytes.push(c as u8);
    }

    Ok(bytes)
}

pub fn usize_to_bytes(u: usize) -> Result<Vec<u8>, ()> {
    return if u > u16::MAX as usize {
        Err(())
    }else if u > u8::MAX as usize {
        Ok((u as u16).to_be_bytes().to_vec())
    }else {
        Ok(vec![u as u8])
    };
}

pub fn build(
    path: &str,
    ast: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants
) -> Result<(), ()> {
    let mut bytes = vec![];

    encode(ast, op_map, constants, &mut bytes)?;
    patch_checksum(&mut bytes);
    write(&bytes, path).map_err(|_| ())?;

    Ok(())
}

/*TODO where was it supposed to be used?
fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}
*/

pub fn encode(
    ast: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants,
    bytes: &mut Vec<u8>,
) -> Result<(), ()> {
    for child in ast.children() {
        match child.ty() {
            MacroCall => encode(
                child.first_of(MacroBody), op_map, constants, bytes)?,

            Instruction => encode_instruction(child, op_map, constants, bytes),

            // Fill empty space with zeroes to reach markers.
            Marker => {
                let marker_kind = child.first();

                let location = match marker_kind.ty() {
                    NamedMark | AnonMark => marker_kind.leaf().value().as_usize(),
                    Label => continue,
                    _ => bug!("Invalid Marker type."),
                };

                let diff = location - bytes.len();
                let mut fill = vec![0; diff];

                bytes.append(&mut fill);
            }

            Identifier => {
                if let ConstExpr::Value(v) = constants.get(child.value().as_str()).unwrap() {
                    match v {
                        //TODO make sure that validation works for values
                        Value::Usize(u) => {
                            let mut b = usize_to_bytes(*u).unwrap();
                            bytes.append(&mut b);
                        }            

                        //TODO make sure that validation works for values
                        Value::Str(s) => {
                            let mut b = str_to_bytes(s).unwrap();
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
                child.lit_to_bytes().map_or_else(|_| {
                    bug!("Could not read literal.");
                }, |mut b| {
                    bytes.append(&mut b);
                });
            }

            Directive => {
                if child.first().ty() == Include {
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

    Ok(())
}

fn encode_instruction(
    token: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants,
    bytes: &mut Vec<u8>,
) {
    let opcode = op_map.get(token);
    let mut op_bytes = vec![];

    if opcode.cb {
        op_bytes.push(0xCB);
    }

    op_bytes.push(opcode.code);

    if op_bytes.len() != opcode.len as usize {
        let children = token.children();

        if let Some(args) = children.get(1..) {
            for arg in args {
                let arg_x = arg.first();

                match arg_x.ty() {
                    Lit => {
                        arg_x.lit_to_bytes().map_or_else(|_| {
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
                                        //TODO make sure that validation works for values
                                        Value::Usize(u) => {
                                            let mut b = usize_to_bytes(*u).unwrap();
                                            op_bytes.append(&mut b);
                                        }            

                                        //TODO make sure that validation works for values
                                        Value::Str(s) => {
                                            let mut b = str_to_bytes(s).unwrap();
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

/// Patches header checksum into the rom.
fn patch_checksum(bytes: &mut [u8]) {
    let mut x = 0u8;

    for i in 0x0134..=0x014C {
        x = x.wrapping_sub(bytes[i]).wrapping_sub(1);
    }

    //TODO handle err: bytes too smol
    bytes[0x014D] = x;
}

fn write(bytes: &[u8], path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(bytes)?;

    Ok(())
}
