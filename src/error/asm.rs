
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
        let ErrCtx{ line_number, line, word } = self.err_ctx;

        let text = fmt::strip()
            .debug(&format!("{}\n", self.source_ctx)) 
            .info(&format!("({:?}) ", self.ty))
            .bold(&format!("{} ", self.ty.msg()))
            .faint(&format!("l{}:", line_number ))
            .bold(&format!("{}\n", word))
            .faint(&format!("    {}\n", line))
            .read();

        write!(f, "{}", text)
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
    /// Number literals are either 1 or 2 bytes long. (255 or 65535 max value)
    HexOverflow,
    DecOverflow,
    BinOverflow,

    EmptyStr,

    /// Syntax errors
    Invalid,
    InvalidHex,
    InvalidBin,
    InvalidDec,
    InvalidStr,
    InvalidDirective,
    InvalidDirectiveIdent,
    InvalidMacroArg,
    InvalidMacroArgIdent,
    InvalidMacroIdent,
    InvalidIdent,
    InvalidLabel,
    InvalidLabelIdent,
    InvalidNamedMark,
    InvalidNamedMarkLabel,
    InvalidNamedMarkLabelIdent,
    InvalidNamedMarkHex,
    InvalidAnonMark,
    InvalidAnonMarkHex,

    //TODO those are bugs
    UnhandledType,
    UnexpectedPrefix,
}

impl AsmMsg for ParseMsg {

    fn msg(&self) -> &'static str {
        use ParseMsg::*;

        match self {
            HexOverflow => 
                "Hexadecimal literal overflow",

            BinOverflow => 
                "Binary literal overflow",

            DecOverflow => 
                "Decimal literal overflow",

            EmptyStr => 
                "Empty string",

            Invalid => 
                "Invalid as any type",

            InvalidHex => 
                "Invalid as hexadecimal literal",

            InvalidBin => 
                "Invalid as binary literal",

            InvalidDec => 
                "Invalid as decimal literal",

            InvalidStr => 
                "Invalid as string literal",

            InvalidDirective => 
                "Invalid as directive",

            InvalidDirectiveIdent => 
                "Invalid as directive identifier",

            InvalidMacroArgIdent => 
                "Invalid as macro argument's identifier",

            InvalidMacroArg => 
                "Invalid as macro argument",

            InvalidMacroIdent => 
                "Invalid as macro call's identifier",

            InvalidLabel =>
                "Invalid as label",

            InvalidLabelIdent =>
                "Invalid as label's identifier",

            InvalidNamedMark => 
                "Invalid as named marker",

            InvalidNamedMarkLabel => 
                "Invalid as named marker's label",

            InvalidNamedMarkLabelIdent => 
                "Invalid as named marker label's identifier",

            InvalidNamedMarkHex => 
                "Invalid as named marker's hexadecimal literal",

            InvalidAnonMark =>
                "Invalid as anonymous marker",

            InvalidAnonMarkHex =>
                "Invalid as anonymous marker's hexadecimal literal",

            InvalidIdent => 
                "Invalid as identifier",

            UnhandledType => 
                "Parser could not handle the token type",

            UnexpectedPrefix => 
                "Wrong type identified from prefix",
        }
    }

}

/// Error variants when building the AST.
#[derive(Debug, Copy, Clone)]
pub enum AstMsg {
    NoTokens,
    UnmatchedParen,
    PlusWithoutRhs,
    MinusWithoutRhs,
    MarkWithoutLiteral,
    //TODO assembler bug
    UnhandledNewline,
    //TODO assembler bug
    UnknownError,
}

impl AsmMsg for AstMsg {

    fn msg(&self) -> &'static str {
        use AstMsg::*;

        match self {
            NoTokens =>
                "No tokens were provided",

            UnmatchedParen =>
                "Parens must come in pair",

            PlusWithoutRhs =>
                "Plus operation expects a right-hand side operand",

            MinusWithoutRhs =>
                "Minus operation expects a right-hand side operand",

            MarkWithoutLiteral => 
                "marker expected a literal",

            UnhandledNewline =>
                "Internal error on new line",

            UnknownError =>
                "Unknown error",
        }
    }
}

/// Error variants when expanding macros.
#[derive(Debug, Copy, Clone)]
pub enum MacroMsg {
    NoDeclIdent,
    InvalidDecl,
    NoDeclBody,
    InvalidDeclToken,
    NoCallIdent,
    DeclNotFound,
    ArgCountMismatch,
    ArgNotFound,
}

impl AsmMsg for MacroMsg {

    fn msg(&self) -> &'static str {
        use MacroMsg::*;

        match self {
            NoDeclIdent =>
                "Declaration has no identifier",

            InvalidDecl =>
                "Declaration is invalid",

            NoDeclBody =>
                "Declaration has no body",

            InvalidDeclToken =>
                "Unexpected token in macro declaration",

            NoCallIdent =>
                "Macro call has no identifier",

            DeclNotFound =>
                "Declaration not found",

            ArgCountMismatch =>
                "Argument count in the call does not match the declaration",

            ArgNotFound =>
                "Argument not found in declaration",
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
            NotFound =>
                "Could not find the corresponding opcode",
        }
    }

}

/// Error variants when calculating constant values.
#[derive(Debug, Copy, Clone)]
pub enum ConstantsMsg {
    DuplicateKey,
    MisplacedMarker,
    FileReadFailed,
}

impl AsmMsg for ConstantsMsg {

    fn msg(&self) -> &'static str {
        use ConstantsMsg::*;

        match self {
            DuplicateKey =>
                "Constant's key already existed.",
            
            MisplacedMarker =>
                "The location of the marker does match its value.",

            FileReadFailed =>
                "The file to include could not be read.",
        }
    }

}