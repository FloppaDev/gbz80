
// File generated automatically
//  - templates in 'gen/lex/data'
//  - code in 'gen/lex/src'
//
// Do no edit manually.

use TokenType::*;

pub const fn is_char_word(c: char) -> bool {
    matches!(c,
        '('|')'|'*'|'/'|'+'|'-')
}

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    Root,
    Instruction,
        InstrName,
            Adc,
            Add,
            And,
            Bit,
            Call,
            Ccf,
            Cp,
            Cpl,
            Daa,
            Dec,
            Di,
            Ei,
            Halt,
            Inc,
            Jp,
            Jr,
            Ld,
            Ldh,
            Ldi,
            Ldd,
            Ldhl,
            Or,
            Pop,
            Push,
            Res,
            Ret,
            Rl,
            Rla,
            Rlc,
            Rld,
            Rr,
            Rra,
            Rrc,
            Rrca,
            Rrd,
            Rst,
            Sbc,
            Scf,
            Set,
            Sla,
            Sll,
            Sra,
            Srl,
            Stop,
            Sub,
            Swap,
            Xor,
            Reti,
            Rlca,
            Nop,
        Argument,
            Register,
                A,
                B,
                C,
                D,
                E,
                H,
                L,
                Af,
                Bc,
                De,
                Hl,
                Sp,
            Lit,
                LitBin,
                LitHex,
                LitDec,
                LitStr,
            At,
                At0,
                At1,
            Flag,
                FlagZ,
                FlagNz,
                FlagC,
                FlagNc,
            Expr,
                BinAdd,
                BinSub,
                BinMul,
                BinDiv,
                BinMod,
                BinShr,
                BinShl,
                BinAnd,
                BinOr,
                BinXor,
                UnNot,
                UnNeg,
            Identifier,
    Directive,
        DefB,
        DefW,
        DefS,
        Include,
        Macro,
            MacroIdent,
            MacroArg,
            MacroBody,
    Marker,
        NamedMark,
        AnonMark,
        Label,
    Repeat,
    MacroCall, 
}

impl TokenType {

    /// Returns the parent of a type.
    pub const fn parent_type(self) -> Self {
        match self {
            Root|Instruction|Directive|Marker|Repeat|MacroCall => Root,

            InstrName|Argument => Instruction,

            Adc|Add|And|Bit|Call|Ccf|Cp|Cpl|Daa|Dec|Di|Ei|Halt|Inc|Jp|Jr|
            Ld|Ldh|Ldi|Ldd|Ldhl|Or|Pop|Push|Res|Ret|Rl|Rla|Rlc|Rld|Rr|Rra|
            Rrc|Rrca|Rrd|Rst|Sbc|Scf|Set|Sla|Sll|Sra|Srl|Stop|Sub|Swap|Xor|
            Reti|Rlca|Nop => InstrName,

            Register|Lit|At|Flag|Expr|Identifier => Argument,

            A|B|C|D|E|H|L|Af|Bc|De|Hl|Sp => Register,

            LitBin|LitHex|LitDec|LitStr => Lit,

            At0|At1 => At,

            FlagZ|FlagNz|FlagC|FlagNc => Flag,

            BinAdd|BinSub|BinMul|BinDiv|BinMod|BinShr|BinShl|BinAnd|BinOr|
            BinXor|UnNot|UnNeg => Expr,

            DefB|DefW|DefS|Include|Macro => Directive,

            MacroIdent|MacroArg|MacroBody => Macro,

            NamedMark|AnonMark|Label => Marker,
        }
    }

    /// Can this token type hold a value?
    pub const fn has_value(self) -> bool {
        matches!(self,
            NamedMark|MacroArg|Label|Repeat|MacroIdent|Identifier|LitBin|
            LitHex|LitDec|LitStr)
    }

    /// Is it one the tokens that end on a newline?
    pub const fn ends_on_newline(self) -> bool {
        matches!(self,
            Instruction|Argument|MacroCall|Directive|Marker|Expr|DefB|DefW|
            DefS|Include|Macro|NamedMark|AnonMark|Label)
    }

    /// Find a token type that can be identified from a word.
    pub fn get_by_word(name: &str) -> Option<Self> {
        match name {
            "adc" => Some(Adc),
            "add" => Some(Add),
            "and" => Some(And),
            "bit" => Some(Bit),
            "call" => Some(Call),
            "ccf" => Some(Ccf),
            "cp" => Some(Cp),
            "cpl" => Some(Cpl),
            "daa" => Some(Daa),
            "dec" => Some(Dec),
            "di" => Some(Di),
            "ei" => Some(Ei),
            "halt" => Some(Halt),
            "inc" => Some(Inc),
            "jp" => Some(Jp),
            "jr" => Some(Jr),
            "ld" => Some(Ld),
            "ldh" => Some(Ldh),
            "ldi" => Some(Ldi),
            "ldd" => Some(Ldd),
            "ldhl" => Some(Ldhl),
            "or" => Some(Or),
            "pop" => Some(Pop),
            "push" => Some(Push),
            "res" => Some(Res),
            "ret" => Some(Ret),
            "rl" => Some(Rl),
            "rla" => Some(Rla),
            "rlc" => Some(Rlc),
            "rld" => Some(Rld),
            "rr" => Some(Rr),
            "rra" => Some(Rra),
            "rrc" => Some(Rrc),
            "rrca" => Some(Rrca),
            "rrd" => Some(Rrd),
            "rst" => Some(Rst),
            "sbc" => Some(Sbc),
            "scf" => Some(Scf),
            "set" => Some(Set),
            "sla" => Some(Sla),
            "sll" => Some(Sll),
            "sra" => Some(Sra),
            "srl" => Some(Srl),
            "stop" => Some(Stop),
            "sub" => Some(Sub),
            "swap" => Some(Swap),
            "xor" => Some(Xor),
            "reti" => Some(Reti),
            "rlca" => Some(Rlca),
            "nop" => Some(Nop),
            "a" => Some(A),
            "b" => Some(B),
            "c" => Some(C),
            "d" => Some(D),
            "e" => Some(E),
            "h" => Some(H),
            "l" => Some(L),
            "af" => Some(Af),
            "bc" => Some(Bc),
            "de" => Some(De),
            "hl" => Some(Hl),
            "sp" => Some(Sp),
            "Z" => Some(FlagZ),
            "NZ" => Some(FlagNz),
            "C" => Some(FlagC),
            "NC" => Some(FlagNc),
            "MOD" => Some(BinMod),
            "AND" => Some(BinAnd),
            "OR" => Some(BinOr),
            "XOR" => Some(BinXor),
            "NOT" => Some(UnNot),
            "SHL" => Some(BinShl),
            "SHR" => Some(BinShr),
            "(" => Some(At0),
            ")" => Some(At1),
            "*" => Some(BinMul),
            "/" => Some(BinDiv),
            "+" => Some(BinAdd),
            "-" => Some(BinSub),
            _ => None
        }
    }

    /// Is there any type that starts with this prefix character?
    pub const fn has_prefix(prefix: char) -> bool {
        matches!(prefix, '&'|'#'|'%'|'"'|'.'|':')
    }
    
    /// Returns a `TokenType` from an index.
    #[cfg(test)]
    pub const fn at(index: usize) -> Self {
        const COUNT: usize = 108;

        match index % COUNT {
            0 => Instruction,
            1 => InstrName,
            2 => Adc,
            3 => Add,
            4 => And,
            5 => Bit,
            6 => Call,
            7 => Ccf,
            8 => Cp,
            9 => Cpl,
            10 => Daa,
            11 => Dec,
            12 => Di,
            13 => Ei,
            14 => Halt,
            15 => Inc,
            16 => Jp,
            17 => Jr,
            18 => Ld,
            19 => Ldh,
            20 => Ldi,
            21 => Ldd,
            22 => Ldhl,
            23 => Or,
            24 => Pop,
            25 => Push,
            26 => Res,
            27 => Ret,
            28 => Rl,
            29 => Rla,
            30 => Rlc,
            31 => Rld,
            32 => Rr,
            33 => Rra,
            34 => Rrc,
            35 => Rrca,
            36 => Rrd,
            37 => Rst,
            38 => Sbc,
            39 => Scf,
            40 => Set,
            41 => Sla,
            42 => Sll,
            43 => Sra,
            44 => Srl,
            45 => Stop,
            46 => Sub,
            47 => Swap,
            48 => Xor,
            49 => Reti,
            50 => Rlca,
            51 => Nop,
            52 => Argument,
            53 => Register,
            54 => A,
            55 => B,
            56 => C,
            57 => D,
            58 => E,
            59 => H,
            60 => L,
            61 => Af,
            62 => Bc,
            63 => De,
            64 => Hl,
            65 => Sp,
            66 => Lit,
            67 => LitBin,
            68 => LitHex,
            69 => LitDec,
            70 => LitStr,
            71 => At,
            72 => At0,
            73 => At1,
            74 => Flag,
            75 => FlagZ,
            76 => FlagNz,
            77 => FlagC,
            78 => FlagNc,
            79 => Expr,
            80 => BinAdd,
            81 => BinSub,
            82 => BinMul,
            83 => BinDiv,
            84 => BinMod,
            85 => BinShr,
            86 => BinShl,
            87 => BinAnd,
            88 => BinOr,
            89 => BinXor,
            90 => UnNot,
            91 => UnNeg,
            92 => Identifier,
            93 => Directive,
            94 => DefB,
            95 => DefW,
            96 => DefS,
            97 => Include,
            98 => Macro,
            99 => MacroIdent,
            100 => MacroArg,
            101 => MacroBody,
            102 => Marker,
            103 => NamedMark,
            104 => AnonMark,
            105 => Label,
            106 => Repeat,
            107 => MacroCall,
            _ => bug!()
        }
    }

    /// Checks if the token has a valid parent.
    pub fn validate(ty: TokenType, parent_type: TokenType) -> bool {
        match ty {
            Instruction|Directive|DefB|DefW|DefS|Include|Macro|InstrName|
            Adc|Add|And|Bit|Call|Ccf|Cp|Cpl|Daa|Dec|Di|Ei|Halt|Inc|Jp|Jr|
            Ld|Ldh|Ldi|Ldd|Ldhl|Or|Pop|Push|Res|Ret|Rl|Rla|Rlc|Rld|Rr|Rra|
            Rrc|Rrca|Rrd|Rst|Sbc|Scf|Set|Sla|Sll|Sra|Srl|Stop|Sub|Swap|Xor|
            Reti|Rlca|Nop|Argument|Register|A|B|C|D|E|H|L|Af|Bc|De|Hl|Sp|
            Flag|FlagZ|FlagNz|FlagC|FlagNc|LitBin|LitHex|LitDec|LitStr|Marker|
            NamedMark|AnonMark|Label|Repeat|MacroCall|MacroIdent|MacroArg|
            MacroBody => ty.parent_type() == parent_type,

            Expr => {
                matches!(parent_type, DefB|DefW)
            }

            BinAdd|BinSub|BinMul|BinDiv|BinMod|BinShr|BinShl|BinAnd|BinOr|
            BinXor|UnNot|UnNeg|At => {
                matches!(parent_type, Expr|At|BinAdd|BinSub|BinMul|BinDiv|BinMod|
                    BinShr|BinShl|BinAnd|BinOr|BinXor|UnNot|UnNeg)
            }

            Lit => {
                matches!(parent_type, Argument|Expr|At|BinAdd|BinSub|BinMul|
                    BinDiv|BinMod|BinShr|BinShl|BinAnd|BinOr|BinXor|UnNot|UnNeg|
                    Root)
            }

            Identifier => {
                matches!(parent_type, DefB|DefW|DefS|Argument|Root)
            }

            Root|At0|At1 => true
        }
    }

}
