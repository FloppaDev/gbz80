
// File generated automatically
//  - templates in 'gen/lex/data'
//  - code in 'gen/lex/src'
//
// Do no edit manually.

use TokenType::*;

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
                BinPow,
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
    Unknown, 
}

impl TokenType {

    /// Returns the parent of a type.
    pub const fn parent_type(&self) -> TokenType {
        match self {
            Root|Instruction|Directive|Marker|Repeat|MacroCall|Unknown => Root,

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
            BinXor|BinPow|UnNot|UnNeg => Expr,

            DefB|DefW|DefS|Include|Macro => Directive,

            MacroIdent|MacroArg|MacroBody => Macro,

            NamedMark|AnonMark|Label => Marker,
        }
    }

    /// Can this token type hold a value?
    pub const fn has_value(&self) -> bool {
        matches!(self,
            NamedMark|MacroArg|Label|Repeat|MacroIdent|Identifier|LitBin|
            LitHex|LitDec|LitStr)
    }

    /// Is it one the tokens that end on a newline?
    pub const fn ends_on_newline(&self) -> bool {
        matches!(self,
            Instruction|Argument|MacroCall|Directive|Marker|DefB|DefW|DefS|
            Include|Macro|NamedMark|AnonMark|Label)
    }

    /// Find a token type that can be identified from a word.
    pub fn get_by_word(name: &str) -> Option<TokenType> {
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
            "(" => Some(At0),
            ")" => Some(At1),
            "Z" => Some(FlagZ),
            "NZ" => Some(FlagNz),
            "C" => Some(FlagC),
            "NC" => Some(FlagNc),
            _ => None
        }
    }

    /// Is there any type that starts with this prefix character?
    pub const fn has_prefix(prefix: char) -> bool {
        matches!(prefix, '&'|'#'|'%'|'"'|'.'|':')
    }

}
