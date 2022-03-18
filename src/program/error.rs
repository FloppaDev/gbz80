
#![allow(unused_mut, unused_variables, dead_code)] //TODO

use crate::{
    parse::{
        lex::TokenType,
        prepare::ParsedToken,
    },
    token::{
        Token, 
        read::TokenRef
    },
    program::fmt,
};

/// Used mostly in recursion fail-safes.
pub const ITERATION_LIMIT: usize = 1000;

/// Contains errors messages for the main compilation stages.
#[macro_use]
pub mod stage {
    
    use crate::program::fmt;

    /// Prints an error from the assembler.
    pub fn log_err<E: std::fmt::Display>(msg: &str, err: E) {
        eprintln!("{}\n{}", msg, err);
    }

    /// Prints the error message for a specific stage.
    /// Arguments:
    /// - the stage constant
    macro_rules! stage_err {
        ($stage:expr) => {
            |e| crate::program::error::stage::log_err(&$stage(), e)
        }
    }

    pub const CLARGS: fn () -> String = | | fmt::strip()
        .err("Compilation Failed. ")
        .info("Invalid command line arguments.")
        .read();

    pub const SPLIT: fn () -> String = | | fmt::strip()
        .err("Compilation Failed. ")
        .info("Could not recognize words in source file.")
        .read();

}

/// Creates a `SourceCtx` containing its location in the source code.
/// No arguments.
macro_rules! source {
    () => {
        crate::program::error::SourceCtx{ 
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }
}

#[derive(Debug)]
/// Stores a location in the source code.
pub struct SourceCtx {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

impl std::fmt::Display for SourceCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self{ file, line, column } = self;
        write!(f, "{}", format!("{}:{}:{}", file, line, column))
    }
}

/// Creates an error for one of the types that expect an `ErrCtx`.
/// Arguments:
/// - Error struct type
/// - Variant from the corresponding enum type
/// - `ErrCtx` object
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

/// Prepends a newline if text is not empty.
fn ln_if(text: &str) -> String {
    return if text.is_empty() {
        "".into()
    }else {
        format!("\n{}", text)
    };
}

//TODO impl fmt properly

#[derive(Debug, Copy, Clone)]
/// Error variants when parsing command line arguments.
pub enum ClargsErrType {
    NoSource,
    NoOutput,
    TooManyParams,
    UnknownArg,
}

#[derive(Debug)]
/// Command line arguments error.
pub struct ClargsErr<'a> {
    ty: ClargsErrType,
    msg: &'a str,
}

impl<'a> ClargsErr<'a> {

    pub const fn new(ty: ClargsErrType, msg: &'a str) -> Self {
        Self { ty, msg }
    }

    pub const fn description(&self) -> &'static str {
        use ClargsErrType::*;

        match self.ty {
            NoSource =>
                "No source file specified",

            NoOutput =>
                "No output file specified",

            TooManyParams =>
                "Too many parameters in argument",

            UnknownArg =>
                "Unknown argument",
        }
    }

}

impl<'a> std::fmt::Display for ClargsErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = fmt::strip()
            .info(&format!("({:?}) ", self.ty))
            .base(&format!("{}{}", self.description(), ln_if(self.msg)))
            .read();

        write!(f, "{}", text)
    }
}

#[derive(Debug, Copy, Clone)]
/// Error variants when reading word from the source file.
pub enum SplitErrType {
    MisplacedDirective,
    InvalidDirective,
    InvalidWord,
}

#[derive(Debug)]
/// Error when reading words from the source file.
pub struct SplitErr<'a> {
    ty: SplitErrType,
    line: &'a str,
    line_number: usize,
}

impl<'a> SplitErr<'a> {

    pub const fn new(ty: SplitErrType, line: &'a str, line_number: usize) -> Self {
        Self { ty, line, line_number }
    }

    pub const fn description(&self) -> &'static str {
        use SplitErrType::*;

        match self.ty {
            MisplacedDirective =>
                "Directives must be placed at the start of a line",

            InvalidDirective =>
                "Invalid directive name",

            InvalidWord =>
                "Could not read word",
        }
    }

}

//TODO split returns a vec of errors.
impl<'a> std::fmt::Display for SplitErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = fmt::strip()
            .info(&format!("({:?}) ", self.ty))
            .base(&format!(
                "{}\nl{}:    {}", self.description(), self.line_number, self.line))
            .read();

        write!(f, "{}", text)
    }
}

#[derive(Debug, Copy, Clone)]
/// Error variants when parsing values from the source file.
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

//TODO AsmErr<ErrType>
#[derive(Debug)]
/// Error when parsing values from the source file.
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

    pub const fn description(&self) -> &'static str {
        use ParseErrType::*;

        match self.ty {
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

impl<'a> std::fmt::Display for ParseErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ErrCtx{ line_number, line, word } = self.err_ctx;

        let text = fmt::strip()
            .debug(&format!("{}", self.source_ctx)) 
            .info(&format!("({:?}) ", self.ty))
            .base(&format!("{}\n", self.description()))
            .faint(&format!("l{}:", line_number ))
            .bold(&format!("{}    ", word))
            .base(line)
            .read();

        write!(f, "{}", text)
    }
}

#[derive(Debug, Copy, Clone)]
/// Error variants when building the AST.
pub enum AstErrType {
    NoTokens,
    UnmatchedParen,
    PlusWithoutRhs,
    MinusWithoutRhs,
    MarkWithoutLiteral,
    UnhandledNewline(TokenType),
    UnknownError,
}

#[derive(Debug)]
/// Error when building the AST.
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

    pub const fn description(&self) -> &'static str {
        use AstErrType::*;

        match self.ty {
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

            UnhandledNewline(_) =>
                "Internal error on new line",

            UnknownError =>
                "Unknown error",
        }
    }
}

#[derive(Debug, Copy, Clone)]
/// Error variants when expanding macros.
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
/// Error when expanding macros.
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

    pub const fn description(&self) -> &'static str {
        use MacroErrType::*;

        match self.ty {
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

#[derive(Debug, Copy, Clone)]
/// Error variants when looking for an opcode.
pub enum OpErrType {
    NotFound,
}

#[derive(Debug)]
/// Error when looking for an opcode.
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

    pub const fn description(&self) -> &'static str {
        use OpErrType::*;

        match self.ty {
            NotFound =>
                "Could not find the corresponding opcode",
        }
    }

}

#[derive(Debug, Copy, Clone)]
/// Error variants when calculating constant values.
pub enum ConstantsErrType {
    DuplicateKey,
    MisplacedMarker,
    FileReadFailed,
}

#[derive(Debug)]
/// Error when calculating constant values.
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
                "Constant's key already existed.",
            
            MisplacedMarker =>
                "The location of the marker does match its value.",

            FileReadFailed =>
                "The file to include could not be read.",
        }
    }

}
