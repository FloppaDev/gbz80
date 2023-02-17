
use std::fs;
use std::fs::File;
use std::io::Read;

use crate::{
    parse::lex::TokenType::{ self, * },
    program,
    write::{ instructions, ops::Arg },
};

const FILE: &str = "build/tmp.gb";

/// Attempts to run the assembler and then print the disassembly.
#[test]
fn disasm() {
    // Run the assembler.
    fs::create_dir_all("build").unwrap();
    program::run().unwrap(); 

    // Read the binary output.
    let mut file = File::open(FILE).unwrap();
    let metadata = fs::metadata(FILE).unwrap();
    let mut bytes = vec![0; metadata.len() as usize];
    file.read_exact(&mut bytes).unwrap();

    // Entry point.
    let mut pc = 0x100;
    let mut instructions = vec![];

    for b in 0..bytes.len() {
        instructions.push(None);
    }

    decode(&mut pc, &bytes, &mut instructions, vec![]);
    fmt(&bytes, &instructions);
}

/// Attempts to decode instructions by reading jumps to determine the program's flow. 
/// Otherwise data could be interpreted as code and mess with the whole disassembly.
/// It does not interpret the instructions and thus can only read jumps to immediate 
/// values. Code that is only ever accessed through other means will be seen as data.
fn decode<'a>(
    pc: &mut usize, 
    bytes: &'a [u8], 
    instructions: &mut [Option<Instruction<'a>>], 
    jumps: Vec<usize>,
) {
    if jumps.contains(*pc) {
        return;
    }

    for i in 0..TokenType::COUNT {
        let ty = TokenType::at(i);
        if let Some((cb, list)) = instructions::get_instruction_info(ty) {
            let mut code_len = 1;

            if cb {
                if bytes[*pc] == 0xCB {
                    *pc += 1;
                    code_len = 2;
                }

                else {
                    continue;
                }
            }

            for (len, code, args) in list {
                if code == bytes[*pc] {
                    match ty {
                        Jp|Jr => branch(ty, bytes, instructions, pc, jumps.clone()),
                        _ => write_and_move(ty, instructions, pc)
                    }
                }
            }
        }

        else {
            *pc += 1;
        }
    }
}

/// Branches from a jump instruction to decode at the jump's position then resumes 
/// at the previous position.
fn branch<'a>(
    jump_ty: TokenType, 
    bytes: &[u8],
    instructions: &mut Vec<Option<Instruction<'a>>>, 
    pc: &mut usize, 
    mut jumps: Vec<usize>,
) {
    jumps.push(*pc);
    let mut jump_pc = jump(jump_ty, bytes, *pc);
    write_and_move(jump_ty, bytes, instructions, pc);
    decode(&mut jump_pc, bytes, instructions, jumps);
}

/// Returns PC's value after a jump.
fn jump(jump_ty: TokenType, bytes: &[u8], pc: usize) -> Option<usize> {
    Some(match jump_ty {
        Jp => u16_from_le(bytes.get((pc + 1)..(pc + 2))?) as usize,
        Jr => ((pc as i8) + (bytes.get(pc + 1)? as i8) as usize),
        _ => bug!("Unexpected jump type.")
    })
}

/// Writes the decoded instruction and moves PC.
fn write_and_move<'a>(
    ty: TokenType,
    bytes: &[u8],
    instructions: &mut Vec<Option<Instruction<'a>>>, 
    pc: &mut usize, 
) {
    let args = todo!();
    let arg_bytes = todo!();
    let instruction = Instruction::new(ty, args, bytes[*pc], arg_bytes);
    instructions[*pc] = Some(instruction); 
    //TODO move pc
}

/// Prints all decoded instructions and data bytes.
fn fmt<'a>(bytes: &[u8], instructions: &[Option<Instruction<'a>>]) {
    let mut data = 0;

    for b in 0..bytes.len() {
        let mut n = "";

        // Group up to 10 data bytes per line.
        if data >= 10 { 
            n = "\n";
            data = 0;
        }

        print!("{n}{b:X} ");

        if let Some(instruction) = &instructions[b] {
            if data > 0 {
                n = "\n";
                data = 0;
            }

            else {
                n = "";
            }

            // Print instruction on a new line.
            print!("{n}{instruction}");
        }

        else {
            data += 1;
        }
    }
}

struct Instruction<'a> {
    ty: TokenType,
    args: Vec<Arg>,
    code: &'a [u8],
    arg_bytes: &'a [u8],
}

impl<'a> Instruction<'a> {

    fn new(ty: TokenType, args: Vec<Arg>, code: &'a [u8], arg_bytes: &'a [u8]) -> Self {
        Self{ ty, args, code, arg_bytes }
    }

}

impl<'a> std::fmt::Display for Instruction<'a> {
    /// Formats an instruction as it would have been written in assembly.
    fn fmt(&self, f: ()) -> () {
        //TODO instr arg arg
        todo!()
    }
}
