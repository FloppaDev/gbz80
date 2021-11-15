use opcodes::{Instruction, Op};
use ast::{Token, TokenType::{self, *}};
use crate::abort;
use std::collections::HashMap;

//TODO not finnished
/// Map intructions in source to those in the opcodes module.
pub fn instruction_len<'a>(
    ast: &Token,
    instructions: &Vec<Instruction>
) -> HashMap<&'a Token, &'a Op> {
    let mut hashmap = HashMap::new();

    for token in &ast.children {

        let ty = token.children[0].ty;
        let child_count = token.children.len();
        let arg_count = if child_count == 1 { 0 }else{ child_count - 1 };

        let mut instruction = None;

        for inst in instructions {
            if inst.ty == ty {
                instruction = Some(inst);
            }
        }

        if instruction.is_none() {
            let e = format!("Instruction '{:?}' not found.", ty);
            abort(&e);
        }

        let instruction = instruction.unwrap();

        if arg_count == 0 {
            for op in &instruction.ops {
                if op.args.len() == 0 {

                }
            }
        }else{
            let mut args = vec![];

            for (i, arg) in token.children[2..].iter().enumerate() {
                match arg.children[0].ty {
                    REGISTER => args.push(arg.children[0].children[0].ty),
                    FLAG => args.push(arg.children[0].children[0].ty),
                    LIT => {
                        if i == 0 {
                            match arg.children[0].children[0].ty {
                                BIT|RES|RST|SET => {
                                    match arg.value.as_str() {
                                        "0" => args.push(B0),
                                        "1" => args.push(B1),
                                        "2" => args.push(B2),
                                        "3" => args.push(B3),
                                        "4" => args.push(B4),
                                        "5" => args.push(B5),
                                        "6" => args.push(B6),
                                        "7" => args.push(B7),
                                        _ => {
                                            let e = format!("Unrecognized bit number {}.", &arg.value);
                                            abort(&e);
                                        }
                                    }
                                }
                                _ => args.push(LIT),
                            }
                        }else {
                            args.push(LIT);
                        }
                    }
                    IDENTIFIER => args.push(LIT),
                    AT => {
                        args.push(AT0);

                        match arg.children[0].ty {
                            REGISTER => args.push(arg.children[0].children[0].ty),
                            LIT => args.push(LIT),
                            PLUS => {
                                args.push(arg.children[0].children[0].ty);
                                args.push(PLUS);
                                args.push(arg.children[0].children[1].ty);
                            }
                            _ => {
                                let e = format!(    "Token of type {:?} not expected in adress \
                                                    for instruction {:?}",
                                                    arg.children[0].ty,
                                                    ty);
                                abort(&e);
                            }
                        }

                        args.push(AT1);
                    }
                    PLUS => {
                        args.push(arg.children[0].children[0].ty);
                        args.push(PLUS);
                        args.push(arg.children[0].children[1].ty);
                    }
                    _ => {}
                }
            }

            //TODO
        }
    }

    hashmap
}

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(ast: Token, instructions: Vec<Instruction>) -> Vec<u8> {
    todo!();
}
