
// File generated automatically
//  - from <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
//  - by code in 'gen/instructions'
//
// Do no edit manually.
use crate::{
    write::ops::{OpCode, Arg, Constant::*},
    parse::lex::TokenType::*,
    token::{
        read::TokenRef,
    },
};

pub fn find(instruction: &TokenRef) -> Option<OpCode> {
    assert_eq!(instruction.ty(), Instruction);

    let instr_ty = instruction.get(0).get(0).ty();

    match instr_ty {
            Adc => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x88, vec![Arg::Token(A), Arg::Token(B)]),
                    (1, 0x89, vec![Arg::Token(A), Arg::Token(C)]),
                    (1, 0x8A, vec![Arg::Token(A), Arg::Token(D)]),
                    (1, 0x8B, vec![Arg::Token(A), Arg::Token(E)]),
                    (1, 0x8C, vec![Arg::Token(A), Arg::Token(H)]),
                    (1, 0x8D, vec![Arg::Token(A), Arg::Token(L)]),
                    (1, 0x8E, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x8F, vec![Arg::Token(A), Arg::Token(A)]),
                    (2, 0xCE, vec![Arg::Token(A), Arg::Const(Byte)]),
                ])
            }

            Add => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x09, vec![Arg::Token(Hl), Arg::Token(Bc)]),
                    (1, 0x19, vec![Arg::Token(Hl), Arg::Token(De)]),
                    (1, 0x29, vec![Arg::Token(Hl), Arg::Token(Hl)]),
                    (1, 0x39, vec![Arg::Token(Hl), Arg::Token(Sp)]),
                    (1, 0x80, vec![Arg::Token(A), Arg::Token(B)]),
                    (1, 0x81, vec![Arg::Token(A), Arg::Token(C)]),
                    (1, 0x82, vec![Arg::Token(A), Arg::Token(D)]),
                    (1, 0x83, vec![Arg::Token(A), Arg::Token(E)]),
                    (1, 0x84, vec![Arg::Token(A), Arg::Token(H)]),
                    (1, 0x85, vec![Arg::Token(A), Arg::Token(L)]),
                    (1, 0x86, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x87, vec![Arg::Token(A), Arg::Token(A)]),
                    (2, 0xC6, vec![Arg::Token(A), Arg::Const(Byte)]),
                    (2, 0xE8, vec![Arg::Token(Sp), Arg::Const(Byte)]),
                ])
            }

            And => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xA0, vec![Arg::Token(B)]),
                    (1, 0xA1, vec![Arg::Token(C)]),
                    (1, 0xA2, vec![Arg::Token(D)]),
                    (1, 0xA3, vec![Arg::Token(E)]),
                    (1, 0xA4, vec![Arg::Token(H)]),
                    (1, 0xA5, vec![Arg::Token(L)]),
                    (1, 0xA6, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0xA7, vec![Arg::Token(A)]),
                    (2, 0xE6, vec![Arg::Const(Byte)]),
                ])
            }

            Call => {
                OpCode::get_opcode(instruction, false, vec![
                    (3, 0xC4, vec![Arg::Token(FlagNz), Arg::Const(Word)]),
                    (3, 0xCC, vec![Arg::Token(FlagZ), Arg::Const(Word)]),
                    (3, 0xCD, vec![Arg::Const(Word)]),
                    (3, 0xD4, vec![Arg::Token(FlagNc), Arg::Const(Word)]),
                    (3, 0xDC, vec![Arg::Token(FlagC), Arg::Const(Word)]),
                ])
            }

            Ccf => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x3F, vec![]),
                ])
            }

            Cp => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xB8, vec![Arg::Token(B)]),
                    (1, 0xB9, vec![Arg::Token(C)]),
                    (1, 0xBA, vec![Arg::Token(D)]),
                    (1, 0xBB, vec![Arg::Token(E)]),
                    (1, 0xBC, vec![Arg::Token(H)]),
                    (1, 0xBD, vec![Arg::Token(L)]),
                    (1, 0xBE, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0xBF, vec![Arg::Token(A)]),
                    (2, 0xFE, vec![Arg::Const(Byte)]),
                ])
            }

            Cpl => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x2F, vec![]),
                ])
            }

            Daa => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x27, vec![]),
                ])
            }

            Dec => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x05, vec![Arg::Token(B)]),
                    (1, 0x0B, vec![Arg::Token(Bc)]),
                    (1, 0x0D, vec![Arg::Token(C)]),
                    (1, 0x15, vec![Arg::Token(D)]),
                    (1, 0x1B, vec![Arg::Token(De)]),
                    (1, 0x1D, vec![Arg::Token(E)]),
                    (1, 0x25, vec![Arg::Token(H)]),
                    (1, 0x2B, vec![Arg::Token(Hl)]),
                    (1, 0x2D, vec![Arg::Token(L)]),
                    (1, 0x35, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x3B, vec![Arg::Token(Sp)]),
                    (1, 0x3D, vec![Arg::Token(A)]),
                ])
            }

            Di => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xF3, vec![]),
                ])
            }

            Ei => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xFB, vec![]),
                ])
            }

            Halt => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x76, vec![]),
                ])
            }

            Inc => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x03, vec![Arg::Token(Bc)]),
                    (1, 0x04, vec![Arg::Token(B)]),
                    (1, 0x0C, vec![Arg::Token(C)]),
                    (1, 0x13, vec![Arg::Token(De)]),
                    (1, 0x14, vec![Arg::Token(D)]),
                    (1, 0x1C, vec![Arg::Token(E)]),
                    (1, 0x23, vec![Arg::Token(Hl)]),
                    (1, 0x24, vec![Arg::Token(H)]),
                    (1, 0x2C, vec![Arg::Token(L)]),
                    (1, 0x33, vec![Arg::Token(Sp)]),
                    (1, 0x34, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x3C, vec![Arg::Token(A)]),
                ])
            }

            Jp => {
                OpCode::get_opcode(instruction, false, vec![
                    (3, 0xC2, vec![Arg::Token(FlagNz), Arg::Const(Word)]),
                    (3, 0xC3, vec![Arg::Const(Word)]),
                    (3, 0xCA, vec![Arg::Token(FlagZ), Arg::Const(Word)]),
                    (3, 0xD2, vec![Arg::Token(FlagNc), Arg::Const(Word)]),
                    (3, 0xDA, vec![Arg::Token(FlagC), Arg::Const(Word)]),
                    (1, 0xE9, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                ])
            }

            Jr => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0x18, vec![Arg::Const(Byte)]),
                    (2, 0x20, vec![Arg::Token(FlagNz), Arg::Const(Byte)]),
                    (2, 0x28, vec![Arg::Token(FlagZ), Arg::Const(Byte)]),
                    (2, 0x30, vec![Arg::Token(FlagNc), Arg::Const(Byte)]),
                    (2, 0x38, vec![Arg::Token(FlagC), Arg::Const(Byte)]),
                ])
            }

            Ld => {
                OpCode::get_opcode(instruction, false, vec![
                    (3, 0x01, vec![Arg::Token(Bc), Arg::Const(Word)]),
                    (1, 0x02, vec![Arg::At(Box::new(Arg::Token(Bc))), Arg::Token(A)]),
                    (2, 0x06, vec![Arg::Token(B), Arg::Const(Byte)]),
                    (3, 0x08, vec![Arg::At(Box::new(Arg::Const(Word))), Arg::Token(Sp)]),
                    (1, 0x0A, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Bc)))]),
                    (2, 0x0E, vec![Arg::Token(C), Arg::Const(Byte)]),
                    (3, 0x11, vec![Arg::Token(De), Arg::Const(Word)]),
                    (1, 0x12, vec![Arg::At(Box::new(Arg::Token(De))), Arg::Token(A)]),
                    (2, 0x16, vec![Arg::Token(D), Arg::Const(Byte)]),
                    (1, 0x1A, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(De)))]),
                    (2, 0x1E, vec![Arg::Token(E), Arg::Const(Byte)]),
                    (3, 0x21, vec![Arg::Token(Hl), Arg::Const(Word)]),
                    (2, 0x26, vec![Arg::Token(H), Arg::Const(Byte)]),
                    (2, 0x2E, vec![Arg::Token(L), Arg::Const(Byte)]),
                    (3, 0x31, vec![Arg::Token(Sp), Arg::Const(Word)]),
                    (2, 0x36, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Const(Byte)]),
                    (2, 0x3E, vec![Arg::Token(A), Arg::Const(Byte)]),
                    (1, 0x40, vec![Arg::Token(B), Arg::Token(B)]),
                    (1, 0x41, vec![Arg::Token(B), Arg::Token(C)]),
                    (1, 0x42, vec![Arg::Token(B), Arg::Token(D)]),
                    (1, 0x43, vec![Arg::Token(B), Arg::Token(E)]),
                    (1, 0x44, vec![Arg::Token(B), Arg::Token(H)]),
                    (1, 0x45, vec![Arg::Token(B), Arg::Token(L)]),
                    (1, 0x46, vec![Arg::Token(B), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x47, vec![Arg::Token(B), Arg::Token(A)]),
                    (1, 0x48, vec![Arg::Token(C), Arg::Token(B)]),
                    (1, 0x49, vec![Arg::Token(C), Arg::Token(C)]),
                    (1, 0x4A, vec![Arg::Token(C), Arg::Token(D)]),
                    (1, 0x4B, vec![Arg::Token(C), Arg::Token(E)]),
                    (1, 0x4C, vec![Arg::Token(C), Arg::Token(H)]),
                    (1, 0x4D, vec![Arg::Token(C), Arg::Token(L)]),
                    (1, 0x4E, vec![Arg::Token(C), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x4F, vec![Arg::Token(C), Arg::Token(A)]),
                    (1, 0x50, vec![Arg::Token(D), Arg::Token(B)]),
                    (1, 0x51, vec![Arg::Token(D), Arg::Token(C)]),
                    (1, 0x52, vec![Arg::Token(D), Arg::Token(D)]),
                    (1, 0x53, vec![Arg::Token(D), Arg::Token(E)]),
                    (1, 0x54, vec![Arg::Token(D), Arg::Token(H)]),
                    (1, 0x55, vec![Arg::Token(D), Arg::Token(L)]),
                    (1, 0x56, vec![Arg::Token(D), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x57, vec![Arg::Token(D), Arg::Token(A)]),
                    (1, 0x58, vec![Arg::Token(E), Arg::Token(B)]),
                    (1, 0x59, vec![Arg::Token(E), Arg::Token(C)]),
                    (1, 0x5A, vec![Arg::Token(E), Arg::Token(D)]),
                    (1, 0x5B, vec![Arg::Token(E), Arg::Token(E)]),
                    (1, 0x5C, vec![Arg::Token(E), Arg::Token(H)]),
                    (1, 0x5D, vec![Arg::Token(E), Arg::Token(L)]),
                    (1, 0x5E, vec![Arg::Token(E), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x5F, vec![Arg::Token(E), Arg::Token(A)]),
                    (1, 0x60, vec![Arg::Token(H), Arg::Token(B)]),
                    (1, 0x61, vec![Arg::Token(H), Arg::Token(C)]),
                    (1, 0x62, vec![Arg::Token(H), Arg::Token(D)]),
                    (1, 0x63, vec![Arg::Token(H), Arg::Token(E)]),
                    (1, 0x64, vec![Arg::Token(H), Arg::Token(H)]),
                    (1, 0x65, vec![Arg::Token(H), Arg::Token(L)]),
                    (1, 0x66, vec![Arg::Token(H), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x67, vec![Arg::Token(H), Arg::Token(A)]),
                    (1, 0x68, vec![Arg::Token(L), Arg::Token(B)]),
                    (1, 0x69, vec![Arg::Token(L), Arg::Token(C)]),
                    (1, 0x6A, vec![Arg::Token(L), Arg::Token(D)]),
                    (1, 0x6B, vec![Arg::Token(L), Arg::Token(E)]),
                    (1, 0x6C, vec![Arg::Token(L), Arg::Token(H)]),
                    (1, 0x6D, vec![Arg::Token(L), Arg::Token(L)]),
                    (1, 0x6E, vec![Arg::Token(L), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x6F, vec![Arg::Token(L), Arg::Token(A)]),
                    (1, 0x70, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(B)]),
                    (1, 0x71, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(C)]),
                    (1, 0x72, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(D)]),
                    (1, 0x73, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(E)]),
                    (1, 0x74, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(H)]),
                    (1, 0x75, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(L)]),
                    (1, 0x77, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(A)]),
                    (1, 0x78, vec![Arg::Token(A), Arg::Token(B)]),
                    (1, 0x79, vec![Arg::Token(A), Arg::Token(C)]),
                    (1, 0x7A, vec![Arg::Token(A), Arg::Token(D)]),
                    (1, 0x7B, vec![Arg::Token(A), Arg::Token(E)]),
                    (1, 0x7C, vec![Arg::Token(A), Arg::Token(H)]),
                    (1, 0x7D, vec![Arg::Token(A), Arg::Token(L)]),
                    (1, 0x7E, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x7F, vec![Arg::Token(A), Arg::Token(A)]),
                    (2, 0xE2, vec![Arg::At(Box::new(Arg::Token(C))), Arg::Token(A)]),
                    (3, 0xEA, vec![Arg::At(Box::new(Arg::Const(Word))), Arg::Token(A)]),
                    (2, 0xF2, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(C)))]),
                    (1, 0xF9, vec![Arg::Token(Sp), Arg::Token(Hl)]),
                    (3, 0xFA, vec![Arg::Token(A), Arg::At(Box::new(Arg::Const(Word)))]),
                ])
            }

            Ldd => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x32, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(A)]),
                    (1, 0x3A, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Hl)))]),
                ])
            }

            Ldh => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0xE0, vec![Arg::At(Box::new(Arg::Const(Byte))), Arg::Token(A)]),
                    (2, 0xF0, vec![Arg::Token(A), Arg::At(Box::new(Arg::Const(Byte)))]),
                ])
            }

            Ldhl => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0xF8, vec![Arg::Token(Hl), Arg::Token(Sp)]),
                ])
            }

            Ldi => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x22, vec![Arg::At(Box::new(Arg::Token(Hl))), Arg::Token(A)]),
                    (1, 0x2A, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Hl)))]),
                ])
            }

            Nop => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x00, vec![]),
                ])
            }

            Or => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xB0, vec![Arg::Token(B)]),
                    (1, 0xB1, vec![Arg::Token(C)]),
                    (1, 0xB2, vec![Arg::Token(D)]),
                    (1, 0xB3, vec![Arg::Token(E)]),
                    (1, 0xB4, vec![Arg::Token(H)]),
                    (1, 0xB5, vec![Arg::Token(L)]),
                    (1, 0xB6, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0xB7, vec![Arg::Token(A)]),
                    (2, 0xF6, vec![Arg::Const(Byte)]),
                ])
            }

            Pop => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC1, vec![Arg::Token(Bc)]),
                    (1, 0xD1, vec![Arg::Token(De)]),
                    (1, 0xE1, vec![Arg::Token(Hl)]),
                    (1, 0xF1, vec![Arg::Token(Af)]),
                ])
            }

            Push => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC5, vec![Arg::Token(Bc)]),
                    (1, 0xD5, vec![Arg::Token(De)]),
                    (1, 0xE5, vec![Arg::Token(Hl)]),
                    (1, 0xF5, vec![Arg::Token(Af)]),
                ])
            }

            Ret => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC0, vec![Arg::Token(FlagNz)]),
                    (1, 0xC8, vec![Arg::Token(FlagZ)]),
                    (1, 0xC9, vec![]),
                    (1, 0xD0, vec![Arg::Token(FlagNc)]),
                    (1, 0xD8, vec![Arg::Token(FlagC)]),
                ])
            }

            Reti => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xD9, vec![]),
                ])
            }

            Rla => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x17, vec![]),
                ])
            }

            Rlca => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x07, vec![]),
                ])
            }

            Rra => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x1F, vec![]),
                ])
            }

            Rrca => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x0F, vec![]),
                ])
            }

            Rst => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC7, vec![Arg::Const(Word)]),
                    (1, 0xCF, vec![Arg::Const(Word)]),
                    (1, 0xD7, vec![Arg::Const(Word)]),
                    (1, 0xDF, vec![Arg::Const(Word)]),
                    (1, 0xE7, vec![Arg::Const(Word)]),
                    (1, 0xEF, vec![Arg::Const(Word)]),
                    (1, 0xF7, vec![Arg::Const(Word)]),
                    (1, 0xFF, vec![Arg::Const(Word)]),
                ])
            }

            Sbc => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x98, vec![Arg::Token(A), Arg::Token(B)]),
                    (1, 0x99, vec![Arg::Token(A), Arg::Token(C)]),
                    (1, 0x9A, vec![Arg::Token(A), Arg::Token(D)]),
                    (1, 0x9B, vec![Arg::Token(A), Arg::Token(E)]),
                    (1, 0x9C, vec![Arg::Token(A), Arg::Token(H)]),
                    (1, 0x9D, vec![Arg::Token(A), Arg::Token(L)]),
                    (1, 0x9E, vec![Arg::Token(A), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x9F, vec![Arg::Token(A), Arg::Token(A)]),
                    (2, 0xDE, vec![Arg::Token(A), Arg::Const(Byte)]),
                ])
            }

            Scf => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x37, vec![]),
                ])
            }

            Stop => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0x10, vec![Arg::Const(Byte)]),
                ])
            }

            Sub => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x90, vec![Arg::Token(B)]),
                    (1, 0x91, vec![Arg::Token(C)]),
                    (1, 0x92, vec![Arg::Token(D)]),
                    (1, 0x93, vec![Arg::Token(E)]),
                    (1, 0x94, vec![Arg::Token(H)]),
                    (1, 0x95, vec![Arg::Token(L)]),
                    (1, 0x96, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0x97, vec![Arg::Token(A)]),
                    (2, 0xD6, vec![Arg::Const(Byte)]),
                ])
            }

            Xor => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xA8, vec![Arg::Token(B)]),
                    (1, 0xA9, vec![Arg::Token(C)]),
                    (1, 0xAA, vec![Arg::Token(D)]),
                    (1, 0xAB, vec![Arg::Token(E)]),
                    (1, 0xAC, vec![Arg::Token(H)]),
                    (1, 0xAD, vec![Arg::Token(L)]),
                    (1, 0xAE, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (1, 0xAF, vec![Arg::Token(A)]),
                    (2, 0xEE, vec![Arg::Const(Byte)]),
                ])
            }

            // CB instructions

            Bit => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x40, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x41, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x42, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x43, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x44, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x45, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x46, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x47, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x48, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x49, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x4A, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x4B, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x4C, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x4D, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x4E, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x4F, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x50, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x51, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x52, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x53, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x54, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x55, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x56, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x57, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x58, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x59, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x5A, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x5B, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x5C, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x5D, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x5E, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x5F, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x60, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x61, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x62, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x63, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x64, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x65, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x66, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x67, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x68, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x69, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x6A, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x6B, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x6C, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x6D, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x6E, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x6F, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x70, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x71, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x72, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x73, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x74, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x75, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x76, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x77, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x78, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x79, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x7A, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x7B, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x7C, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x7D, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x7E, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x7F, vec![Arg::Const(Byte), Arg::Token(A)]),
                ])
            }

            Res => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x80, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x81, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x82, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x83, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x84, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x85, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x86, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x87, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x88, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x89, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x8A, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x8B, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x8C, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x8D, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x8E, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x8F, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x90, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x91, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x92, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x93, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x94, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x95, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x96, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x97, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0x98, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0x99, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0x9A, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0x9B, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0x9C, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0x9D, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0x9E, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x9F, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xA0, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xA1, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xA2, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xA3, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xA4, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xA5, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xA6, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xA7, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xA8, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xA9, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xAA, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xAB, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xAC, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xAD, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xAE, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xAF, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xB0, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xB1, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xB2, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xB3, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xB4, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xB5, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xB6, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xB7, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xB8, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xB9, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xBA, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xBB, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xBC, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xBD, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xBE, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xBF, vec![Arg::Const(Byte), Arg::Token(A)]),
                ])
            }

            Rl => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x10, vec![Arg::Token(B)]),
                    (2, 0x11, vec![Arg::Token(C)]),
                    (2, 0x12, vec![Arg::Token(D)]),
                    (2, 0x13, vec![Arg::Token(E)]),
                    (2, 0x14, vec![Arg::Token(H)]),
                    (2, 0x15, vec![Arg::Token(L)]),
                    (2, 0x16, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x17, vec![Arg::Token(A)]),
                ])
            }

            Rlc => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x00, vec![Arg::Token(B)]),
                    (2, 0x01, vec![Arg::Token(C)]),
                    (2, 0x02, vec![Arg::Token(D)]),
                    (2, 0x03, vec![Arg::Token(E)]),
                    (2, 0x04, vec![Arg::Token(H)]),
                    (2, 0x05, vec![Arg::Token(L)]),
                    (2, 0x06, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x07, vec![Arg::Token(A)]),
                ])
            }

            Rr => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x18, vec![Arg::Token(B)]),
                    (2, 0x19, vec![Arg::Token(C)]),
                    (2, 0x1A, vec![Arg::Token(D)]),
                    (2, 0x1B, vec![Arg::Token(E)]),
                    (2, 0x1C, vec![Arg::Token(H)]),
                    (2, 0x1D, vec![Arg::Token(L)]),
                    (2, 0x1E, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x1F, vec![Arg::Token(A)]),
                ])
            }

            Rrc => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x08, vec![Arg::Token(B)]),
                    (2, 0x09, vec![Arg::Token(C)]),
                    (2, 0x0A, vec![Arg::Token(D)]),
                    (2, 0x0B, vec![Arg::Token(E)]),
                    (2, 0x0C, vec![Arg::Token(H)]),
                    (2, 0x0D, vec![Arg::Token(L)]),
                    (2, 0x0E, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x0F, vec![Arg::Token(A)]),
                ])
            }

            Set => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0xC0, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xC1, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xC2, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xC3, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xC4, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xC5, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xC6, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xC7, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xC8, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xC9, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xCA, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xCB, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xCC, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xCD, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xCE, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xCF, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xD0, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xD1, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xD2, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xD3, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xD4, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xD5, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xD6, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xD7, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xD8, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xD9, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xDA, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xDB, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xDC, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xDD, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xDE, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xDF, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xE0, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xE1, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xE2, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xE3, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xE4, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xE5, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xE6, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xE7, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xE8, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xE9, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xEA, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xEB, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xEC, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xED, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xEE, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xEF, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xF0, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xF1, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xF2, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xF3, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xF4, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xF5, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xF6, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xF7, vec![Arg::Const(Byte), Arg::Token(A)]),
                    (2, 0xF8, vec![Arg::Const(Byte), Arg::Token(B)]),
                    (2, 0xF9, vec![Arg::Const(Byte), Arg::Token(C)]),
                    (2, 0xFA, vec![Arg::Const(Byte), Arg::Token(D)]),
                    (2, 0xFB, vec![Arg::Const(Byte), Arg::Token(E)]),
                    (2, 0xFC, vec![Arg::Const(Byte), Arg::Token(H)]),
                    (2, 0xFD, vec![Arg::Const(Byte), Arg::Token(L)]),
                    (2, 0xFE, vec![Arg::Const(Byte), Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0xFF, vec![Arg::Const(Byte), Arg::Token(A)]),
                ])
            }

            Sla => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x20, vec![Arg::Token(B)]),
                    (2, 0x21, vec![Arg::Token(C)]),
                    (2, 0x22, vec![Arg::Token(D)]),
                    (2, 0x23, vec![Arg::Token(E)]),
                    (2, 0x24, vec![Arg::Token(H)]),
                    (2, 0x25, vec![Arg::Token(L)]),
                    (2, 0x26, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x27, vec![Arg::Token(A)]),
                ])
            }

            Sra => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x28, vec![Arg::Token(B)]),
                    (2, 0x29, vec![Arg::Token(C)]),
                    (2, 0x2A, vec![Arg::Token(D)]),
                    (2, 0x2B, vec![Arg::Token(E)]),
                    (2, 0x2C, vec![Arg::Token(H)]),
                    (2, 0x2D, vec![Arg::Token(L)]),
                    (2, 0x2E, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x2F, vec![Arg::Token(A)]),
                ])
            }

            Srl => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x38, vec![Arg::Token(B)]),
                    (2, 0x39, vec![Arg::Token(C)]),
                    (2, 0x3A, vec![Arg::Token(D)]),
                    (2, 0x3B, vec![Arg::Token(E)]),
                    (2, 0x3C, vec![Arg::Token(H)]),
                    (2, 0x3D, vec![Arg::Token(L)]),
                    (2, 0x3E, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x3F, vec![Arg::Token(A)]),
                ])
            }

            Swap => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x30, vec![Arg::Token(B)]),
                    (2, 0x31, vec![Arg::Token(C)]),
                    (2, 0x32, vec![Arg::Token(D)]),
                    (2, 0x33, vec![Arg::Token(E)]),
                    (2, 0x34, vec![Arg::Token(H)]),
                    (2, 0x35, vec![Arg::Token(L)]),
                    (2, 0x36, vec![Arg::At(Box::new(Arg::Token(Hl)))]),
                    (2, 0x37, vec![Arg::Token(A)]),
                ])
            }

            _ => bug!("Op not found"),
        }
}