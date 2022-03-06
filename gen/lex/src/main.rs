
mod parse;

use crate::{
    parse::{split, Tree, Node},
};

use std::fs::File;
use std::io::prelude::*;

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
    fmt_types(tree, types, 0, &mut token_types);

    let mut parent_type = String::new();
    let root = &tree.nodes[types.children[0]];
    fmt_parents(tree, root, &mut parent_type, &mut 0);
    parent_type.push_str("_ => unreachable!()");
    parent_type = tab(2, &parent_type);

    let has_value = tree.find("has_value");
    let mut values = String::new();
    fmt_match_true(tree, has_value, &mut values, &mut 0);
    values = tab(2, &values);

    let ends_on_newline = tree.find("ends_on_newline");
    let mut newline = String::new();
    fmt_match_true(tree, ends_on_newline, &mut newline, &mut 0);
    newline = tab(2, &newline);

    let are_words = tree.find("are_words");
    let word_pairs = tree.find("word_pairs");
    let mut words = String::new();
    fmt_words(tree, are_words, word_pairs, &mut words);
    words = tab(2, &words);

    //let _end_on_newline = tree.find("end_on_newline");

    let result = template
        .replace(&key("no_touchy"), NO_TOUCHY)
        .replace(&key("token_types"), token_types.trim_end())
        .replace(&key("parent_type"), parent_type.trim_end())
        //.replace(&key("argument_type"), &argument_type)
        .replace(&key("has_value"), values.trim_end())
        .replace(&key("ends_on_newline"), newline.trim_end())
        .replace(&key("get_by_word"), words.trim_end());
        //.replace(&key("get_by_prefix"), &get_by_prefix);

    println!("{}", result);

    let mut file = File::create("../../src/lex.rs").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}

fn key(name: &str) -> String {
    format!("//[[{}]]", name)
}

fn tab(n: usize, s: &str) -> String {
    let mut result = String::new();
    let lines = s.lines().collect::<Vec<_>>();

    for (i, line) in lines.iter().enumerate() {
        if !line.trim().is_empty() && i > 0 {
            result.push_str(&(0..n).map(|_| "    ").collect::<String>()); 
        }

        result.push_str(line);
        if i != lines.len() - 1 {
            result.push_str("\n");
        }
    }

    result
}

pub fn fmt_types(
    tree: &Tree, 
    node: &Node, 
    indent: usize, 
    out: &mut String
) {
    let tab = (0..indent).map(|_| "    ").collect::<String>();

    for index in &node.children {
        let child = &tree.nodes[*index];
        out.push_str(&format!("{}{}\n", tab, &child.value)); 

        fmt_types(tree, child, indent + 1, out)
    }
}

fn fmt_parents(
    tree: &Tree, 
    node: &Node, 
    out: &mut String, 
    ln_start: &mut usize
) {
    if node.children.is_empty() {
        return;
    }

    for (i, index) in node.children.iter().enumerate() {
        if out.len() - *ln_start >= 60 {
            out.push_str("\n"); 
            *ln_start = out.len();
        }

        let child = &tree.nodes[*index];
        out.push_str(&child.value);

        if i != node.children.len() - 1 {
            out.push_str("|");
        }
    }

    out.push_str(&format!(" => {},\n\n", node.value));
    *ln_start = out.len() - 1;

    for index in &node.children {
        let child = &tree.nodes[*index];
        fmt_parents(tree, child, out, ln_start);
    }
}

fn fmt_match_true(
    tree: &Tree, 
    node: &Node, 
    out: &mut String, 
    ln_start: &mut usize
) {
    for (i, index) in node.children.iter().enumerate() {
        if out.len() - *ln_start >= 60 {
            out.push_str("\n"); 
            *ln_start = out.len();
        }

        let child = &tree.nodes[*index];
        out.push_str(&child.value);

        if i != node.children.len() - 1 {
            out.push_str("|");
        }
    }

    out.push_str(" => true,\n\n");
}

fn fmt_words(tree: &Tree, words: &Node, pairs: &Node, out: &mut String) {
    for (i, index) in words.children.iter().enumerate() {
        let child = &tree.nodes[*index];

        out.push_str(
            &format!("\"{}\" => {},\n", child.value.to_lowercase(), child.value));
    }

    for (i, index) in pairs.children.iter().enumerate() {
        let child = &tree.nodes[*index];

        let word = &tree.nodes[child.children[0]].value;
        out.push_str(&format!("\"{}\" => {},\n", word, child.value));
    }
}
