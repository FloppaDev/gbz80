
use crate::{
    error::init::{ClargsErr, ClargsErrType::*},
};

#[derive(Default)]
/// Contains the parsed arguments.
pub struct Clargs<'a> {
    pub path: &'a str,
    pub symbols: Vec<&'a str>,
    pub output: Option<&'a str>,
}

impl<'a> Clargs<'a> {

    pub fn output(&self) -> String {
        self.output.map_or_else(|| {
            "./out.gb".into()
        }, |output| {
            output.into()
        })
    }

}

/// Get CLI arguments into a struct.
///
/// List of arguments:
/// source file
/// -o                      Output file
/// -D [SYMBOLS]            Defined symbols, optional.
pub fn parse(args: &[String]) -> Result<Clargs, ClargsErr> {
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
