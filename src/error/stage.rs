
use crate::program::fmt;

fn stage_err<E: std::fmt::Display + Sized>(e: E, msg: &str) {
    let msg = fmt::strip().err("Compilation Failed. ").info(msg).read();
    eprintln!("{}\n{}", msg, e);
}

fn stage_err_vec<E: std::fmt::Display + Sized>(ev: Vec<E>, msg: &str) {
    let msg = fmt::strip().err("Compilation Failed. ").info(msg).read();
    eprintln!("{}\n{}", msg, ev.iter().map(|e| format!("{}\n", e)).collect::<String>());
}

pub fn clargs<E: std::fmt::Display + Sized>(e: E) {
    stage_err(e, "Invalid command line arguments.");
}

pub fn source<E: std::fmt::Display + Sized>(e: E) {
    stage_err(e, "Could not read source file.");
}

pub fn split<E: std::fmt::Display + Sized>(ev: Vec<E>) {
    stage_err_vec(ev, "Could not split words from source file.");
}

pub fn parse<E: std::fmt::Display + Sized>(ev: Vec<E>) {
    stage_err_vec(ev, "Could not parse words.");
}

pub fn ast<E: std::fmt::Display + Sized>(ev: Vec<E>) {
    stage_err_vec(ev, "Could not build the token tree.");
}

pub fn macros<E: std::fmt::Display + Sized>(ev: Vec<E>) {
    stage_err_vec(ev, "Could not expand macros.");
}

pub fn ops<E: std::fmt::Display + Sized>(ev: Vec<E>) {
    stage_err_vec(ev, "Could not find instructions.");
}
