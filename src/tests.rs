
use crate::{
    parse::{
        split::Split,
        lex::TokenType,
        prepare::{ self, ParsedToken },
    },
    token::{
        Value,
        ast::Ast,
        macros::Macros,
    },
};

use std::fs::File;
use std::io::Read;

#[test]
fn split() {
    let symbols = ["TEST"];

    for _ in 0..100 {
        let input = rand_file(); 
        let split = Split::new(&input, &symbols);
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

    let split = Split::new(&input, &symbols).unwrap();
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

    let split = Split::new(&input, &symbols).unwrap();
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
            };

            tokens.push(token);

            if urand(4) == 0 {
                rand_ln += 1;
            }
        }

        let ast = Ast::new(tokens, &mut macros);
    }
}

/// Random number between 0 and max.
fn urand(max: usize) -> usize {
    let mut buffer = [0u8; 4];
    let mut f = File::open("/dev/urandom").unwrap();
    f.read_exact(&mut buffer).unwrap();

    let mut rand = buffer[0] as usize;
    rand += (buffer[1] as usize) << 8;
    rand += (buffer[2] as usize) << 16;
    rand += (buffer[3] as usize) << 24;

    rand % max
}

fn rand_file() -> String {
    rand(&mut [0u8; 10_000])
}

fn rand_word() -> String {
    rand(&mut [0u8; 10])
}

/// Create a string from random bytes.
fn rand(buffer: &mut [u8]) -> String {
    let mut f = File::open("/dev/urandom").unwrap();
    f.read_exact(buffer).unwrap();

    let mut input = String::with_capacity(buffer.len());

    for c in buffer {
        let mut c = *c as char;

        if c == '\0' {
            c = ' ';
        }

        input.push(c);
    }

    input
}
