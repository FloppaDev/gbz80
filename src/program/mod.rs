
/// Provide all error types needed throughout the compilation.
#[macro_use]
pub mod error;

use crate::{
    program::error::{ ClargsErr, ClargsErrType::* },
};

pub const RECURSION_LIMIT: usize = 1000;

#[derive(Default)]
/// Contains the parsed arguments.
pub struct Clargs<'a> {
    pub path: &'a str,
    pub symbols: Vec<&'a str>,
    pub output: Option<&'a str>,
}

/// Get CLI arguments into a struct.
///
/// List of options:
///
/// -o                      Output file
/// -D [SYMBOLS]            Defined symbols
pub fn clargs(args: &[String]) -> Result<Clargs, ClargsErr> {
    enum Ty { Unknown, Define, Output }

    let mut clargs = Clargs::default();

    if args.len() >= 2 { 
        clargs.path = &args[1];
    }

    else { 
        return Err(ClargsErr::new(NoSource, ""))
    }

    // No more arguments.
    if args.len() == 2 { 
        return Ok(clargs)
    }

    let mut ty = Ty::Unknown;

    for arg in &args[2..] {
        match arg.as_str() {
            "-D" => ty = Ty::Define,

            "-o" => ty = Ty::Output,

            _ => {
                match ty {
                    Ty::Define => clargs.symbols.push(arg),

                    Ty::Output => {
                        if clargs.output.is_none() {
                            clargs.output = Some(arg);
                        }else{
                            return Err(ClargsErr::new(TooManyParams, arg))
                        }
                    }

                    Ty::Unknown => {
                        return Err(ClargsErr::new(UnknownArg, arg))
                    }
                }
            }
        }
    }

    if clargs.output.is_none() {
        return Err(ClargsErr::new(NoOutput, ""))
    }

    Ok(clargs)
}

#[cfg(debug_assertions)]
pub fn title(title: &str) {
    let decoration = "=".repeat(79);
    println!("\n{}\n\t\t\t\t{}\n{}\n", decoration, title, decoration);
}

macro_rules! source {
    () => {
        crate::program::SourceCtx{ 
            file: file!(),
            line: line!(),
            column: column!(),
        }
    }
}

/// Stores a location in the source code.
#[derive(Debug)]
pub struct SourceCtx {
    pub file: &'static str,
    pub line: u32,
    pub column: u32,
}

pub mod color {

    #[cfg(target_family="unix")]
    mod values {
        pub const BASE: &'static str = "\x1b[0m";
        pub const OK: &'static str = "\x1b[32m";
        pub const ERR: &'static str = "\x1b[31m";
        pub const INFO: &'static str = "\x1b[93m";
    }

    #[cfg(not(target_family="unix"))]
    mod values {
        pub const BASE: &'static str = "";
        pub const OK: &'static str = "";
        pub const ERR: &'static str = "";
        pub const INFO: &'static str = "";
    }

    pub fn strip() -> Strip {
        Strip{ value: "".into() }
    }

    pub struct Strip {
        value: String,
    }

    impl Strip {

        pub fn base(mut self, text: &str) -> Self {
            self.value.push_str(text);
            self
        }

        pub fn ok(mut self, text: &str) -> Self {
            self.value.push_str(values::OK);
            self.value.push_str(text);
            self.value.push_str(values::BASE);
            self
        }

        pub fn err(mut self, text: &str) -> Self {
            self.value.push_str(values::ERR);
            self.value.push_str(text);
            self.value.push_str(values::BASE);
            self
        }

        pub fn info(mut self, text: &str) -> Self {
            self.value.push_str(values::INFO);
            self.value.push_str(text);
            self.value.push_str(values::BASE);
            self
        }

        pub fn end(mut self) -> String {
            self.value
        }


    }

}


