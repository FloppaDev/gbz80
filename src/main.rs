#[macro_use]
mod utils;

mod split;
mod ast;
mod opcodes;
mod encode;
mod validation;

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
// - Macros are expanded.
//
// - Check the tree for syntax errors.
//
// - Validate markers. 
//      .i.e marker &0104: code before the marker cannot exceed &0104.
//
// - Validate Nintendo logo and calculate checksums
//
// - Encode result into the output.

fn main() {
    let args = clargs();

    let input = match std::fs::read_to_string(args.path) {
        Ok(file) => file,
        Err(_) => abort("File not found")
    };

    let split = split::Split::new(&input, &args.symbols);
    let ast = ast::Token::make_ast(split);

    let instructions = opcodes::get_instructions();

    if !validation::check(&ast, &instructions) {
        eprintln!("\x1b[0;31mCompilation failed at syntax validation.\x1b[0m");
        std::process::exit(1);
    }

    encode::build(ast, instructions);
}

pub fn abort(e: &str) -> ! {
    eprintln!("{}", e);
    std::process::exit(1);
}

#[derive(Default)]
struct Args {
    path: String,
    symbols: Vec<String>,
    output: Option<String>,
}

fn clargs() -> Args {
    let mut args = Args::default();

    let words = std::env::args().collect::<Vec<_>>();
    args.path = if words.len() >= 2 { 
        words[1].to_string() 
    }else { 
        abort("No file specified")
    };
    if words.len() == 2 { return args }

    enum Ty { Unknown, Define, Output }
    let mut ty = Ty::Unknown;

    for word in &words[2..] {
        match word.as_str() {
            "-D" => ty = Ty::Define,
            "-o" => ty = Ty::Output,
            _ => {
                match ty {
                    Ty::Define => args.symbols.push(word.to_string()),
                    Ty::Output => if args.output.is_none() {
                        args.output = Some(word.to_string());
                    }else{
                        abort("Only one output can be specified");
                    }
                    Ty::Unknown => abort(&format!("Unknown argument: '{}'", word)),
                }
            }
        }
    }

    args
}
