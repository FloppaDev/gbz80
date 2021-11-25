use crate::opcodes::{InstructionDef, Op};
use crate::ast::{Token, TokenType::{self, *}};
use crate::abort;
use std::collections::HashMap;
use crate::utils;

/// Map intructions in source to those in the opcodes module.
pub fn instruction_ops<'a>(
    ast: &Token,
    instructions: &[InstructionDef],
) -> HashMap<&'a Token, &'a Op> {
    #[cfg(feature = "debug")] {
        utils::debug_title("Reading instructions");
    }

    let mut hashmap = HashMap::new();

    // For all instructions:
    // Format arguments in the same way as in the opcodes module.
    // Compare the arguments to find the correct Op from opcodes.
    // Note: When the first argument is A, it is optionnal.

    fn walk<'a>(
        root: &Token,
        instructions: &[InstructionDef], 
        mut hashmap: &mut HashMap<&'a Token, &'a Op>
    ) {
        for token in &root.children {
            match token.ty {
                Instruction => {}
                MacroCall => {
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
                    Register => args.push(arg.children[0].children[0].ty),
                    Flag => args.push(arg.children[0].children[0].ty),
                    Lit => {
                        if i == 0 {
                            match token.children[0].children[0].ty {
                                Bit|Res|Rst|Set => {
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
                                _ => args.push(Lit),
                            }
                        }else {
                            args.push(Lit);
                        }
                    }
                    Identifier => args.push(Lit),
                    At => {
                        let at = &arg.children[0];
                        args.push(At0);

                        match at.children[0].ty {
                            Register => args.push(at.children[0].children[0].ty),
                            Lit|Identifier => args.push(Lit),
                            Plus => {
                                args.push(at.children[0].children[0].ty);
                                args.push(Plus);
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

                        args.push(At1);
                    }
                    Plus => {
                        args.push(arg.children[0].children[0].ty);
                        args.push(Plus);
                        args.push(arg.children[0].children[1].ty);
                    }
                    _ => {}
                }
            }

            let arg_count = args.len();
            let mut instr_op = None;

            'op_loop: for op in &instruction.ops {
                if arg_count == op.args.len() {
                    #[allow(clippy::branches_sharing_code)]
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

            #[cfg(feature = "debug")] {
                let mut arg_str = String::new();

                if let Some(instr_op) = instr_op {
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
                }else {
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

                    //err TODO
                }

                token.debug();
            }
        }
    }

    walk(ast, instructions, &mut hashmap);

    hashmap
}

pub fn get_markers<'a>(
    ast: &Token,
    ops_map: &HashMap<&'a Token, &'a Op>,
) -> HashMap<&'a Token, &'a usize> {
    let mut hashmap = HashMap::new();
    let mut offset = 0;

    //  Within expressions, markers and defines are both identifiers.
    //  The size of an identifier could be 1 or 2 bytes, or anything with strings.

    //  #def FOO 10                         ; size is 1
    //  #def BAR Label1 + 5                 ; size is the size of the identifier
    //  #def MOO "uwu~"                     ; size is 4
    //  #def BAZ Label1 + Label2            ; size is the size of the greatest operand
    //  #def ABC BAZ + FOO                  ; size of BAZ
    //  #def NO1 NO2;                       ; /!\ circular dependency
    //  #def NO2 NO1                        ; /!\ circular dependency

    //  add a                   ; Instruction size: 1 byte
    //  &01                     ; 1 byte
    //  &2938                   ; 2 bytes
    //  MOO                     ; 4 (no \0)
    //  Label1:                 ; @8
    //  10                      ; 1
    //  1000                    ; 2
    //  FOO                     ; 1 
    //  "Hello"                 ; 5
    //  BAR                     ; 1? 2?
    //  :Label2                 ; @?
    //  BAR                     ; 1? 2?
    //  BAR                     ; 1? 2?
    //  &00FF:Marker            ; @256

    //  The first thing to do is to determine which identifiers are markers/labels because
    //      their size will always be 2 bytes.
    //
    //  Create a dictionnary of all defines and check for undefined identifiers.
    //
    //  ABC depends on BAZ and FOO: try to calculate them before calculating ABC.
    //      There's a possiblity for circular dependencies.

    fn walk<'a>(
        ast: &Token,
        ops_map: &HashMap<&'a Token, &'a Op>,
        mut hashmap: &mut HashMap<&'a Token, &'a usize>,
        mut offset: &mut usize,
    ) {
        for token in &ast.children {
            match token.ty {
                MacroCall => walk(token, ops_map, hashmap, offset),
                Instruction => {

                }
                Lit => {

                }
                Identifier => {
                    
                }
                Marker => {

                }
                _ => {}
            }
        }
    }

    walk(ast, ops_map, &mut hashmap, &mut offset);

    hashmap
}

fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}

pub fn build(ast: Token, instructions: &[InstructionDef]) -> Vec<u8> {
    todo!();
}
