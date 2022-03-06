
// File generated automatically
//  - templates in 'gen/lex/data'
//  - code in 'gen/lex/src'
//
// Do no edit manually.

use TokenType::*;

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    Root
    Instruction
        InstrName
            Adc
            Add
            And
            Bit
            Call
            Ccf
            Cp
            Cpl
            Daa
            Dec
            Di
            Ei
            Halt
            Inc
            Jp
            Jr
            Ld
            Ldh
            Ldi
            Ldd
            Ldhl
            Or
            Pop
            Push
            Res
            Ret
            Rl
            Rla
            Rlc
            Rld
            Rr
            Rra
            Rrc
            Rrca
            Rrd
            Rst
            Sbc
            Scf
            Set
            Sla
            Sll
            Sra
            Srl
            Stop
            Sub
            Swap
            Xor
            Reti
            Rlca
            Nop
        Argument
            Register
                A
                B
                C
                D
                E
                H
                L
                Af
                Bc
                De
                Hl
                Sp
            Lit
                LitBin
                LitHex
                LitDec
                LitStr
            At
                At0
                At1
            Flag
                FlagZ
                FlagNz
                FlagC
                FlagNc
            Expr
                BinAdd
                BinSub
                BinMul
                BinDiv
                BinMod
                BinShr
                BinShl
                BinAnd
                BinOr
                BinXor
                BinPow
                UnNot
                UnNeg
            Identifier
    Directive
        Define
        Include
        Macro
            MacroIdent
            MacroArg
            MacroBody
    Marker
        NamedMark
        AnonMark
        Label
    Repeat
    MacroCall
    Unknown 
}

/// Returns the parent of a type.
pub const fn parent_type(ty: TokenType) -> TokenType {
    match ty {
        Root => Root,

        Instruction|Directive|Marker|Repeat|MacroCall|Unknown => Root,

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

        Define|Include|Macro => Directive,

        MacroIdent|MacroArg|MacroBody => Macro,

        NamedMark|AnonMark|Label => Marker,

        _ => unreachable!()
    }
}

/// Can this token type hold a value?
pub const fn has_value(ty: TokenType) -> bool {
    match ty {
        NamedMark|MacroArg|Label|Repeat|MacroIdent|Identifier|LitBin|
        LitHex|LitDec|LitStr => true,

        _ => false
    }
}

/// Is it one the tokens that end on a newline?
pub const fn ends_on_newline(ty: TokenType) -> bool {
    match ty {
        Instruction|Argument|MacroCall|Directive|Marker|Define|Include|
        Macro|NamedMark|AnonMark|Label => true,

        _ => false
    }
}

/// Find a token type that can be identified from a word.
pub const fn get_by_word(name: &str) -> Option<TokenType> {
    match name {
        "adc" => Adc,
        "add" => Add,
        "and" => And,
        "bit" => Bit,
        "call" => Call,
        "ccf" => Ccf,
        "cp" => Cp,
        "cpl" => Cpl,
        "daa" => Daa,
        "dec" => Dec,
        "di" => Di,
        "ei" => Ei,
        "halt" => Halt,
        "inc" => Inc,
        "jp" => Jp,
        "jr" => Jr,
        "ld" => Ld,
        "ldh" => Ldh,
        "ldi" => Ldi,
        "ldd" => Ldd,
        "ldhl" => Ldhl,
        "or" => Or,
        "pop" => Pop,
        "push" => Push,
        "res" => Res,
        "ret" => Ret,
        "rl" => Rl,
        "rla" => Rla,
        "rlc" => Rlc,
        "rld" => Rld,
        "rr" => Rr,
        "rra" => Rra,
        "rrc" => Rrc,
        "rrca" => Rrca,
        "rrd" => Rrd,
        "rst" => Rst,
        "sbc" => Sbc,
        "scf" => Scf,
        "set" => Set,
        "sla" => Sla,
        "sll" => Sll,
        "sra" => Sra,
        "srl" => Srl,
        "stop" => Stop,
        "sub" => Sub,
        "swap" => Swap,
        "xor" => Xor,
        "reti" => Reti,
        "rlca" => Rlca,
        "nop" => Nop,
        "a" => A,
        "b" => B,
        "c" => C,
        "d" => D,
        "e" => E,
        "h" => H,
        "l" => L,
        "af" => Af,
        "bc" => Bc,
        "de" => De,
        "hl" => Hl,
        "sp" => Sp,
        "(" => At0,
        ")" => At1,
        "Z" => FlagZ,
        "NZ" => FlagNz,
        "C" => FlagC,
        "NC" => FlagNc,
        _ => None
    }
}

/// Is there any type that starts with this prefix character?
pub const fn has_prefix(prefix: char) -> bool {
    matches!(prefix, '&'|'#'|'%'|'"'|'.'|':')
}
