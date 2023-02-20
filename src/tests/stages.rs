
use crate::{
    parse::{ split::Split, lex::TokenType, prepare::{ self, ParsedToken }, source::Source },
    token::{ Value, ast::{ macros::Macros, Ast }, read::TokenRef },
    write::{ ops::OpMap, constants::Constants, encode },
    program::clargs,
    error::stage,
};

use super::{ rand_file, rand_word, urand, };

#[test]
#[ignore]
fn endless() {
    loop {
        split();
        parse1();
        parse2();
        ast();
        shuffle();
    }
}

#[test]
fn split() {
    let symbols = ["TEST"];

    for _ in 0..100 {
        let source = Source::from_content(rand_file()); 
        let _ = Split::new(source.main(), &symbols);
    }
}

#[test]
fn parse1() {
    let symbols = ["TEST"];

    let iter = 40_000;
    let mut input = String::new();

    // Test with completely random words.
    for _ in 0..iter {
        input.push_str(&rand_word()); 
        input.push(' ');
    }

    let source = Source::from_content(input); 
    let split = Split::new(source.main(), &symbols).unwrap();
    let _ = prepare::parse(&split);
}

#[test]
fn parse2() {
    let symbols = ["TEST"];

    let iter = 400_000;
    let mut input = String::with_capacity(iter);

    // Test words with specific characters
    let chars = vec!['.', ':', '#', '&', '%', '(', ')', 'X', '0', ' '];

    for _ in 0..iter {
        input.push(chars[urand(chars.len() - 1)]);
    }

    let source = Source::from_content(input); 
    let split = Split::new(source.main(), &symbols).unwrap();
    let _ = prepare::parse(&split);
}

#[test]
fn ast() {
    let mut macros = Macros::new();
    let iter = 2000;

    for _ in 0..iter {
        let mut rand_ln = 1;
        let mut tokens = vec![];

        let token_count = 200;

        for _ in 0..token_count{
            let token = ParsedToken {
                ty: TokenType::at(urand(50000)),
                line_number: rand_ln,
                value: Value::Str(""),
                line: "",
                word: "",
                file: "",
            };

            tokens.push(token);

            if urand(4) == 0 {
                rand_ln += 1;
            }
        }

        let source = Source::from_content(String::new()); 
        let _ = Ast::new(tokens, &mut macros, &source);
    }
}

/// Start from a correct source and shuffle the words.
#[test]
fn shuffle() {
    for _ in 0..1000 {
        let _ = _shuffle();
    }
}

fn _shuffle() -> Result<(), ()> {
    let args = vec![
        String::new(),
        String::from("asm/hello/hello.gb.asm"),
        String::from("-o"),
        String::from("/dev/null"),
    ];

    let clargs = clargs::parse(&args).map_err(stage::clargs)?;

    let source = Source::new(clargs.path).map_err(stage::source)?;
    let split = Split::new(source.main(), &clargs.symbols).map_err(stage::split)?;
    let words = split.words();
    
    let mut shuffled = String::new();

    for i in 0..100 {
        let word = words[i % words.len()].0;
        shuffled.push_str(word);
        shuffled.push(' ');

        if urand(5) == 0 {
            shuffled.push('\n');
        }
    }

    let source = Source::from_content(shuffled);
    let split = Split::new(source.main(), &clargs.symbols).map_err(stage::split)?;

    let parsed_tokens = prepare::parse(&split).map_err(stage::parse)?;
    let mut macros = Macros::new();
    let mut ast = Ast::new(parsed_tokens, &mut macros, &source).map_err(stage::ast)?;
    macros.expand(&mut ast).map_err(stage::macros)?;
    let ast_ref = TokenRef::new(&ast);
    ast_ref.validate().map_err(stage::ast_validation)?;
    let op_map = OpMap::new(&ast_ref).map_err(stage::ops)?;
    let mut constants = Constants::new(&ast_ref, &op_map).map_err(stage::constants)?;
    let updates = constants.eval().map_err(stage::expressions)?;
    constants.update(updates);
    constants.validate(&ast_ref).map_err(stage::constants_validation)?;
    encode::build(&clargs.output(), &ast_ref, &op_map, &constants).map_err(stage::encode)?;

    Ok(())
}
