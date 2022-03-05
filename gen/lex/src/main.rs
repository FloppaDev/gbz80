
mod parse;

use crate::{
    parse::{split, Tree},
};

const NO_TOUCHY: &'static str = "\
// File generated automatically
//  - templates in 'gen/lex/data'
//  - code in 'gen/lex/src'
//
// Do no edit manually.";

fn main() {
    let text = include_str!("../data/types.gen");
    let split = split(text);
    let tree = Tree::new(&split);

    //tree.debug();

    build(&tree);
}

fn build(tree: &Tree) {
    let template = include_str!("../data/lex.rs");

    let types = tree.find("types");
    let mut token_types = String::new();
    tree.fmt_node(types, 0, &mut token_types);
    token_types.drain(token_types.len() - 2 ..);

    //let _are_words = tree.find("are_words");
    //let _word_pairs = tree.find("word_pairs");
    //let _prefix_pairs = tree.find("prefix_pairs");
    //let _have_no_value = tree.find("have_no_value");
    //let _end_on_newline = tree.find("end_on_newline");

    let result = template
        .replace(&key("no_touchy"), NO_TOUCHY)
        .replace(&key("token_types"), &token_types);
        //.replace(&key("parent_type"), parent_type)
        //.replace(&key("argument_type"), argument_type)
        //.replace(&key("has_value"), has_value)
        //.replace(&key("ends_on_newline"), ends_on_newline)
        //.replace(&key("get_by_word"), get_by_word)
        //.replace(&key("get_by_prefix"), get_by_prefix);

    println!("{}", result);
}

fn key(name: &str) -> String {
    format!("//[[{}]]", name)
}
