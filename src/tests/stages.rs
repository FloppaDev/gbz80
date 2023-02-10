
use crate::{
    parse::{ split::Split, lex::TokenType, prepare::{ self, ParsedToken }, source::Source },
    token::{ Value, ast::{ Ast, macros::Macros, } },
};

use super::{ rand_file, rand_word, urand, };

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
