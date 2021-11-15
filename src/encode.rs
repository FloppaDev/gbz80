use opcodes::{Instruction, Op};
use ast::{Token, TokenType::{self, *}};
use crate::abort;
use std::collections::HashMap;
use utils;

/// Map intructions in source to those in the opcodes module.
pub fn instruction_ops<'a>(
    ast: &Token,
    instructions: &Vec<Instruction>
) -> HashMap<&'a Token, &'a Op> {
    #[cfg(debug)] {
        utils::debug_title("Reading instructions");
    }

    let mut hashmap = HashMap::new();

    fn walk<'a>(
        root: &Token,
        instructions: &Vec<Instruction>, 
        mut hashmap: &mut HashMap<&'a Token, &'a Op>
    ) {
        for token in &root.children {
            match token.ty {
                INSTRUCTION => {}
                MACRO_CALL => {
                    walk(token, instructions, hashmap);
                    continue;
                }
                _ => continue,
            }

            let ty = token.children[0].children[0].ty;
            let child_count = token.children.len();
            let arg_count = if child_count == 1 { 0 }else{ child_count - 1 };

            let mut instruction = None;

            for inst in instructions {
                if inst.ty == ty {
                    instruction = Some(inst);
                }
            }

            if instruction.is_none() { 
                //TODO err: not found...
                println!("{:?}: No instruction: {:?} (L{})\n", token.children[0].ty, ty, token.line);
                continue;
            }

            let instruction = instruction.unwrap();

            let mut args = vec![];

            for (i, arg) in token.children[1..].iter().enumerate() {
                match arg.children[0].ty {
                    REGISTER => args.push(arg.children[0].children[0].ty),
                    FLAG => args.push(arg.children[0].children[0].ty),
                    LIT => {
                        if i == 0 {
                            match token.children[0].children[0].ty {
                                BIT|RES|RST|SET => {
                                    match arg.children[0].children[0].value.as_str() {
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
                        let at = &arg.children[0];
                        args.push(AT0);

                        match at.children[0].ty {
                            REGISTER => args.push(at.children[0].children[0].ty),
                            LIT|IDENTIFIER => args.push(LIT),
                            PLUS => {
                                args.push(at.children[0].children[0].ty);
                                args.push(PLUS);
                                args.push(at.children[0].children[1].ty);
                            }
                            _ => {
                                let e = format!(    "Token of type {:?} not expected in adress \
                                                    for instruction {:?}. (L{})",
                                                    at.children[0].ty,
                                                    ty,
                                                    at.line);
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

            let arg_count = args.len();
            let mut instr_op = None;

            'op_loop: for op in &instruction.ops {
                if arg_count == op.args.len() {
                    if arg_count == 0 {
                        instr_op = Some(op);
                        break 'op_loop;
                    }else {
                        // This loop completes if all arguments match.
                        for (i, arg) in args.iter().enumerate() {
                            if *arg != op.args[i] {
                                continue 'op_loop;
                            }
                        }

                        instr_op = Some(op);
                        break 'op_loop;
                    }
                }else if arg_count == op.args.len() - 1 && op.args[0] == A {
                    // When the first agument is A, it is optionnal.
                    for (i, arg) in args.iter().enumerate() {
                        if *arg != op.args[i+1] {
                            continue 'op_loop;
                        }
                    }

                    instr_op = Some(op);
                    break 'op_loop;
                }
            }

            if let Some(instr_op) = instr_op {
                    let mut arg_str = String::new();
                    for instr_op_arg in &instr_op.args {
                        arg_str.push(' ');
                        let s = format!("{:?}", instr_op_arg);
                        arg_str.push_str(&s);
                    }

                    println!(
                        "    Instruction found \x1b[31m{:?}{}\x1b[0m",
                        instruction.ty,
                        arg_str
                    );

                    token.debug();
            }else {
                let mut arg_str = String::new();
                for arg in &args {
                    arg_str.push(' ');
                    let s = format!("{:?}", arg);
                    arg_str.push_str(&s);
                }

                println!(
                    "    \x1b[33mInstruction not found (L{}) \x1b[31m{:?}{}\x1b[0m",
                    token.line,
                    instruction.ty,
                    arg_str
                );

                token.debug();
                //err TODO
            }
        }
    }

    walk(ast, instructions, &mut hashmap);

    hashmap
}

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(ast: Token, instructions: Vec<Instruction>) -> Vec<u8> {
    todo!();
}
