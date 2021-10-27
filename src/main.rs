#[macro_use]
mod utils;

mod split;
mod ast;
mod opcodes;
mod encode;

// Compilation will follow these steps:
//
// - Split file into lines, and lines into words.
//      .Words can be delimited by:
//          .spaces
//          .newlines
//          .expressions characters ') ( + -'
//          .macro parameters delimiter '.'
//          .double quotes '"', string literals count as one word
//      .Line comments are ignored during this step.
//      .#if directives are also treated then.
//
// - Words are parsed into basic tokens, wich will simplify checking
// the syntax and the structure of the code.
//      .e.g.   &0143 becomes LIT_HEX: "0143"
//              A literal wich is an hexadecimal number
//      .At this point the values in the tokens are still strings
//
// - Tokens are put into more generic containers, forming a tree.
// This is an AST (Abstract Token Tree)
//
// - Check the tree for syntax errors.
//
// - Validate fences
//      .i.e fence &0104: code before the fence cannot exceed &0104.
//
// - Validate Nintendo logo and calculate checksums

fn main() {
    // Get source file
    let path = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => abort("No file specified")
    };

    let input = match std::fs::read_to_string(path) {
        Ok(file) => file,
        Err(_) => abort("File not found")
    };

    //TODO
    let symbols = vec![];

    // Parse file and output a token tree
    let split = split::Split::new(&input, &symbols);
    let ast = ast::Token::make_ast(split);
}

pub fn abort(e: &str) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}
