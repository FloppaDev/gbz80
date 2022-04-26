
// File generated automatically
//  - from <https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html>
//  - by code in 'gen/instructions'
//
// Do no edit manually.
use crate::{
    write::ops::{OpCode, ty, imm, at, Constant::*},
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
                    (1, 0x88, vec![ty(A), ty(B)]),
                    (1, 0x89, vec![ty(A), ty(C)]),
                    (1, 0x8A, vec![ty(A), ty(D)]),
                    (1, 0x8B, vec![ty(A), ty(E)]),
                    (1, 0x8C, vec![ty(A), ty(H)]),
                    (1, 0x8D, vec![ty(A), ty(L)]),
                    (1, 0x8E, vec![ty(A), at(ty(Hl))]),
                    (1, 0x8F, vec![ty(A), ty(A)]),
                    (2, 0xCE, vec![ty(A), imm(Byte)]),
                ])
            }

            Add => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x09, vec![ty(Hl), ty(Bc)]),
                    (1, 0x19, vec![ty(Hl), ty(De)]),
                    (1, 0x29, vec![ty(Hl), ty(Hl)]),
                    (1, 0x39, vec![ty(Hl), ty(Sp)]),
                    (1, 0x80, vec![ty(A), ty(B)]),
                    (1, 0x81, vec![ty(A), ty(C)]),
                    (1, 0x82, vec![ty(A), ty(D)]),
                    (1, 0x83, vec![ty(A), ty(E)]),
                    (1, 0x84, vec![ty(A), ty(H)]),
                    (1, 0x85, vec![ty(A), ty(L)]),
                    (1, 0x86, vec![ty(A), at(ty(Hl))]),
                    (1, 0x87, vec![ty(A), ty(A)]),
                    (2, 0xC6, vec![ty(A), imm(Byte)]),
                    (2, 0xE8, vec![ty(Sp), imm(Byte)]),
                ])
            }

            And => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xA0, vec![ty(B)]),
                    (1, 0xA1, vec![ty(C)]),
                    (1, 0xA2, vec![ty(D)]),
                    (1, 0xA3, vec![ty(E)]),
                    (1, 0xA4, vec![ty(H)]),
                    (1, 0xA5, vec![ty(L)]),
                    (1, 0xA6, vec![at(ty(Hl))]),
                    (1, 0xA7, vec![ty(A)]),
                    (2, 0xE6, vec![imm(Byte)]),
                ])
            }

            Call => {
                OpCode::get_opcode(instruction, false, vec![
                    (3, 0xC4, vec![ty(FlagNz), imm(Word)]),
                    (3, 0xCC, vec![ty(FlagZ), imm(Word)]),
                    (3, 0xCD, vec![imm(Word)]),
                    (3, 0xD4, vec![ty(FlagNc), imm(Word)]),
                    (3, 0xDC, vec![ty(FlagC), imm(Word)]),
                ])
            }

            Ccf => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x3F, vec![]),
                ])
            }

            Cp => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xB8, vec![ty(B)]),
                    (1, 0xB9, vec![ty(C)]),
                    (1, 0xBA, vec![ty(D)]),
                    (1, 0xBB, vec![ty(E)]),
                    (1, 0xBC, vec![ty(H)]),
                    (1, 0xBD, vec![ty(L)]),
                    (1, 0xBE, vec![at(ty(Hl))]),
                    (1, 0xBF, vec![ty(A)]),
                    (2, 0xFE, vec![imm(Byte)]),
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
                    (1, 0x05, vec![ty(B)]),
                    (1, 0x0B, vec![ty(Bc)]),
                    (1, 0x0D, vec![ty(C)]),
                    (1, 0x15, vec![ty(D)]),
                    (1, 0x1B, vec![ty(De)]),
                    (1, 0x1D, vec![ty(E)]),
                    (1, 0x25, vec![ty(H)]),
                    (1, 0x2B, vec![ty(Hl)]),
                    (1, 0x2D, vec![ty(L)]),
                    (1, 0x35, vec![at(ty(Hl))]),
                    (1, 0x3B, vec![ty(Sp)]),
                    (1, 0x3D, vec![ty(A)]),
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
                    (1, 0x03, vec![ty(Bc)]),
                    (1, 0x04, vec![ty(B)]),
                    (1, 0x0C, vec![ty(C)]),
                    (1, 0x13, vec![ty(De)]),
                    (1, 0x14, vec![ty(D)]),
                    (1, 0x1C, vec![ty(E)]),
                    (1, 0x23, vec![ty(Hl)]),
                    (1, 0x24, vec![ty(H)]),
                    (1, 0x2C, vec![ty(L)]),
                    (1, 0x33, vec![ty(Sp)]),
                    (1, 0x34, vec![at(ty(Hl))]),
                    (1, 0x3C, vec![ty(A)]),
                ])
            }

            Jp => {
                OpCode::get_opcode(instruction, false, vec![
                    (3, 0xC2, vec![ty(FlagNz), imm(Word)]),
                    (3, 0xC3, vec![imm(Word)]),
                    (3, 0xCA, vec![ty(FlagZ), imm(Word)]),
                    (3, 0xD2, vec![ty(FlagNc), imm(Word)]),
                    (3, 0xDA, vec![ty(FlagC), imm(Word)]),
                    (1, 0xE9, vec![at(ty(Hl))]),
                ])
            }

            Jr => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0x18, vec![imm(Byte)]),
                    (2, 0x20, vec![ty(FlagNz), imm(Byte)]),
                    (2, 0x28, vec![ty(FlagZ), imm(Byte)]),
                    (2, 0x30, vec![ty(FlagNc), imm(Byte)]),
                    (2, 0x38, vec![ty(FlagC), imm(Byte)]),
                ])
            }

            Ld => {
                OpCode::get_opcode(instruction, false, vec![
                    (3, 0x01, vec![ty(Bc), imm(Word)]),
                    (1, 0x02, vec![at(ty(Bc)), ty(A)]),
                    (2, 0x06, vec![ty(B), imm(Byte)]),
                    (3, 0x08, vec![at(imm(Word)), ty(Sp)]),
                    (1, 0x0A, vec![ty(A), at(ty(Bc))]),
                    (2, 0x0E, vec![ty(C), imm(Byte)]),
                    (3, 0x11, vec![ty(De), imm(Word)]),
                    (1, 0x12, vec![at(ty(De)), ty(A)]),
                    (2, 0x16, vec![ty(D), imm(Byte)]),
                    (1, 0x1A, vec![ty(A), at(ty(De))]),
                    (2, 0x1E, vec![ty(E), imm(Byte)]),
                    (3, 0x21, vec![ty(Hl), imm(Word)]),
                    (2, 0x26, vec![ty(H), imm(Byte)]),
                    (2, 0x2E, vec![ty(L), imm(Byte)]),
                    (3, 0x31, vec![ty(Sp), imm(Word)]),
                    (2, 0x36, vec![at(ty(Hl)), imm(Byte)]),
                    (2, 0x3E, vec![ty(A), imm(Byte)]),
                    (1, 0x40, vec![ty(B), ty(B)]),
                    (1, 0x41, vec![ty(B), ty(C)]),
                    (1, 0x42, vec![ty(B), ty(D)]),
                    (1, 0x43, vec![ty(B), ty(E)]),
                    (1, 0x44, vec![ty(B), ty(H)]),
                    (1, 0x45, vec![ty(B), ty(L)]),
                    (1, 0x46, vec![ty(B), at(ty(Hl))]),
                    (1, 0x47, vec![ty(B), ty(A)]),
                    (1, 0x48, vec![ty(C), ty(B)]),
                    (1, 0x49, vec![ty(C), ty(C)]),
                    (1, 0x4A, vec![ty(C), ty(D)]),
                    (1, 0x4B, vec![ty(C), ty(E)]),
                    (1, 0x4C, vec![ty(C), ty(H)]),
                    (1, 0x4D, vec![ty(C), ty(L)]),
                    (1, 0x4E, vec![ty(C), at(ty(Hl))]),
                    (1, 0x4F, vec![ty(C), ty(A)]),
                    (1, 0x50, vec![ty(D), ty(B)]),
                    (1, 0x51, vec![ty(D), ty(C)]),
                    (1, 0x52, vec![ty(D), ty(D)]),
                    (1, 0x53, vec![ty(D), ty(E)]),
                    (1, 0x54, vec![ty(D), ty(H)]),
                    (1, 0x55, vec![ty(D), ty(L)]),
                    (1, 0x56, vec![ty(D), at(ty(Hl))]),
                    (1, 0x57, vec![ty(D), ty(A)]),
                    (1, 0x58, vec![ty(E), ty(B)]),
                    (1, 0x59, vec![ty(E), ty(C)]),
                    (1, 0x5A, vec![ty(E), ty(D)]),
                    (1, 0x5B, vec![ty(E), ty(E)]),
                    (1, 0x5C, vec![ty(E), ty(H)]),
                    (1, 0x5D, vec![ty(E), ty(L)]),
                    (1, 0x5E, vec![ty(E), at(ty(Hl))]),
                    (1, 0x5F, vec![ty(E), ty(A)]),
                    (1, 0x60, vec![ty(H), ty(B)]),
                    (1, 0x61, vec![ty(H), ty(C)]),
                    (1, 0x62, vec![ty(H), ty(D)]),
                    (1, 0x63, vec![ty(H), ty(E)]),
                    (1, 0x64, vec![ty(H), ty(H)]),
                    (1, 0x65, vec![ty(H), ty(L)]),
                    (1, 0x66, vec![ty(H), at(ty(Hl))]),
                    (1, 0x67, vec![ty(H), ty(A)]),
                    (1, 0x68, vec![ty(L), ty(B)]),
                    (1, 0x69, vec![ty(L), ty(C)]),
                    (1, 0x6A, vec![ty(L), ty(D)]),
                    (1, 0x6B, vec![ty(L), ty(E)]),
                    (1, 0x6C, vec![ty(L), ty(H)]),
                    (1, 0x6D, vec![ty(L), ty(L)]),
                    (1, 0x6E, vec![ty(L), at(ty(Hl))]),
                    (1, 0x6F, vec![ty(L), ty(A)]),
                    (1, 0x70, vec![at(ty(Hl)), ty(B)]),
                    (1, 0x71, vec![at(ty(Hl)), ty(C)]),
                    (1, 0x72, vec![at(ty(Hl)), ty(D)]),
                    (1, 0x73, vec![at(ty(Hl)), ty(E)]),
                    (1, 0x74, vec![at(ty(Hl)), ty(H)]),
                    (1, 0x75, vec![at(ty(Hl)), ty(L)]),
                    (1, 0x77, vec![at(ty(Hl)), ty(A)]),
                    (1, 0x78, vec![ty(A), ty(B)]),
                    (1, 0x79, vec![ty(A), ty(C)]),
                    (1, 0x7A, vec![ty(A), ty(D)]),
                    (1, 0x7B, vec![ty(A), ty(E)]),
                    (1, 0x7C, vec![ty(A), ty(H)]),
                    (1, 0x7D, vec![ty(A), ty(L)]),
                    (1, 0x7E, vec![ty(A), at(ty(Hl))]),
                    (1, 0x7F, vec![ty(A), ty(A)]),
                    (2, 0xE2, vec![at(ty(C)), ty(A)]),
                    (3, 0xEA, vec![at(imm(Word)), ty(A)]),
                    (2, 0xF2, vec![ty(A), at(ty(C))]),
                    (1, 0xF9, vec![ty(Sp), ty(Hl)]),
                    (3, 0xFA, vec![ty(A), at(imm(Word))]),
                ])
            }

            Ldd => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x32, vec![at(ty(Hl)), ty(A)]),
                    (1, 0x3A, vec![ty(A), at(ty(Hl))]),
                ])
            }

            Ldh => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0xE0, vec![at(imm(Byte)), ty(A)]),
                    (2, 0xF0, vec![ty(A), at(imm(Byte))]),
                ])
            }

            Ldhl => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0xF8, vec![ty(Hl), ty(Sp)]),
                ])
            }

            Ldi => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x22, vec![at(ty(Hl)), ty(A)]),
                    (1, 0x2A, vec![ty(A), at(ty(Hl))]),
                ])
            }

            Nop => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x00, vec![]),
                ])
            }

            Or => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xB0, vec![ty(B)]),
                    (1, 0xB1, vec![ty(C)]),
                    (1, 0xB2, vec![ty(D)]),
                    (1, 0xB3, vec![ty(E)]),
                    (1, 0xB4, vec![ty(H)]),
                    (1, 0xB5, vec![ty(L)]),
                    (1, 0xB6, vec![at(ty(Hl))]),
                    (1, 0xB7, vec![ty(A)]),
                    (2, 0xF6, vec![imm(Byte)]),
                ])
            }

            Pop => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC1, vec![ty(Bc)]),
                    (1, 0xD1, vec![ty(De)]),
                    (1, 0xE1, vec![ty(Hl)]),
                    (1, 0xF1, vec![ty(Af)]),
                ])
            }

            Push => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC5, vec![ty(Bc)]),
                    (1, 0xD5, vec![ty(De)]),
                    (1, 0xE5, vec![ty(Hl)]),
                    (1, 0xF5, vec![ty(Af)]),
                ])
            }

            Ret => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xC0, vec![ty(FlagNz)]),
                    (1, 0xC8, vec![ty(FlagZ)]),
                    (1, 0xC9, vec![]),
                    (1, 0xD0, vec![ty(FlagNc)]),
                    (1, 0xD8, vec![ty(FlagC)]),
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
                    (1, 0xC7, vec![imm(Word)]),
                    (1, 0xCF, vec![imm(Word)]),
                    (1, 0xD7, vec![imm(Word)]),
                    (1, 0xDF, vec![imm(Word)]),
                    (1, 0xE7, vec![imm(Word)]),
                    (1, 0xEF, vec![imm(Word)]),
                    (1, 0xF7, vec![imm(Word)]),
                    (1, 0xFF, vec![imm(Word)]),
                ])
            }

            Sbc => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x98, vec![ty(A), ty(B)]),
                    (1, 0x99, vec![ty(A), ty(C)]),
                    (1, 0x9A, vec![ty(A), ty(D)]),
                    (1, 0x9B, vec![ty(A), ty(E)]),
                    (1, 0x9C, vec![ty(A), ty(H)]),
                    (1, 0x9D, vec![ty(A), ty(L)]),
                    (1, 0x9E, vec![ty(A), at(ty(Hl))]),
                    (1, 0x9F, vec![ty(A), ty(A)]),
                    (2, 0xDE, vec![ty(A), imm(Byte)]),
                ])
            }

            Scf => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x37, vec![]),
                ])
            }

            Stop => {
                OpCode::get_opcode(instruction, false, vec![
                    (2, 0x10, vec![imm(Byte)]),
                ])
            }

            Sub => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0x90, vec![ty(B)]),
                    (1, 0x91, vec![ty(C)]),
                    (1, 0x92, vec![ty(D)]),
                    (1, 0x93, vec![ty(E)]),
                    (1, 0x94, vec![ty(H)]),
                    (1, 0x95, vec![ty(L)]),
                    (1, 0x96, vec![at(ty(Hl))]),
                    (1, 0x97, vec![ty(A)]),
                    (2, 0xD6, vec![imm(Byte)]),
                ])
            }

            Xor => {
                OpCode::get_opcode(instruction, false, vec![
                    (1, 0xA8, vec![ty(B)]),
                    (1, 0xA9, vec![ty(C)]),
                    (1, 0xAA, vec![ty(D)]),
                    (1, 0xAB, vec![ty(E)]),
                    (1, 0xAC, vec![ty(H)]),
                    (1, 0xAD, vec![ty(L)]),
                    (1, 0xAE, vec![at(ty(Hl))]),
                    (1, 0xAF, vec![ty(A)]),
                    (2, 0xEE, vec![imm(Byte)]),
                ])
            }

            // CB instructions

            Bit => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x40, vec![imm(Byte), ty(B)]),
                    (2, 0x41, vec![imm(Byte), ty(C)]),
                    (2, 0x42, vec![imm(Byte), ty(D)]),
                    (2, 0x43, vec![imm(Byte), ty(E)]),
                    (2, 0x44, vec![imm(Byte), ty(H)]),
                    (2, 0x45, vec![imm(Byte), ty(L)]),
                    (2, 0x46, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x47, vec![imm(Byte), ty(A)]),
                    (2, 0x48, vec![imm(Byte), ty(B)]),
                    (2, 0x49, vec![imm(Byte), ty(C)]),
                    (2, 0x4A, vec![imm(Byte), ty(D)]),
                    (2, 0x4B, vec![imm(Byte), ty(E)]),
                    (2, 0x4C, vec![imm(Byte), ty(H)]),
                    (2, 0x4D, vec![imm(Byte), ty(L)]),
                    (2, 0x4E, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x4F, vec![imm(Byte), ty(A)]),
                    (2, 0x50, vec![imm(Byte), ty(B)]),
                    (2, 0x51, vec![imm(Byte), ty(C)]),
                    (2, 0x52, vec![imm(Byte), ty(D)]),
                    (2, 0x53, vec![imm(Byte), ty(E)]),
                    (2, 0x54, vec![imm(Byte), ty(H)]),
                    (2, 0x55, vec![imm(Byte), ty(L)]),
                    (2, 0x56, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x57, vec![imm(Byte), ty(A)]),
                    (2, 0x58, vec![imm(Byte), ty(B)]),
                    (2, 0x59, vec![imm(Byte), ty(C)]),
                    (2, 0x5A, vec![imm(Byte), ty(D)]),
                    (2, 0x5B, vec![imm(Byte), ty(E)]),
                    (2, 0x5C, vec![imm(Byte), ty(H)]),
                    (2, 0x5D, vec![imm(Byte), ty(L)]),
                    (2, 0x5E, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x5F, vec![imm(Byte), ty(A)]),
                    (2, 0x60, vec![imm(Byte), ty(B)]),
                    (2, 0x61, vec![imm(Byte), ty(C)]),
                    (2, 0x62, vec![imm(Byte), ty(D)]),
                    (2, 0x63, vec![imm(Byte), ty(E)]),
                    (2, 0x64, vec![imm(Byte), ty(H)]),
                    (2, 0x65, vec![imm(Byte), ty(L)]),
                    (2, 0x66, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x67, vec![imm(Byte), ty(A)]),
                    (2, 0x68, vec![imm(Byte), ty(B)]),
                    (2, 0x69, vec![imm(Byte), ty(C)]),
                    (2, 0x6A, vec![imm(Byte), ty(D)]),
                    (2, 0x6B, vec![imm(Byte), ty(E)]),
                    (2, 0x6C, vec![imm(Byte), ty(H)]),
                    (2, 0x6D, vec![imm(Byte), ty(L)]),
                    (2, 0x6E, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x6F, vec![imm(Byte), ty(A)]),
                    (2, 0x70, vec![imm(Byte), ty(B)]),
                    (2, 0x71, vec![imm(Byte), ty(C)]),
                    (2, 0x72, vec![imm(Byte), ty(D)]),
                    (2, 0x73, vec![imm(Byte), ty(E)]),
                    (2, 0x74, vec![imm(Byte), ty(H)]),
                    (2, 0x75, vec![imm(Byte), ty(L)]),
                    (2, 0x76, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x77, vec![imm(Byte), ty(A)]),
                    (2, 0x78, vec![imm(Byte), ty(B)]),
                    (2, 0x79, vec![imm(Byte), ty(C)]),
                    (2, 0x7A, vec![imm(Byte), ty(D)]),
                    (2, 0x7B, vec![imm(Byte), ty(E)]),
                    (2, 0x7C, vec![imm(Byte), ty(H)]),
                    (2, 0x7D, vec![imm(Byte), ty(L)]),
                    (2, 0x7E, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x7F, vec![imm(Byte), ty(A)]),
                ])
            }

            Res => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x80, vec![imm(Byte), ty(B)]),
                    (2, 0x81, vec![imm(Byte), ty(C)]),
                    (2, 0x82, vec![imm(Byte), ty(D)]),
                    (2, 0x83, vec![imm(Byte), ty(E)]),
                    (2, 0x84, vec![imm(Byte), ty(H)]),
                    (2, 0x85, vec![imm(Byte), ty(L)]),
                    (2, 0x86, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x87, vec![imm(Byte), ty(A)]),
                    (2, 0x88, vec![imm(Byte), ty(B)]),
                    (2, 0x89, vec![imm(Byte), ty(C)]),
                    (2, 0x8A, vec![imm(Byte), ty(D)]),
                    (2, 0x8B, vec![imm(Byte), ty(E)]),
                    (2, 0x8C, vec![imm(Byte), ty(H)]),
                    (2, 0x8D, vec![imm(Byte), ty(L)]),
                    (2, 0x8E, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x8F, vec![imm(Byte), ty(A)]),
                    (2, 0x90, vec![imm(Byte), ty(B)]),
                    (2, 0x91, vec![imm(Byte), ty(C)]),
                    (2, 0x92, vec![imm(Byte), ty(D)]),
                    (2, 0x93, vec![imm(Byte), ty(E)]),
                    (2, 0x94, vec![imm(Byte), ty(H)]),
                    (2, 0x95, vec![imm(Byte), ty(L)]),
                    (2, 0x96, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x97, vec![imm(Byte), ty(A)]),
                    (2, 0x98, vec![imm(Byte), ty(B)]),
                    (2, 0x99, vec![imm(Byte), ty(C)]),
                    (2, 0x9A, vec![imm(Byte), ty(D)]),
                    (2, 0x9B, vec![imm(Byte), ty(E)]),
                    (2, 0x9C, vec![imm(Byte), ty(H)]),
                    (2, 0x9D, vec![imm(Byte), ty(L)]),
                    (2, 0x9E, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0x9F, vec![imm(Byte), ty(A)]),
                    (2, 0xA0, vec![imm(Byte), ty(B)]),
                    (2, 0xA1, vec![imm(Byte), ty(C)]),
                    (2, 0xA2, vec![imm(Byte), ty(D)]),
                    (2, 0xA3, vec![imm(Byte), ty(E)]),
                    (2, 0xA4, vec![imm(Byte), ty(H)]),
                    (2, 0xA5, vec![imm(Byte), ty(L)]),
                    (2, 0xA6, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xA7, vec![imm(Byte), ty(A)]),
                    (2, 0xA8, vec![imm(Byte), ty(B)]),
                    (2, 0xA9, vec![imm(Byte), ty(C)]),
                    (2, 0xAA, vec![imm(Byte), ty(D)]),
                    (2, 0xAB, vec![imm(Byte), ty(E)]),
                    (2, 0xAC, vec![imm(Byte), ty(H)]),
                    (2, 0xAD, vec![imm(Byte), ty(L)]),
                    (2, 0xAE, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xAF, vec![imm(Byte), ty(A)]),
                    (2, 0xB0, vec![imm(Byte), ty(B)]),
                    (2, 0xB1, vec![imm(Byte), ty(C)]),
                    (2, 0xB2, vec![imm(Byte), ty(D)]),
                    (2, 0xB3, vec![imm(Byte), ty(E)]),
                    (2, 0xB4, vec![imm(Byte), ty(H)]),
                    (2, 0xB5, vec![imm(Byte), ty(L)]),
                    (2, 0xB6, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xB7, vec![imm(Byte), ty(A)]),
                    (2, 0xB8, vec![imm(Byte), ty(B)]),
                    (2, 0xB9, vec![imm(Byte), ty(C)]),
                    (2, 0xBA, vec![imm(Byte), ty(D)]),
                    (2, 0xBB, vec![imm(Byte), ty(E)]),
                    (2, 0xBC, vec![imm(Byte), ty(H)]),
                    (2, 0xBD, vec![imm(Byte), ty(L)]),
                    (2, 0xBE, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xBF, vec![imm(Byte), ty(A)]),
                ])
            }

            Rl => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x10, vec![ty(B)]),
                    (2, 0x11, vec![ty(C)]),
                    (2, 0x12, vec![ty(D)]),
                    (2, 0x13, vec![ty(E)]),
                    (2, 0x14, vec![ty(H)]),
                    (2, 0x15, vec![ty(L)]),
                    (2, 0x16, vec![at(ty(Hl))]),
                    (2, 0x17, vec![ty(A)]),
                ])
            }

            Rlc => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x00, vec![ty(B)]),
                    (2, 0x01, vec![ty(C)]),
                    (2, 0x02, vec![ty(D)]),
                    (2, 0x03, vec![ty(E)]),
                    (2, 0x04, vec![ty(H)]),
                    (2, 0x05, vec![ty(L)]),
                    (2, 0x06, vec![at(ty(Hl))]),
                    (2, 0x07, vec![ty(A)]),
                ])
            }

            Rr => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x18, vec![ty(B)]),
                    (2, 0x19, vec![ty(C)]),
                    (2, 0x1A, vec![ty(D)]),
                    (2, 0x1B, vec![ty(E)]),
                    (2, 0x1C, vec![ty(H)]),
                    (2, 0x1D, vec![ty(L)]),
                    (2, 0x1E, vec![at(ty(Hl))]),
                    (2, 0x1F, vec![ty(A)]),
                ])
            }

            Rrc => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x08, vec![ty(B)]),
                    (2, 0x09, vec![ty(C)]),
                    (2, 0x0A, vec![ty(D)]),
                    (2, 0x0B, vec![ty(E)]),
                    (2, 0x0C, vec![ty(H)]),
                    (2, 0x0D, vec![ty(L)]),
                    (2, 0x0E, vec![at(ty(Hl))]),
                    (2, 0x0F, vec![ty(A)]),
                ])
            }

            Set => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0xC0, vec![imm(Byte), ty(B)]),
                    (2, 0xC1, vec![imm(Byte), ty(C)]),
                    (2, 0xC2, vec![imm(Byte), ty(D)]),
                    (2, 0xC3, vec![imm(Byte), ty(E)]),
                    (2, 0xC4, vec![imm(Byte), ty(H)]),
                    (2, 0xC5, vec![imm(Byte), ty(L)]),
                    (2, 0xC6, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xC7, vec![imm(Byte), ty(A)]),
                    (2, 0xC8, vec![imm(Byte), ty(B)]),
                    (2, 0xC9, vec![imm(Byte), ty(C)]),
                    (2, 0xCA, vec![imm(Byte), ty(D)]),
                    (2, 0xCB, vec![imm(Byte), ty(E)]),
                    (2, 0xCC, vec![imm(Byte), ty(H)]),
                    (2, 0xCD, vec![imm(Byte), ty(L)]),
                    (2, 0xCE, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xCF, vec![imm(Byte), ty(A)]),
                    (2, 0xD0, vec![imm(Byte), ty(B)]),
                    (2, 0xD1, vec![imm(Byte), ty(C)]),
                    (2, 0xD2, vec![imm(Byte), ty(D)]),
                    (2, 0xD3, vec![imm(Byte), ty(E)]),
                    (2, 0xD4, vec![imm(Byte), ty(H)]),
                    (2, 0xD5, vec![imm(Byte), ty(L)]),
                    (2, 0xD6, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xD7, vec![imm(Byte), ty(A)]),
                    (2, 0xD8, vec![imm(Byte), ty(B)]),
                    (2, 0xD9, vec![imm(Byte), ty(C)]),
                    (2, 0xDA, vec![imm(Byte), ty(D)]),
                    (2, 0xDB, vec![imm(Byte), ty(E)]),
                    (2, 0xDC, vec![imm(Byte), ty(H)]),
                    (2, 0xDD, vec![imm(Byte), ty(L)]),
                    (2, 0xDE, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xDF, vec![imm(Byte), ty(A)]),
                    (2, 0xE0, vec![imm(Byte), ty(B)]),
                    (2, 0xE1, vec![imm(Byte), ty(C)]),
                    (2, 0xE2, vec![imm(Byte), ty(D)]),
                    (2, 0xE3, vec![imm(Byte), ty(E)]),
                    (2, 0xE4, vec![imm(Byte), ty(H)]),
                    (2, 0xE5, vec![imm(Byte), ty(L)]),
                    (2, 0xE6, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xE7, vec![imm(Byte), ty(A)]),
                    (2, 0xE8, vec![imm(Byte), ty(B)]),
                    (2, 0xE9, vec![imm(Byte), ty(C)]),
                    (2, 0xEA, vec![imm(Byte), ty(D)]),
                    (2, 0xEB, vec![imm(Byte), ty(E)]),
                    (2, 0xEC, vec![imm(Byte), ty(H)]),
                    (2, 0xED, vec![imm(Byte), ty(L)]),
                    (2, 0xEE, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xEF, vec![imm(Byte), ty(A)]),
                    (2, 0xF0, vec![imm(Byte), ty(B)]),
                    (2, 0xF1, vec![imm(Byte), ty(C)]),
                    (2, 0xF2, vec![imm(Byte), ty(D)]),
                    (2, 0xF3, vec![imm(Byte), ty(E)]),
                    (2, 0xF4, vec![imm(Byte), ty(H)]),
                    (2, 0xF5, vec![imm(Byte), ty(L)]),
                    (2, 0xF6, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xF7, vec![imm(Byte), ty(A)]),
                    (2, 0xF8, vec![imm(Byte), ty(B)]),
                    (2, 0xF9, vec![imm(Byte), ty(C)]),
                    (2, 0xFA, vec![imm(Byte), ty(D)]),
                    (2, 0xFB, vec![imm(Byte), ty(E)]),
                    (2, 0xFC, vec![imm(Byte), ty(H)]),
                    (2, 0xFD, vec![imm(Byte), ty(L)]),
                    (2, 0xFE, vec![imm(Byte), at(ty(Hl))]),
                    (2, 0xFF, vec![imm(Byte), ty(A)]),
                ])
            }

            Sla => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x20, vec![ty(B)]),
                    (2, 0x21, vec![ty(C)]),
                    (2, 0x22, vec![ty(D)]),
                    (2, 0x23, vec![ty(E)]),
                    (2, 0x24, vec![ty(H)]),
                    (2, 0x25, vec![ty(L)]),
                    (2, 0x26, vec![at(ty(Hl))]),
                    (2, 0x27, vec![ty(A)]),
                ])
            }

            Sra => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x28, vec![ty(B)]),
                    (2, 0x29, vec![ty(C)]),
                    (2, 0x2A, vec![ty(D)]),
                    (2, 0x2B, vec![ty(E)]),
                    (2, 0x2C, vec![ty(H)]),
                    (2, 0x2D, vec![ty(L)]),
                    (2, 0x2E, vec![at(ty(Hl))]),
                    (2, 0x2F, vec![ty(A)]),
                ])
            }

            Srl => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x38, vec![ty(B)]),
                    (2, 0x39, vec![ty(C)]),
                    (2, 0x3A, vec![ty(D)]),
                    (2, 0x3B, vec![ty(E)]),
                    (2, 0x3C, vec![ty(H)]),
                    (2, 0x3D, vec![ty(L)]),
                    (2, 0x3E, vec![at(ty(Hl))]),
                    (2, 0x3F, vec![ty(A)]),
                ])
            }

            Swap => {
                OpCode::get_opcode(instruction, true, vec![
                    (2, 0x30, vec![ty(B)]),
                    (2, 0x31, vec![ty(C)]),
                    (2, 0x32, vec![ty(D)]),
                    (2, 0x33, vec![ty(E)]),
                    (2, 0x34, vec![ty(H)]),
                    (2, 0x35, vec![ty(L)]),
                    (2, 0x36, vec![at(ty(Hl))]),
                    (2, 0x37, vec![ty(A)]),
                ])
            }

            _ => bug!("Op not found"),
        }
}