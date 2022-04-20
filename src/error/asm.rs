
use crate::{
    program::fmt,
    error::{ErrCtx, SourceCtx},
    parse::lex::TokenType::*,
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
        let ErrCtx{ ty, line_number, line, word } = self.err_ctx;

        let mut strip = fmt::strip()
            .debug(&format!("{}\n", self.source_ctx)) 
            .info(&format!("({:?}) ", self.ty))
            .bold(&format!("{} ", self.ty.msg()))
            .faint(&format!("l{}:", line_number ))
            .faint(&(if ty == Unknown { "".into() } else { format!("({:?})", ty) }));

        if let Some(word_start) = self.err_ctx.word_start() {
            let line_a = line.get(..word_start).unwrap();
            let line_word = line.get(word_start..word_start+word.len()).unwrap();
            let line_b = line.get(word_start+word.len()..).unwrap();

            strip = strip
                .faint(&format!("\n    {}", line_a))
                .err(&format!("{}", line_word))
                .faint(&format!("{}\n", line_b));
        }

        else {
            if line_number == 13 {
                dbg!(word);
            }
            strip = strip.faint(&format!("\n    {}\n", line))
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
    ReservedKeyword,
    //TODO those are bugs
    UnhandledType,
    UnexpectedPrefix,
}

impl AsmMsg for ParseMsg {

    fn msg(&self) -> &'static str {
        use ParseMsg::*;

        match self {
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

            ReservedKeyword =>
                "Identifier cannot be a reserved keyword",

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
    MarkWithoutLiteral,
    UnaryWithoutRhs,
    BinaryWithoutLhs,
    BinaryWithoutRhs,
    //TODO assembler bug
    UnhandledNewline,
}

impl AsmMsg for AstMsg {

    fn msg(&self) -> &'static str {
        use AstMsg::*;

        match self {
            NoTokens =>
                "No tokens were provided",

            UnmatchedParen =>
                "Parens must come in pair",

            MarkWithoutLiteral => 
                "marker expected a literal",

            UnaryWithoutRhs =>
                "Unary operator expected an operand on its right",

            BinaryWithoutLhs =>
                "Binary operator expected an operand on its left",

            BinaryWithoutRhs =>
                "Binary operator expected an operand on its right",

            UnhandledNewline =>
                "Internal error on new line",
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
    //TODO ArgNotFound,
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

            //ArgNotFound =>
                //"Argument not found in declaration",
        }
    }

}

/// Error variants when validating the Ast.
#[derive(Debug, Copy, Clone)]
pub enum ValidationMsg {
    InvalidParent,
}

impl AsmMsg for ValidationMsg {

    fn msg(&self) -> &'static str {
        use ValidationMsg::*;

        match self {
            InvalidParent =>
                "Parent of the token is of an unexpected type",
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
                "Constant's key already existed",
            
            MisplacedMarker =>
                "The location of the marker does match its value",

            FileReadFailed =>
                "The file to include could not be read",
        }
    }

}

/// Error variants when evaluating expressions.
#[derive(Debug, Copy, Clone)]
pub enum ExprMsg {
    StrInExpr,
    TooManyChildren,
    ConstantNotFound,
    CircularDependency,
}

impl AsmMsg for ExprMsg {

    fn msg(&self) -> &'static str {
        use ExprMsg::*;

        match self {
            StrInExpr =>
                "String literal not allowed in expressions, unless it is completely alone.",

            TooManyChildren =>
                "The token is not alone in its scope.",

            ConstantNotFound =>
                "The constant does not exist.",

            CircularDependency => 
                "Infinite loop, the constant's dependencies depend on the constant itself.",
        }
    }

}
