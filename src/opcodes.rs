// Macro seems to work but throws unused warnings.
#![allow(unused, unused_mut)]

use crate::ast::TokenType::{self, *};

#[derive(Debug)]
pub struct InstructionDef {
    pub ty: TokenType,
    pub ops: Vec<Op>,
}

#[derive(Debug)]
pub struct Op {
    pub args: Vec<TokenType>,
    pub bytes: u16,
    //TODO rename var_size in Token
    pub input: usize //TODO? u16
}

/// Maps intructions in source to those in the opcodes declaration.
pub struct OpMap<'a> {
    map: HashMap<usize, &'a Op>,
}

impl<'a> OpMap<'a> {

    pub fn new<'a>(
        ast: &'a Token,
        instructions: &'a[InstructionDef],
    ) -> Result<Self, Vec<OpErr>> {
        let mut errors = vec![];
        let mut map = HashMap::new();

        Self::collect_instructions(ast, instructions, &mut map, &mut errors);

        return if errors.is_empty() {
            Ok(Self{ map })
        }else {
            Err(errors)
        };
    }

    /// Iterates through all base level tokens.
    /// If a token is a macro call, iterates through its children.
    fn collect_instructions<'a>(
        root: &'a Token,
        instructions: &'a[InstructionDef], 
        mut hashmap: &mut HashMap<usize, &'a Op>,
        mut errors: &mut Vec<OpErr>,
    ) {
        for token in &root.children {
            match token.ty {
                Instruction => {}

                // Macro calls cannot be nested so recursion is not an issue.
                MacroCall => {
                    Self::collect_instructions(token, instructions, hashmap, errors);
                    continue;
                }

                _ => continue,
            }

            Self::get_instruction(); 
        }
    }

    /// Collects mappings of tokens to instructions.
    fn get_instruction(
        token: &'a Token,
        instructions: &'a[InstructionDef], 
        mut hashmap: &mut HashMap<usize, &'a Op>,
        mut errors: &mut Vec<OpErr>,
    ) {
        let err_ctx = token.into();

        let ty = token.children[0].children[0].ty;
        let child_count = token.children.len();
        let arg_count = child_count - 1;

        let mut instruction = None;

        // Find instruction by type.
        for inst in instructions {
            if inst.ty == ty {
                instruction = Some(inst);
            }
        }

        if instruction.is_none() { 
            errors.push(OpErr::new(OpErrType::NotFound, err_ctx));
            continue;
        }

        let instruction = instruction.unwrap();

        let mut args = Self::format_args();
        let arg_count = args.len();

        if let Some(instr_op) = Self::compare_args() {
            hashmap.insert(token.index, instr_op);
        }

        else {
            //TODO OpErr
        }
    }

    /// Format arguments in the same way as in the opcodes module.
    fn format_args() {
        for (i, arg) in token.children[1..].iter().enumerate() {
            let arg_cat = &arg.children[0];

            match arg_cat.ty {
                Register => args.push(arg_cat.children[0].ty),

                Flag => args.push(arg_cat.children[0].ty),

                Lit => {
                    // Is it it a bit index?
                    if matches!(token.children[0].children[0].ty, Bit|Res|Rst|Set) {
                        match Self::bx_from_str(arg_cat.children[0].value.as_str()) {
                            Ok(bx) => args.push(bx),

                            Err(e) => ()//TODO
                        }
                    }

                    else {
                        args.push(Lit);
                    }
                }

                Identifier => args.push(Lit),

                At => {
                    args.push(At0);
                    let at_child = &arg_cat.children[0];

                    match at_child.ty {
                        Register => args.push(at_child.children[0].ty),

                        Lit|Identifier => args.push(Lit),

                        Plus => {
                            args.push(at_child.children[0].ty);
                            args.push(Plus);
                            args.push(at_child.children[1].ty);
                        }

                        _ => {
                            //TODO OpErr
                            let e = format!(    "Token of type {:?} not expected in adress \
                                                for instruction {:?}. (L{})",
                                                arg_cat.children[0].ty,
                                                ty,
                                                arg_cat.line);
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
    }

    /// Convert a number string to bit index (0-7).
    fn bx_from_str(s: &str) -> Result<TokenType, OpErr> {
        match s {
            "0" => Ok(args.push(B0)),
            "1" => Ok(args.push(B1)),
            "2" => Ok(args.push(B2)),
            "3" => Ok(args.push(B3)),
            "4" => Ok(args.push(B4)),
            "5" => Ok(args.push(B5)),
            "6" => Ok(args.push(B6)),
            "7" => Ok(args.push(B7)),
            _ => Err(OpErr::new()),
        }
    }

    /// Compare the arguments to find the correct `Op` from opcodes.
    fn compare_args() -> Option<()> {
        // Iterate through all variations of the instruction.
        'op_loop: for op in &instruction.ops {
            if arg_count == op.args.len() {
                if arg_count == 0 {
                    return Some(op);
                }

                else {
                    // This loop completes if all arguments match.
                    for (i, arg) in args.iter().enumerate() {
                        if *arg != op.args[i] {
                            continue 'op_loop;
                        }
                    }

                    return Some(op);
                }
            }

            else if arg_count == op.args.len() - 1 && op.args[0] == A {
                // When the first agument is A, it is optionnal.
                for (i, arg) in args.iter().enumerate() {
                    if *arg != op.args[i+1] {
                        continue 'op_loop;
                    }
                }

                return Some(op);
            }
        }
    }

}

macro_rules! opcodes {(
    $(
        :$instr:ident $(
            $opc:literal $in:literal $($arg:ident)*
        )*
    )*
) => {
        /// Get instructions from the opcodes file.
        pub fn get_instructions() -> Vec<InstructionDef> {
            let mut instructions = vec![];    

            $(//:Adc
                let mut instr = InstructionDef { ty: $instr, ops: vec![] };
                $(//0x8E 0
                    let mut op = Op { args: vec![], bytes: $opc, input: $in };
                    $(//A At0 Hl At1
                        op.args.push($arg);
                    )*
                    instr.ops.push(op);
                )*
                instructions.push(instr);
            )*

            instructions
        }
    }
}

opcodes!{
    :Adc    0x88   0 A B
            0x89   0 A C
            0x8A   0 A D
            0x8B   0 A E
            0x8C   0 A H
            0x8D   0 A L
            0x8E   0 A At0 Hl At1
            0x8F   0 A A
            0xCE   1 A Lit

    :Add    0x80   0 A B
            0x81   0 A C
            0x82   0 A D
            0x83   0 A E
            0x84   0 A H
            0x85   0 A L
            0x86   0 A At0 Hl At1
            0x87   0 A A
            0xC6   1 A Lit
            0x09   0 Hl Bc
            0x19   0 Hl De
            0x29   0 Hl Hl
            0x39   0 Hl Sp
            0xE8   1 Sp Lit

    :And    0xA0   0 B
            0xA1   0 C
            0xA2   0 D
            0xA3   0 E
            0xA4   0 H
            0xA5   0 L
            0xA6   0 At0 Hl At1
            0xA7   0 A
            0xE6   1 Lit

    :Bit    0xCB40 0 B0 B
            0xCB41 0 B0 C
            0xCB42 0 B0 D
            0xCB43 0 B0 E
            0xCB44 0 B0 H
            0xCB45 0 B0 L
            0xCB46 0 B0 At0 Hl At1
            0xCB47 0 B0 A
            0xCB48 0 B1 B
            0xCB49 0 B1 C
            0xCB4A 0 B1 D
            0xCB4B 0 B1 E
            0xCB4C 0 B1 H
            0xCB4D 0 B1 L
            0xCB4E 0 B1 At0 Hl At1
            0xCB1F 0 B1 A
            0xCB50 0 B2 B
            0xCB51 0 B2 C
            0xCB52 0 B2 D
            0xCB53 0 B2 E
            0xCB54 0 B2 H
            0xCB55 0 B2 L
            0xCB56 0 B2 At0 Hl At1
            0xCB57 0 B2 A
            0xCB58 0 B3 B
            0xCB59 0 B3 C
            0xCB5A 0 B3 D
            0xCB5B 0 B3 E
            0xCB5C 0 B3 H
            0xCB5D 0 B3 L
            0xCB5E 0 B3 At0 Hl At1
            0xCB5F 0 B3 A
            0xCB60 0 B4 B
            0xCB61 0 B4 C
            0xCB62 0 B4 D
            0xCB63 0 B4 E
            0xCB64 0 B4 H
            0xCB65 0 B4 L
            0xCB66 0 B4 At0 Hl At1
            0xCB67 0 B4 A
            0xCB68 0 B5 B
            0xCB69 0 B5 C
            0xCB6A 0 B5 D
            0xCB6B 0 B5 E
            0xCB6C 0 B5 H
            0xCB6D 0 B5 L
            0xCB6E 0 B5 At0 Hl At1
            0xCB6F 0 B5 A
            0xCB70 0 B6 B
            0xCB71 0 B6 C
            0xCB72 0 B6 D
            0xCB73 0 B6 E
            0xCB74 0 B6 H
            0xCB75 0 B6 L
            0xCB76 0 B6 At0 Hl At1
            0xCB77 0 B6 A
            0xCB78 0 B7 B
            0xCB79 0 B7 C
            0xCB7A 0 B7 D
            0xCB7B 0 B7 E
            0xCB7C 0 B7 H
            0xCB7D 0 B7 L
            0xCB72 0 B7 At0 Hl At1
            0xCB7F 0 B7 A

    :Call   0xCD   2 Lit
            0xDC   2 FlagC Lit
            0xD4   2 FlagNc Lit
            0xC4   2 FlagNz Lit
            0xCC   2 FlagZ Lit

    :Ccf    0x3F   0

    :Cp     0xB8   0 B
            0xB9   0 C
            0xBA   0 D
            0xBB   0 E
            0xBC   0 H
            0xBD   0 L
            0xBE   0 At0 Hl At1
            0xBF   0 A
            0xFE   1 Lit

    :Cpl    0x2F   0

    :Daa    0x27   0

    :Dec    0x05   0 B
            0x0B   0 Bc
            0x0D   0 C
            0x15   0 D
            0x1B   0 De
            0x1D   0 E
            0x25   0 H
            0xDD25 0 Hix
            0xFD25 0 Hiy
            0x2B   0 Hl
            0x2D   0 L
            0x35   0 At0 Hl At1
            0x3D   0 A
            0x3B   0 Sp

    :Di     0xF3   0

    :Ei     0xFB   0

    :Halt   0x76   0

    :Inc    0x04   0 B
            0x03   0 Bc
            0x0C   0 C
            0x14   0 D
            0x13   0 De
            0x1C   0 E
            0x24   0 H
            0x23   0 Hl
            0x2C   0 L
            0x34   0 At0 Hl At1
            0x3C   0 A
            0x33   0 Sp

    :Jp     0xE9   0 At0 Hl At1
            0xC3   2 Lit
            0xDA   2 FlagC Lit
            0xD2   2 FlagNc Lit
            0xC2   2 FlagNz Lit
            0xCA   2 FlagZ Lit

    :Jr     0x38   1 FlagC Lit
            0x18   1 Lit
            0x30   1 FlagNc Lit
            0x20   1 FlagNz Lit
            0x28   1 FlagZ Lit

    :Ld     0xEA   2 At0 Lit At1 A
            0x08   2 At0 Lit At1 Sp
            0x02   0 At0 Bc At1 A
            0x12   0 At0 De At1 A
            0x77   0 At0 Hl At1 A
            0x70   0 At0 Hl At1 B
            0x71   0 At0 Hl At1 C
            0x72   0 At0 Hl At1 D
            0x73   0 At0 Hl At1 E
            0x74   0 At0 Hl At1 H
            0x75   0 At0 Hl At1 L
            0x36   1 At0 Hl At1 Lit
            0xE2   0 At0 Lit Plus C At1 A
            0xE0   1 At0 Lit Plus Lit At1 A
            0xFA   2 A At0 Lit At1
            0x0A   0 A At0 Bc At1
            0x1A   0 A At0 De At1
            0x78   0 A B
            0x79   0 A C
            0x7A   0 A D
            0x7B   0 A E
            0x7C   0 A H
            0x7D   0 A L
            0x7E   0 A At0 Hl At1
            0x7F   0 A A
            0x3E   1 A Lit
            0xF0   1 A At0 Lit Plus Lit At1
            0xF2   0 A At0 Lit Plus C At1
            0x40   0 B B
            0x41   0 B C
            0x42   0 B D
            0x43   0 B E
            0x44   0 B H
            0x45   0 B L
            0x46   0 B At0 Hl At1
            0x47   0 B A
            0x06   1 B Lit
            0x01   2 Bc Lit
            0x48   0 C B
            0x49   0 C C
            0x4A   0 C D
            0x4B   0 C E
            0x4C   0 C H
            0x4D   0 C L
            0x4E   0 C At0 Hl At1
            0x4F   0 C A
            0x0E   1 C Lit
            0x50   0 D B
            0x51   0 D C
            0x52   0 D D
            0x53   0 D E
            0x54   0 D H
            0x55   0 D L
            0x56   0 D At0 Hl At1
            0x57   0 D A
            0x16   1 D Lit
            0x11   2 De Lit
            0x58   0 E B
            0x59   0 E C
            0x5A   0 E D
            0x5B   0 E E
            0x5C   0 E H
            0x5D   0 E L
            0x5E   0 E At0 Hl At1
            0x5F   0 E A
            0x1E   1 E Lit
            0x60   0 H B
            0x61   0 H C
            0x62   0 H D
            0x63   0 H E
            0x64   0 H H
            0x65   0 H L
            0x66   0 H At0 Hl At1
            0x67   0 H A
            0x26   1 H Lit
            0x21   2 Hl Lit
            0xF8   1 Hl Sp Plus Lit
            0x68   0 L B
            0x69   0 L C
            0x6A   0 L D
            0x6B   0 L E
            0x6C   0 L H
            0x6D   0 L L
            0x6E   0 L At0 Hl At1
            0x6F   0 L A
            0x2E   1 L Lit
            0xED7B 2 Sp At0 Lit At1
            0x31   1 Sp Lit
            0xF9   0 Sp Hl

    :Ldi    0x2A   0 A At0 Hl At1
            0x22   0 At0 Hl At1 A

    :Ldd    0x32   0 A At0 Hl At1
            0x3A   0 At0 Hl At1 A

    :Nop    0x00   0

    :Or     0xB0   0 B
            0xB1   0 C
            0xB2   0 D
            0xB3   0 E
            0xB4   0 H
            0xB5   0 L
            0xB6   0 At0 Hl At1
            0xB7   0 A
            0xF6   1 Lit

    :Pop    0xF1   0 Af
            0xC1   0 Bc
            0xD1   0 De
            0xE1   0 Hl

    :Push   0xF5   0 Af
            0xC5   0 Bc
            0xD5   0 De
            0xE5   0 Hl

    :Res    0xCB80 0 B0 B
            0xCB81 0 B0 C
            0xCB82 0 B0 D
            0xCB83 0 B0 E
            0xCB84 0 B0 H
            0xCB85 0 B0 L
            0xCB86 0 B0 At0 Hl At1
            0xCB87 0 B0 A
            0xCB88 0 B1 B
            0xCB89 0 B1 C
            0xCB8A 0 B1 D
            0xCB8B 0 B1 E
            0xCB8C 0 B1 H
            0xCB8D 0 B1 L
            0xCB8E 0 B1 At0 Hl At1
            0xCB8F 0 B1 A
            0xCB90 0 B2 B
            0xCB91 0 B2 C
            0xCB92 0 B2 D
            0xCB93 0 B2 E
            0xCB94 0 B2 H
            0xCB95 0 B2 L
            0xCB96 0 B2 At0 Hl At1
            0xCB97 0 B2 A
            0xCB98 0 B3 B
            0xCB99 0 B3 C
            0xCB9A 0 B3 D
            0xCB9B 0 B3 E
            0xCB9C 0 B3 H
            0xCB9D 0 B3 L
            0xCB9E 0 B3 At0 Hl At1
            0xCB9F 0 B3 A
            0xCBA0 0 B4 B
            0xCBA1 0 B4 C
            0xCBA2 0 B4 D
            0xCBA3 0 B4 E
            0xCBA4 0 B4 H
            0xCBA5 0 B4 L
            0xCBA6 0 B4 At0 Hl At1
            0xCBA7 0 B4 A
            0xCBA8 0 B5 B
            0xCBA9 0 B5 C
            0xCBAA 0 B5 D
            0xCBAB 0 B5 E
            0xCBAC 0 B5 H
            0xCBAD 0 B5 L
            0xCBAE 0 B5 At0 Hl At1
            0xCBAF 0 B5 A
            0xCBB0 0 B6 B
            0xCBB1 0 B6 C
            0xCBB2 0 B6 D
            0xCBB3 0 B6 E
            0xCBB4 0 B6 H
            0xCBB5 0 B6 L
            0xCBB6 0 B6 At0 Hl At1
            0xCBB7 0 B6 A
            0xCBB8 0 B7 B
            0xCBB9 0 B7 C
            0xCBBA 0 B7 D
            0xCBBB 0 B7 E
            0xCBBC 0 B7 H
            0xCBBD 0 B7 L
            0xCBBE 0 B7 At0 Hl At1
            0xCBBF 0 B7 A

    :Ret    0xC9   0
            0xD8   0 FlagC
            0xD0   0 FlagNc
            0xC0   0 FlagNz
            0xC8   0 FlagZ
            0xD9   0

    :Reti   0xD9   0 

    :Rl     0xCB10 0 B
            0xCB11 0 C
            0xCB12 0 D
            0xCB13 0 E
            0xCB14 0 H
            0xCB15 0 L
            0xCB16 0 At0 Hl At1
            0xCB17 0 A

    :Rla    0x17   0

    :Rlc    0xCB00 0 B
            0xCB01 0 C
            0xCB02 0 D
            0xCB03 0 E
            0xCB04 0 H
            0xCB05 0 L
            0xCB06 0 At0 Hl At1
            0xCB07 0 A

    :Rlca   0x07   0

    :Rld    0xED6F 0

    :Rr     0xCB18 0 B
            0xCB19 0 C
            0xCB1A 0 D
            0xCB1B 0 E
            0xCB1C 0 H
            0xCB1D 0 L
            0xCB1E 0 At0 Hl At1
            0xCB1F 0 A

    :Rra    0x1F   0

    :Rrc    0xCB08 0 B
            0xCB09 0 C
            0xCB0A 0 D
            0xCB0B 0 E
            0xCB0C 0 H
            0xCB0D 0 L
            0xCB0E 0 At0 Hl At1
            0xCB0F 0 A

    :Rrca   0x0F   0

    :Rrd    0xED67 0

    :Rst    0xC7   0 B0
            0xCF   2 B1 Lit
            0xD7   2 B2 Lit
            0xDF   2 B3 Lit
            0xE7   0 B4
            0xEF   2 B5 Lit
            0xF7   0 B6
            0xFF   0 B7

    :Sbc    0x98   0 A B
            0x99   0 A C
            0x9A   0 A D
            0x9B   0 A E
            0x9C   0 A H
            0x9D   0 A L
            0x9E   0 A At0 Hl At1
            0x9F   0 A A
            0xDE   1 A Lit

    :Scf    0x37   0

    :Set    0xCBC0 0 B0 B
            0xCBC1 0 B0 C
            0xCBC2 0 B0 D
            0xCBC3 0 B0 E
            0xCBC4 0 B0 H
            0xCBC5 0 B0 L
            0xCBC6 0 B0 At0 Hl At1
            0xCBC7 0 B0 A
            0xCBC8 0 B1 B
            0xCBC9 0 B1 C
            0xCBCA 0 B1 D
            0xCBCB 0 B1 E
            0xCBCC 0 B1 H
            0xCBCD 0 B1 L
            0xCBCE 0 B1 At0 Hl At1
            0xCBCF 0 B1 A
            0xCBD0 0 B2 B
            0xCBD1 0 B2 C
            0xCBD2 0 B2 D
            0xCBD3 0 B2 E
            0xCBD4 0 B2 H
            0xCBD5 0 B2 L
            0xCBD6 0 B2 At0 Hl At1
            0xCBD7 0 B2 A
            0xCBD8 0 B3 B
            0xCBD9 0 B3 C
            0xCBDA 0 B3 D
            0xCBDB 0 B3 E
            0xCBDC 0 B3 H
            0xCBDD 0 B3 L
            0xCBDE 0 B3 At0 Hl At1
            0xCBDF 0 B3 A
            0xCBE0 0 B4 B
            0xCBE1 0 B4 C
            0xCBE2 0 B4 D
            0xCBE3 0 B4 E
            0xCBE4 0 B4 H
            0xCBE5 0 B4 L
            0xCBE6 0 B4 At0 Hl At1
            0xCBE7 0 B4 A
            0xCBE8 0 B5 B
            0xCBE9 0 B5 C
            0xCBEA 0 B5 D
            0xCBEB 0 B5 E
            0xCBEC 0 B5 H
            0xCBED 0 B5 L
            0xCBEE 0 B5 At0 Hl
            0xCBEF 0 B5 A
            0xCBF0 0 B6 B
            0xCBF1 0 B6 C
            0xCBF2 0 B6 D
            0xCBF3 0 B6 E
            0xCBF4 0 B6 H
            0xCBF5 0 B6 L
            0xCBF6 0 B6 At0 Hl At1
            0xCBF7 0 B6 A
            0xCBF8 0 B7 B
            0xCBF9 0 B7 C
            0xCBFA 0 B7 D
            0xCBFB 0 B7 E
            0xCBFC 0 B7 H
            0xCBFD 0 B7 L
            0xCBFE 0 B7 At0 Hl At1
            0xCBFF 0 B7 A

    :Sla    0xCB20 0 B
            0xCB21 0 C
            0xCB22 0 D
            0xCB23 0 E
            0xCB24 0 H
            0xCB25 0 L
            0xCB26 0 At0 Hl At1
            0xCB27 0 A

    :Sll    0xCB30 0 B
            0xCB31 0 C
            0xCB32 0 D
            0xCB33 0 E
            0xCB34 0 H
            0xCB35 0 L
            0xCB36 0 At0 Hl At1
            0xCB37 0 A

    :Sra    0xCB28 0 B
            0xCB29 0 C
            0xCB2A 0 D
            0xCB2B 0 E
            0xCB2C 0 H
            0xCB2D 0 L
            0xCB2E 0 At0 Hl At1
            0xCB2F 0 A

            0xCB38 0 B
            0xCB39 0 C
            0xCB3A 0 D
            0xCB3B 0 E
            0xCB3C 0 H
            0xCB3D 0 L
            0xCB3E 0 At0 Hl At1
            0xCB3F 0 A

    :Stop   0x1000 0

    :Sub    0x90   0 B
            0x91   0 C
            0x92   0 D
            0x93   0 E
            0x94   0 H
            0x95   0 L
            0x96   0 At0 Hl At1
            0x97   0 A
            0xDDAD 0 Lix
            0xFDAD 0 Liy
            0xD6   1 Lit

    :Swap   0xCB30 0 B
            0xCB31 0 C
            0xCB32 0 D
            0xCB33 0 E
            0xCB34 0 H
            0xCB35 0 L
            0xCB36 0 At0 Hl At1
            0xCB37 0 A

    :Xor    0xA8   0 B
            0xA9   0 C
            0xAA   0 D
            0xAB   0 E
            0xAC   0 H
            0xAD   0 L
            0xAE   0 At0 Hl At1
            0xAF   0 A
            0xEE   1 Lit
}
