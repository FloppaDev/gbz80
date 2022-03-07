
//TODO update tests

use crate::lex::{self, TokenType}; 
use crate::parse::prepare::{ self, ParsedToken };
use crate::program;
use crate::token::Ast;
use crate::token::macros::Macros;
use std::thread;
use std::fs::File;
use std::io::Read;

#[test]
fn split() {
    threaded(| | {
        let symbols = ["TEST"];

        for _ in 0..100 {
            let input = rand_file(); 
            let split = super::split::split(&input, &symbols);
        }
    });

    return;
}

#[test]
fn parse1() {
    threaded(| | {
        let lexicon = Lexicon::new();
        let mut data = Data::new();
        let symbols = ["TEST"];

        let iter = 10_000;
        let mut input = String::new();

        // Test with completely random words.
        for _ in 0..iter {
            input.push_str(&rand_word()); 
            input.push(' ');
        }

        let split = super::split::split(&input, &symbols).unwrap();
        let _ = parse::parse(&lexicon, &mut data, &split);
    });
}

#[test]
fn parse2() {
    threaded(| | {
        let lexicon = Lexicon::new();
        let mut data = Data::new();
        let symbols = ["TEST"];

        let iter = 100_000;
        let mut input = String::with_capacity(iter);

        // Test words with specific characters
        let chars = vec!['.', ':', '#', '&', '%', '(', ')', 'X', '0', ' '];

        for _ in 0..iter {
            input.push(chars[urand(chars.len() - 1)]);
        }

        let split = super::split::split(&input, &symbols).unwrap();
        let _ = parse::parse(&lexicon, &mut data, &split);
    });
}

#[test]
fn ast() {
    threaded(| | {
        let lexicon = Lexicon::new();
        let mut macros = Macros::new();
        let ty_max = lexicon.type_count();

        let iter = 1000;

        for _ in 0..iter {
            let mut rand_ln = 1;
            let mut tokens = vec![];

            let token_count = 100;

            for _ in 0..token_count{
                let token = ParsedToken {
                    ty: lexicon.get_type_at(urand(ty_max)),
                    data_key: Key::Void,
                    line_number: rand_ln,
                    line: "",
                    word: "",
                };

                tokens.push(token);

                if urand(4) == 0 {
                    rand_ln += 1;
                }
            }

            let ast = Ast::new(&lexicon, tokens, &mut macros);
        }
    });
}

//TODO remove, not closs-platform
/// Run the function on multiple threads.
fn threaded(f: fn() -> ()) {
    let mut handlers = vec![];

    for t in 0..process::thread_count() {
        handlers.push(thread::spawn(f));
    }

    for handler in handlers {
        handler.join().unwrap();
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
