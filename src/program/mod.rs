
/// Provide all error types needed throughout the compilation.
pub mod error;

use crate::{
    parse::text::charset,
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

    //TODO use try.
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

/// Try to get the number of threads from 'proc/cpuinfo' (Linux only).
/// Returns 4 if it fails.
pub fn thread_count() -> usize {
    #[cfg(not(target_os = "linux"))] return 4;

    // Get the file.
    let cpuinfo = std::fs::read_to_string("/proc/cpuinfo");
    if cpuinfo.is_err() { 
        return 4 
    }

    // Find the first 'siblings' line
    let cpuinfo = cpuinfo.unwrap();
    let siblings_i = cpuinfo.find("siblings");
    if siblings_i.is_none() { 
        return 4 
    }

    // Slice from 'siblings' to the end.
    let cpuinfo = cpuinfo.get(siblings_i.unwrap() ..);
    if cpuinfo.is_none() {
        return 4
    }

    let cpuinfo = cpuinfo.unwrap();
    let mut num = 0;

    // Find and parse the number.
    for (i, c) in cpuinfo.chars().enumerate() {
        if num == 0 && charset::is_char_num(c) {
            num = i;
        }
        
        else if num != 0 && charset::is_new_line(c) || charset::is_space(c) {
            if let Some(dec) = cpuinfo.get(num..i) {
                if let Ok(dec) = dec.parse::<usize>() {
                    return dec
                }
            }
        }
    }

    4
}

#[cfg(debug_assertions)]
pub fn title(title: &str) {
    let decoration = "=".repeat(79);
    println!("\n{}\n\t\t\t\t{}\n{}\n", decoration, title, decoration);
}
