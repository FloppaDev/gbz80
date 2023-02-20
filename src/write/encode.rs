
//TODO fix: using a label on a instruction that takes one byte makes the other byte spill?

use crate::{
    write::{ ops::OpMap, constants::Constants },
    token::read::TokenRef,
    parse::lex::TokenType::*,
    error::asm::EncodeErr,
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
) -> Result<(), EncodeErr> {
    let mut bytes = vec![];

    encode(ast, op_map, constants, &mut bytes)?;
    patch_checksum(&mut bytes)?;
    write(&bytes, path)?;

    Ok(())
}

pub fn encode(
    ast: &TokenRef, 
    op_map: &OpMap, 
    constants: &Constants,
    bytes: &mut Vec<u8>,
) -> Result<(), EncodeErr> {
    for child in ast.children() {
        match child.ty() {
            MacroCall => encode(
                child.first_of(MacroBody), op_map, constants, bytes)?,

            Instruction => encode_instruction(child, op_map, constants, bytes),

            // Fill empty space to reach markers.
            Marker => {
                let marker_kind = child.first();

                let location = match marker_kind.ty() {
                    NamedMark | AnonMark => marker_kind.leaf().value().as_num().unwrap(),
                    Label => continue,
                    _ => bug!("Invalid Marker type."),
                };

                let diff = location - bytes.len();
                let mut fill = vec![255u8; diff];

                bytes.append(&mut fill);
            }

            Identifier => {
                let ident = child.value().as_str().unwrap();
                let const_expr = constants.get(ident).unwrap();
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

/// Patches header and ROM checksums into the binary.
fn patch_checksum(bytes: &mut [u8]) -> Result<(), EncodeErr> {
    if bytes.len() < 0x014F {
        return Err(EncodeErr::BadHeader);
    }

    let mut x = 0u8;

    for i in 0x0134..=0x014C {
        x = x.wrapping_sub(bytes[i]).wrapping_sub(1);
    }

    bytes[0x014D] = x;

    let mut rom_sum = 0u16;

    for byte in &*bytes {
        rom_sum = rom_sum.wrapping_add(*byte as u16);
    }

    let rom_sum_bytes = rom_sum.to_be_bytes();
    bytes[0x014E] = rom_sum_bytes[0];
    bytes[0x014F] = rom_sum_bytes[1];

    Ok(())
}

fn write(bytes: &[u8], path: &str) -> Result<(), EncodeErr> {
    let mut file = File::create(path).map_err(|_| EncodeErr::CreateFailed)?;
    file.write_all(bytes).map_err(|_| EncodeErr::WriteFailed)?;

    Ok(())
}
