
use crate::program::fmt;

/// Prepends a newline if text is not empty.
fn ln_if(text: &str) -> String {
    return if text.is_empty() {
        "".into()
    }else {
        format!("\n{}", text)
    };
}

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

impl<'a> std::fmt::Display for SplitErr<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO same formatting as AsmErr
        let text = fmt::strip()
            .info(&format!("({:?}) ", self.ty))
            .base(&format!(
                "{}\nl{}:    {}", self.description(), self.line_number, self.line))
            .read();

        write!(f, "{}", text)
    }
}
