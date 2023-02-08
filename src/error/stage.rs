
#![allow(clippy::needless_pass_by_value)]

use crate::program::fmt;

fn stage_err<E: std::fmt::Display + Sized>(e: E, msg: &str) {
    let msg = fmt::strip().err("Compilation Failed.\n\n    ").info(msg).read();
    eprintln!("{e}\n{msg}\n");
}

fn stage_err_vec<E: std::fmt::Display + Sized>(ev: Vec<E>, msg: &str) {
    let f = |(i, e)| if i != ev.len() - 1 {
        format!("{e}\n")
    }else {
        format!("{e}")
    };

    let msg = fmt::strip()
        .err("Build failed with")
        .bold(&format!(" {} ", ev.len()))
        .err("errors:\n\n    ")
        .info(msg)
        .read();

    eprintln!("{}\n{}\n", ev.iter().enumerate().map(f).collect::<String>(), msg);
}

macro_rules! stage_err { ($fn:ident, $lit:literal) => {
    pub fn $fn<E: std::fmt::Display + Sized>(e: E) { stage_err(e, $lit); }   
}}

macro_rules! stage_err_vec { ($fn:ident, $lit:literal) => {
    pub fn $fn<E: std::fmt::Display + Sized>(ev: Vec<E>) { stage_err_vec(ev, $lit); }
}}

stage_err!(clargs, "Invalid command line arguments.");
stage_err!(source, "Could not read source file.");
stage_err_vec!(split, "Could not split words from source file.");
stage_err_vec!(parse, "Could not parse words.");
stage_err_vec!(ast, "Could not build the token tree.");
stage_err_vec!(macros, "Could not expand macros.");
stage_err_vec!(ops, "Could not find instructions.");
stage_err_vec!(validation, "Could not validate the token tree.");
stage_err!(constants, "Could not collect constants.");
stage_err_vec!(expressions, "Could not evaluate expressions in constants.");
