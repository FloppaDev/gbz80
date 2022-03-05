
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

/// Generalization of a type within Argument.
/// Instruction -> Argument -> Lit -> ...   = Lit
/// Instruction -> Argument -> Identifier   = Identifier
pub const fn argument_type(ty: TokenType) -> TokenType {
    //[[argument_type]]
}

/// Can this token type hold a value?
pub const fn has_value(ty: TokenType) -> bool {
    //[[has_value]]
}

/// Is it one the tokens that end on a newline?
pub const fn ends_on_newline(ty: TokenType) -> bool {
    //[[ends_on_newline]]
}

/// Find a token type that can be identified from a word.
pub const fn get_by_word(name: &str) -> Option<TokenType> {
    //[[get_by_word]]
}

/// Find all types that match the prefix.
/// e.g. &2893 is a hexadecimal literal.
pub const fn get_by_prefix(first: &str) -> Vec<TokenType> {
    //[[get_by_prefix]]
}
