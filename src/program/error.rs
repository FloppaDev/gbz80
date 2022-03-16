
use crate::{
    parse::{
        lex::TokenType,
        prepare::ParsedToken,
    },
    token::{
        Token, 
        read::TokenRef
    },
    program::SourceCtx,
};

macro_rules! err {
    // err!(SomeErr, SomeErrType::NoWorky, err_ctx)
    ($ty:ty, $e:expr, $ctx:expr) => {
        <$ty>::new($e, $ctx, source!())
    }
}

/// Provides context for an error in the parsed source file.
#[derive(Debug, Copy, Clone)]
pub struct ErrCtx<'a> {
    line_number: usize,
    line: &'a str,
    word: &'a str,
}

impl<'a> ErrCtx<'a> {

    pub const fn new(
        line_number: usize,
        line: &'a str,
        word: &'a str,
    ) -> Self {
        Self{ line_number, line, word }
    }

}

impl<'a> From<&ParsedToken<'a>> for ErrCtx<'a> {

    fn from(token: &ParsedToken<'a>) -> Self {
        let ParsedToken{ line_number, line, word, .. } = *token;
        Self { line_number, line, word }
    }

}

impl<'a> From<&Token<'a>> for ErrCtx<'a> {

    fn from(token: &Token<'a>) -> Self {
        let Token{ line_number, line, word, .. } = *token;
        Self { line_number, line, word }
    }

}

impl<'a> From<&TokenRef<'a>> for ErrCtx<'a> {

    fn from(token_ref: &TokenRef<'a>) -> Self {
        let Token{ line_number, line, word, .. } = *token_ref.token();
        Self { line_number, line, word }
    }

}

//TODO impl fmt properly

#[derive(Debug, Copy, Clone)]
pub enum ClargsErrType {
    NoSource,
    NoOutput,
    TooManyParams,
    UnknownArg,
}

#[derive(Debug)]
pub struct ClargsErr<'a> {
    ty: ClargsErrType,
    msg: &'a str,
}

impl<'a> ClargsErr<'a> {

    pub const fn new(ty: ClargsErrType, msg: &'a str) -> Self {
        Self { ty, msg }
    }

    pub const fn fmt(&self) -> &'static str {
        use ClargsErrType::*;

        match self.ty {

            NoSource =>
                "NoSource: No source file specified",

            NoOutput =>
                "NoOutput: No output file specified",

            TooManyParams =>
                "TooManyParams: Too many parameters in argument",

            UnknownArg =>
                "UnknownArg: Unknown argument",

        }
    }

}

#[derive(Debug, Copy, Clone)]
pub enum SplitErrType {
    MisplacedDirective,
    InvalidDirective,
    InvalidWord,
}

#[derive(Debug)]
pub struct SplitErr<'a> {
    ty: SplitErrType,
    line: &'a str,
    line_number: usize,
}

impl<'a> SplitErr<'a> {

    pub const fn new(ty: SplitErrType, line: &'a str, line_number: usize) -> Self {
        Self { ty, line, line_number }
    }

    pub const fn fmt(&self) -> &'static str {
        use SplitErrType::*;

        match self.ty {
            MisplacedDirective =>
                "MisplacedDirective: Directives must be placed at the start of a line",

            InvalidDirective =>
                "InvalidDirective: Invalid directive name",

            InvalidWord =>
                "InvalidWord: Could not read word",
        }
    }

}

#[derive(Debug, Copy, Clone)]
pub enum ParseErrType {
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

    /// Those mean errors in Rust code.
    /// Parser cannot handle this type.
    UnhandledType,
    /// Lexicon identified word prefix wrong.
    UnexpectedPrefix,
}

#[derive(Debug)]
pub struct ParseErr<'a> {
    ty: ParseErrType,
    err_ctx: ErrCtx<'a>,
    source_ctx: SourceCtx,
}

impl<'a> ParseErr<'a> {

    pub const fn new(
        ty: ParseErrType, 
        err_ctx: ErrCtx<'a>, 
        source_ctx: SourceCtx,
    ) -> Self {
        Self { ty, err_ctx, source_ctx }
    }

    pub const fn fmt(&self) -> &'static str {
        use ParseErrType::*;

        match self.ty {
            HexOverflow => 
                "HexOverflow: Hexadecimal literal overflow",

            BinOverflow => 
                "BinOverflow: Binary literal overflow",

            DecOverflow => 
                "DecOverflow: Decimal literal overflow",

            EmptyStr => 
                "EmptryStr: Empty string",

            Invalid => 
                "Invalid: Invalid as any type",

            InvalidHex => 
                "InvalidHex: Invalid as hexadecimal literal",

            InvalidBin => 
                "InvalidBin: Invalid as binary literal",

            InvalidDec => 
                "InvalidDec: Invalid as decimal literal",

            InvalidStr => 
                "InvalidStr: Invalid as string literal",

            InvalidDirective => 
                "InvalidDirective: Invalid as directive",

            InvalidDirectiveIdent => 
                "InvalidDirectiveIdent: Invalid as directive identifier",

            InvalidMacroArgIdent => 
                "InvalidMacroArgIdent: Invalid as macro argument's identifier",

            InvalidMacroArg => 
                "InvalidMacroArg: Invalid as macro argument",

            InvalidMacroIdent => 
                "InvalidMacroCallIdent: Invalid as macro call's identifier",

            InvalidLabel =>
                "InvalidLabel: Invalid as label",

            InvalidLabelIdent =>
                "InvalidLabelIdent: Invalid as label's identifier",

            InvalidNamedMark => 
                "InvalidNamedMark: Invalid as named marker",

            InvalidNamedMarkLabel => 
                "InvalidNamedMarkLabel: Invalid as named marker's label",

            InvalidNamedMarkLabelIdent => 
                "InvalidNamedMarkLabelIdent: Invalid as named marker label's identifier",

            InvalidNamedMarkHex => 
                "InvalidNamedMarkHex: Invalid as named marker's hexadecimal literal",

            InvalidAnonMark =>
                "InvalidAnonMark: Invalid as anonymous marker",

            InvalidAnonMarkHex =>
                "InvalidAnonMarkHex: Invalid as anonymous marker's hexadecimal literal",

            InvalidIdent => 
                "InvalidIdent: Invalid as identifier",

            UnhandledType => 
                "UnhandledType: Parser could not handle the token type",

            UnexpectedPrefix => 
                "UnexpectedPrefix: Wrong type identified from prefix",
        }
    }

}

#[derive(Debug, Copy, Clone)]
pub enum AstErrType {
    NoTokens,
    UnmatchedParen,
    PlusWithoutRhs,
    MinusWithoutRhs,
    MarkWithoutLiteral,
    InvalidExprLhs(TokenType),
    NoExprLhs,
    UnhandledNewline(TokenType),
    UnknownError,
}

#[derive(Debug)]
pub struct AstErr<'a> {
    ty: AstErrType,
    err_ctx: ErrCtx<'a>,
    source_ctx: SourceCtx,
}

impl<'a> AstErr<'a> {

    pub const fn new(
        ty: AstErrType, 
        err_ctx: ErrCtx<'a>, 
        source_ctx: SourceCtx,
    ) -> Self {
        Self { ty, err_ctx, source_ctx }
    }

    pub const fn fmt(&self) -> &'static str {
        use AstErrType::*;

        match self.ty {
            NoTokens =>
                "NoTokens: No tokens were provided",

            UnmatchedParen =>
                "UnmatchedParen: Parens must come in pair",

            PlusWithoutRhs =>
                "PlusWithoutRhs: Plus operation expects a right-hand side operand",

            MinusWithoutRhs =>
                "MinusWithoutRhs: Minus operation expects a right-hand side operand",

            MarkWithoutLiteral => 
                "MarkWithoutLiteral: marker expected a literal",

            InvalidExprLhs(_) =>
                "InvalidExprLhs: Invalid token for binary operation's left-hand side",

            NoExprLhs =>
                "NoExprLhs: There is no token preceding this binary operation",

            UnhandledNewline(_) =>
                "UnhandledNewline: Internal error on new line",

            UnknownError =>
                "Unknown error",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MacroErrType {
    NoDeclIdent,
    InvalidDecl,
    NoDeclBody,
    InvalidDeclToken,
    NoCallIdent,
    DeclNotFound,
    ArgCountMismatch,
    ArgNotFound,
}

#[derive(Debug)]
pub struct MacroErr<'a> {
    ty: MacroErrType,
    err_ctx: ErrCtx<'a>,
    source_ctx: SourceCtx,
}

impl<'a> MacroErr<'a> {

    pub const fn new(
        ty: MacroErrType, 
        err_ctx: ErrCtx<'a>, 
        source_ctx: SourceCtx,
    ) -> Self {
        Self { ty, err_ctx, source_ctx }
    }

    pub const fn fmt(&self) -> &'static str {
        use MacroErrType::*;

        match self.ty {
            NoDeclIdent =>
                "NoDeclIdent: Declaration has no identifier",

            InvalidDecl =>
                "InvalidDecl: Declaration is invalid",

            NoDeclBody =>
                "NoDeclBody: Declaration has no body",

            InvalidDeclToken =>
                "InvalidDeclToken: Unexpected token in macro declaration",

            NoCallIdent =>
                "NoCallIdent: Macro call has no identifier",

            DeclNotFound =>
                "DeclNotFound: Declaration not found",

            ArgCountMismatch =>
                "ArgCountMismatch: Argument count in the call does not match the declaration",

            ArgNotFound =>
                "ArgNotFound: Argument not found in declaration",
        }
    }

}

#[derive(Debug, Copy, Clone)]
pub enum OpErrType {
    NotFound,
}

#[derive(Debug)]
pub struct OpErr<'a> {
    ty: OpErrType,
    err_ctx: ErrCtx<'a>,
    source_ctx: SourceCtx,
}

impl<'a> OpErr<'a> {

    pub const fn new(
        ty: OpErrType, 
        err_ctx: ErrCtx<'a>, 
        source_ctx: SourceCtx,
    ) -> Self {
        Self { ty, err_ctx, source_ctx }
    }

    pub const fn fmt(&self) -> &'static str {
        use OpErrType::*;

        match self.ty {
            NotFound =>
                "NotFound: Could not find the corresponding opcode",
        }
    }

}

#[derive(Debug, Copy, Clone)]
pub enum ConstantsErrType {
    DuplicateKey,
    MisplacedMarker,
    FileReadFailed,
}

#[derive(Debug)]
pub struct ConstantsErr<'a> {
    ty: ConstantsErrType,
    err_ctx: ErrCtx<'a>,
    source_ctx: SourceCtx,
}

impl<'a> ConstantsErr<'a> {

    pub const fn new(
        ty: ConstantsErrType, 
        err_ctx: ErrCtx<'a>, 
        source_ctx: SourceCtx,
    ) -> Self {
        Self { ty, err_ctx, source_ctx }
    }

    pub const fn fmt(&self) -> &'static str {
        use ConstantsErrType::*;

        match self.ty {
            DuplicateKey =>
                "DuplicateKey: Constant's key already existed.",
            
            MisplacedMarker =>
                "MisplacedMarker: The location of the marker does match its value.",

            FileReadFailed =>
                "FileReadFailed: The file to include could not be read.",
        }
    }

}
