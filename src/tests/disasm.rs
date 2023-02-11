
use std::fs;
use std::fs::File;
use std::io::Read;

use crate::{
    parse::lex::TokenType,
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

    // Instruction starts will be set to non-zero slices from the opcode to the arguments.
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
    jumps: Vec<usize>
) {
    //TODO keep track of jumps to avoid infinite loops

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
                        TokenType::Jp => {
                            //TODO write to ops and inc pc
                            //TODO recursive decode()
                        }

                        TokenType::Jr => {
                            //TODO write to ops and inc pc
                            //TODO recursive decode()
                        }

                        _ => {
                            //TODO write to ops and inc pc
                        }
                    }
                }
            }
        }

        else {
            *pc += 1;
        }
    }
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
            print!("{n}{}", instruction.fmt(&bytes[b..]));
        }

        else {
            data += 1;
        }
    }
}

struct Instruction<'a> {
    code: &'a [u8],
    args: Vec<Arg>,
}

impl<'a> Instruction<'a> {

    /// Formats an instruction as it would have been written in assembly.
    fn fmt(&self, bytes: &[u8]) -> String {
        //TODO instr arg arg
        todo!()
    }

}
