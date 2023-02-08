
use crate::program::fmt;

/// Error variants when parsing command line arguments.
#[derive(Debug, Copy, Clone)]
pub enum ClargsErrType {
    NoSource,
    NoOutput,
    TooManyParams,
    UnknownArg,
}

/// Command line arguments error.
#[derive(Debug)]
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
            NoSource => "No source file specified",
            NoOutput => "No output file specified",
            TooManyParams => "Too many parameters in argument",
            UnknownArg => "Unknown argument",
        }
    }

}

impl<'a> std::fmt::Display for ClargsErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = fmt::strip()
            .info(&format!("({:?}) ", self.ty))
            .base(&format!("{}{}", self.description(), fmt::ln_if(self.msg)))
            .read();

        write!(f, "{text}")
    }
}

/// Error variants when reading word from the source file.
#[derive(Debug, Copy, Clone)]
pub enum SplitErrType {
    MisplacedDirective,
    BadDirective,
    BadWord,
}

/// Error when reading words from the source file.
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

    pub const fn description(&self) -> &'static str {
        use SplitErrType::*;

        match self.ty {
            MisplacedDirective => "Directives must be placed at the start of a line",
            BadDirective => "Bad directive name",
            BadWord => "Could not read word",
        }
    }

}

impl<'a> std::fmt::Display for SplitErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = &format!("({:?}) ", self.ty);
        let msg = &format!("{}\nl{}:    {}", self.description(), self.line_number, self.line);
        let text = fmt::strip().info(ty).bold(msg).read();

        write!(f, "{text}")
    }
}
