
use crate::{
    write::{ ops::OpMap, constants::Constants },
    token::read::TokenRef,
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

pub fn u16_to_bytes(u: u16) -> Vec<u8> {
    u.to_le_bytes().to_vec()
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

            // Fill empty space to reach markers.
            Marker => {
                let marker_kind = child.first();

                let location = match marker_kind.ty() {
                    NamedMark | AnonMark => marker_kind.leaf().value().as_u16().unwrap(),
                    Label => continue,
                    _ => bug!("Invalid Marker type."),
                };

                let diff = (location as usize) - bytes.len();
                let mut fill = vec![255u8; diff];

                bytes.append(&mut fill);
            }

            Identifier => {
                let ident = child.value().as_str().unwrap();
                let const_expr = constants.get(ident).unwrap();
                //TODO are markers sanitized?
                let mut b = const_expr.as_value().unwrap().as_bytes().unwrap();
                bytes.append(&mut b);
            }

            Lit => {
                let mut b = child.leaf().value().as_bytes().unwrap();
                bytes.append(&mut b);
            }

            Directive => {
                if child.first().ty() == Include {
                    let path = child.leaf().value().as_str().unwrap();

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
                let arg_x = arg.leaf();
                let mut arg_bytes = vec![];

                match arg_x.ty() {
                    LitBin|LitDec|LitHex|LitStr => {
                        let mut b = arg_x.leaf().value().as_bytes().unwrap();
                        arg_bytes.append(&mut b);
                    }

                    Identifier => {
                        let ident = arg_x.value().as_str().unwrap();
                        let const_expr = constants.get(ident).unwrap();
                        let mut b = const_expr.as_value().unwrap().as_bytes().unwrap();
                        arg_bytes.append(&mut b);
                    }

                    _ => {}
                }

                op_bytes.append(&mut arg_bytes);
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
