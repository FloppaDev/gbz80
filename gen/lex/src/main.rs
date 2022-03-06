
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

    fmt_parents(
        tree, &tree.nodes[types.children[0]], &mut parent_type, &mut 0);

    parent_type.push_str("_ => unreachable!()");
    parent_type = tab(2, &parent_type);

    let has_value = tree.find("has_value");
    let mut values = String::new();

    fmt_values(tree, has_value, &mut values, &mut 0);
    values = tab(2, &values);

    //let _are_words = tree.find("are_words");
    //let _word_pairs = tree.find("word_pairs");
    //let _prefix_pairs = tree.find("prefix_pairs");
    //let _end_on_newline = tree.find("end_on_newline");

    let result = template
        .replace(&key("no_touchy"), NO_TOUCHY)
        .replace(&key("token_types"), token_types.trim_end())
        .replace(&key("parent_type"), parent_type.trim_end())
        //.replace(&key("argument_type"), &argument_type)
        .replace(&key("has_value"), &values.trim_end());
        //.replace(&key("ends_on_newline"), &ends_on_newline)
        //.replace(&key("get_by_word"), &get_by_word)
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

    for (i, line) in s.lines().enumerate() {
        if !line.trim().is_empty() && i > 0 {
            result.push_str(&(0..n).map(|_| "    ").collect::<String>()); 
        }

        result.push_str(line);
        result.push_str("\n");
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

fn fmt_values(
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

