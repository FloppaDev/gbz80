
/// Provide all error types needed throughout the compilation.
pub mod error;

use crate::{
    program::error::{ ClargsErr, ClargsErrType },
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
        return Err(ClargsErr::new(ClargsErrType::NoSource, ""))
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
                            return Err(ClargsErr::new(ClargsErrType::TooManyParams, arg))
                        }
                    }

                    Ty::Unknown => {
                        return Err(ClargsErr::new(ClargsErrType::UnknownArg, arg))
                    }
                }
            }
        }
    }

    if clargs.output.is_none() {
        return Err(ClargsErr::new(ClargsErrType::NoOutput, ""))
    }

    Ok(clargs)
}

#[cfg(debug_assertions)]
pub fn title(title: &str) {
    let decoration = "=".repeat(79);
    println!("\n{}\n\t\t\t\t{}\n{}\n", decoration, title, decoration);
}
