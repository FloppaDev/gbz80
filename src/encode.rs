use opcodes::Instruction;
use ast::{Token, TokenType::{self, *}};
use crate::abort;

fn token_of_type(ty: TokenType) -> Token {
    let mut token = Token::root();
    token.ty = ty;
    token
}

pub fn instruction_len(token: &Token, instructions: &Vec<Instruction>) -> usize {
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
                return op.bytes as usize;
            }
        }
    }else{
        let mut args = vec![];

        for (i, arg) in token.children[2..].iter().enumerate() {
            match arg.children[0].ty {
                REGISTER => args.push(arg.children[0].clone()),
                FLAG => args.push(arg.children[0].clone()),
                LIT => {
                    if i == 0 {
                        match ty {
                            BIT|RES|RST|SET => {
                                match arg.value.as_str() {
                                    "0" => args.push(token_of_type(B0)),
                                    "1" => args.push(token_of_type(B1)),
                                    "2" => args.push(token_of_type(B2)),
                                    "3" => args.push(token_of_type(B3)),
                                    "4" => args.push(token_of_type(B4)),
                                    "5" => args.push(token_of_type(B5)),
                                    "6" => args.push(token_of_type(B6)),
                                    "7" => args.push(token_of_type(B7)),
                                    _ => {
                                        let e = format!("Unrecognized bit number {}.", &arg.value);
                                        abort(&e);
                                    }
                                }
                            }
                            _ => {
                                args.push(arg.children[0].clone());
                            }
                        }
                    }else {
                        args.push(arg.children[0].clone());
                    }
                }
                AT => {
                    match arg.children[0].ty {
                        REGISTER => args.push(arg.children[0].children[0].clone()),
                        LIT => args.push(arg.children[0].clone()),
                        PLUS|MINUS => {
                            //TODO
                        }
                        _ => {
                            let e = format!(    "Token of type {:?} not expected in adress \
                                                for instruction {:?}",
                                                arg.children[0].ty,
                                                ty);
                            abort(&e);
                        }
                    }
                }
                PLUS|MINUS => {
                    //TODO
                }
                _ => {}
            }
        }
    }

    0
}

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(ast: Token, instructions: Vec<Instruction>) -> Vec<u8> {
    todo!();
}
