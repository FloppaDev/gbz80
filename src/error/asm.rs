
use crate::{
    program::fmt,
    error::{ErrCtx, SourceCtx},
};

pub trait AsmMsg: Sized + std::fmt::Debug {
    fn msg(&self) -> &'static str;
}

/// Error when parsing values from the source file.
#[derive(Debug)]
pub struct AsmErr<'a, T: AsmMsg> {
    pub ty: T,
    pub err_ctx: ErrCtx<'a>,
    pub source_ctx: SourceCtx,
}

impl<'a, T: AsmMsg> std::fmt::Display for AsmErr<'a, T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ErrCtx{ ty, file, line_number, line, word } = self.err_ctx;

        let mut strip = fmt::strip()
            .debug(&format!("{} ({:?})\n", self.source_ctx, ty)) 
            .info(&format!("({:?}) ", self.ty))
            .bold(&format!("{}\n", self.ty.msg()));

        if let Some(word_start) = self.err_ctx.word_start() {
            let line_a = line.get(..word_start).unwrap();
            let line_word = line.get(word_start..word_start+word.len()).unwrap();
            let line_b = line.get(word_start+word.len()..).unwrap();

            strip = strip
                .faint(&format!("{file}:{line_number}:"))
                .base(&format!("    {line_a}"))
                .err(line_word)
                .faint(&format!("{line_b}\n"));
        }

        else {
            strip = strip.faint(&format!("{file}:{line_number}:    {line}\n"));
        }

        write!(f, "{}", strip.read())
    }

}

/// Creates an error for one of the types that expect an `ErrCtx`.
/// Arguments:
/// - Error enum type
/// - Variant value
/// - `ErrCtx` object
macro_rules! err {
    // T: AsmMsg + Debug
    // err!(T, T::NoWorky, err_ctx)
    ($ty:ty, $e:expr, $ctx:expr) => {
        crate::error::asm::AsmErr::<$ty>{ ty: $e, err_ctx: $ctx, source_ctx: source!() }
    }
}

/// Error variants when parsing values from the source file.
#[derive(Debug, Copy, Clone)]
pub enum ParseMsg {
    EmptyStr,
    Invalid,
    BadHex,
    BadBin,
    BadDec,
    BadStr,
    BadDirective,
    BadDirectiveIdent,
    BadMacroArg,
    BadMacroArgIdent,
    BadMacroIdent,
    BadIdent,
    BadLabel,
    BadLabelIdent,
    BadNamedMark,
    BadNamedMarkLabel,
    BadNamedMarkLabelIdent,
    BadNamedMarkHex,
    BadAnonMark,
    BadAnonMarkHex,
    ReservedKeyword,
}

impl AsmMsg for ParseMsg {
    fn msg(&self) -> &'static str {
        use ParseMsg::*;

        match self {
            EmptyStr => "Empty string",
            Invalid => "Invalid as any type",
            BadHex => "Bad hexadecimal literal",
            BadBin => "Bad binary literal",
            BadDec => "Bad decimal literal",
            BadStr => "Bad string literal",
            BadDirective => "Bad directive",
            BadDirectiveIdent => "Bad directive identifier",
            BadMacroArgIdent => "Bad macro argument's identifier",
            BadMacroArg => "Bad macro argument",
            BadMacroIdent => "Bad macro call's identifier",
            BadLabel => "Bad label",
            BadLabelIdent => "Bad label's identifier",
            BadNamedMark => "Bad named marker",
            BadNamedMarkLabel => "Bad named marker's label",
            BadNamedMarkLabelIdent => "Bad named marker label's identifier",
            BadNamedMarkHex => "Bad named marker's hexadecimal literal",
            BadAnonMark => "Bad anonymous marker",
            BadAnonMarkHex => "Bad anonymous marker's hexadecimal literal",
            BadIdent => "Bad identifier",
            ReservedKeyword => "Identifier cannot be a reserved keyword",
        }
    }
}

/// Error variants when building the AST.
#[derive(Debug, Copy, Clone)]
pub enum AstMsg {
    NoTokens,
    UnmatchedParen,
    MarkWithoutLiteral,
    UnaryWithoutRhs,
    BinaryWithoutLhs,
    BinaryWithoutRhs,
}

impl AsmMsg for AstMsg {
    fn msg(&self) -> &'static str {
        use AstMsg::*;

        match self {
            NoTokens => "No tokens provided",
            UnmatchedParen => "Unclosed parens",
            MarkWithoutLiteral => "marker expected a literal",
            UnaryWithoutRhs => "Unary operator expected an operand on its right",
            BinaryWithoutLhs => "Binary operator expected an operand on its left",
            BinaryWithoutRhs => "Binary operator expected an operand on its right",
        }
    }
}

/// Error variants when expanding macros.
#[derive(Debug, Copy, Clone)]
pub enum MacroMsg {
    NoDeclIdent,
    BadDecl,
    NoDeclBody,
    BadDeclToken,
    NoCallIdent,
    DeclNotFound,
    ArgCountMismatch,
}

impl AsmMsg for MacroMsg {
    fn msg(&self) -> &'static str {
        use MacroMsg::*;

        match self {
            NoDeclIdent => "Declaration has no identifier",
            BadDecl => "Declaration is invalid",
            NoDeclBody => "Declaration has no body",
            BadDeclToken => "Unexpected token in macro declaration",
            NoCallIdent => "Macro call has no identifier",
            DeclNotFound => "Declaration not found",
            ArgCountMismatch => "Argument count in the call does not match the declaration",
        }
    }
}

/// Error variants when validating the Ast.
#[derive(Debug, Copy, Clone)]
pub enum ValidationMsg {
    BadParent,
}

impl AsmMsg for ValidationMsg {
    fn msg(&self) -> &'static str {
        use ValidationMsg::*;

        match self {
            BadParent => "Parent of the token is of an unexpected type",
        }
    }
}

/// Error variants when looking for an opcode.
#[derive(Debug, Copy, Clone)]
pub enum OpMsg {
    NotFound,
}

impl AsmMsg for OpMsg {

    fn msg(&self) -> &'static str {
        use OpMsg::*;

        match self {
            NotFound => "Could not find the corresponding opcode",
        }
    }

}

/// Error variants when calculating constant values.
#[derive(Debug, Copy, Clone)]
pub enum ConstantsMsg {
    DuplicateKey,
    MisplacedMarker,
    FileReadFailed,
    Overflow,
}

impl AsmMsg for ConstantsMsg {
    fn msg(&self) -> &'static str {
        use ConstantsMsg::*;

        match self {
            DuplicateKey => "Constant's key already existed",
            MisplacedMarker => "The location of the marker does match its value",
            FileReadFailed => "The file to include could not be read",
            Overflow => "The value overflows its capacity",
        }
    }
}

/// Error variants when evaluating expressions.
#[derive(Debug, Copy, Clone)]
pub enum ExprMsg {
    StrInExpr,
    ConstantNotFound,
    CircularDependency,
    NegativeResult,
}

impl AsmMsg for ExprMsg {
    fn msg(&self) -> &'static str {
        use ExprMsg::*;

        match self {
            StrInExpr => "String literal in expressions must be the only parameter",
            ConstantNotFound => "The constant does not exist",
            CircularDependency => "Infinite loop of dependencies",
            NegativeResult => "The result of an expression cannot be negative",
        }
    }
}

