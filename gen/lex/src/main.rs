
mod parse;

use crate::{
    parse::{split, Tree, Node},
};

use std::fs::File;
use std::io::prelude::*;

const NO_TOUCHY: &str = "\
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
    let mut types_str= String::new();
    fmt_types(tree, types, 0, &mut types_str);

    let mut parent_type = String::new();
    let root = &tree.nodes[types.children[0]];
    fmt_parents(tree, root, &mut parent_type, &mut 0);
    parent_type = tab(3, &parent_type);

    let has_value = tree.find("has_value");
    let mut has_value_str = String::new();
    fmt_match_true(tree, has_value, &mut has_value_str, &mut 0);
    has_value_str = tab(3, &has_value_str);

    let ends_on_newline = tree.find("ends_on_newline");
    let mut ends_on_newline_str = String::new();
    fmt_match_true(tree, ends_on_newline, &mut ends_on_newline_str, &mut 0);
    ends_on_newline_str = tab(3, &ends_on_newline_str);

    let char_words = tree.find("char_words");
    
    let are_words = tree.find("are_words");
    let word_pairs = tree.find("word_pairs");
    let mut words = String::new();
    fmt_words(tree, are_words, word_pairs, char_words, &mut words);
    words = tab(3, &words);

    let prefixes = tree.find("prefixes");
    let mut prefixes_str = String::new();
    fmt_prefixes(tree, prefixes, &mut prefixes_str);
    prefixes_str = tab(3, &prefixes_str);

    let mut at_str = String::new(); 
    let mut at_arms = String::new();
    let mut len = 0;
    fmt_at(tree, root, &mut at_arms, &mut len);
    at_str.push_str(&format!("const COUNT: usize = {};\n\n", len));
    at_str.push_str("match index % COUNT {\n");
    at_str.push_str(&at_arms);
    at_str.push_str("    _ => bug!()\n}");
    at_str = tab(2, &at_str);

    let mut char_words_str = String::new();
    fmt_char_words(tree, char_words, &mut char_words_str);
    char_words_str = tab(2, &char_words_str);

    let result = template
        .replace(&key("no_touchy"), NO_TOUCHY)
        .replace(&key("types"), types_str.trim_end())
        .replace(&key("parent_type"), parent_type.trim_end())
        .replace(&key("has_value"), has_value_str.trim_end())
        .replace(&key("ends_on_newline"), ends_on_newline_str.trim_end())
        .replace(&key("get_by_word"), words.trim_end())
        .replace(&key("prefixes"), prefixes_str.trim_end())
        .replace(&key("at"), at_str.trim_end())
        .replace(&key("is_char_word"), char_words_str.trim_end());

    println!("{}", result);

    let mut file = File::create("../../src/parse/lex.rs").unwrap();
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
            result.push('\n');
        }
    }

    result
}

pub fn fmt_types(tree: &Tree, node: &Node, indent: usize, out: &mut String) {
    let tab = (0..indent).map(|_| "    ").collect::<String>();

    for index in &node.children {
        let child = &tree.nodes[*index];
        let line = format!("{}{},\n", tab, &child.value);
        out.push_str(&line); 

        fmt_types(tree, child, indent + 1, out)
    }
}

fn fmt_parents(tree: &Tree, node: &Node, out: &mut String, ln_start: &mut usize) {
    if node.children.is_empty() {
        return;
    }

    for (i, index) in node.children.iter().enumerate() {
        if out.len() - *ln_start >= 60 {
            out.push('\n'); 
            *ln_start = out.len();
        }

        out.push_str(&tree.nodes[*index].value);

        if i != node.children.len() - 1 {
            out.push('|');
        }
    }

    let arm = format!(" => {},\n\n", node.value);
    out.push_str(&arm);
    *ln_start = out.len() - 1;

    for index in &node.children {
        fmt_parents(tree, &tree.nodes[*index], out, ln_start);
    }
}

fn fmt_match_true(tree: &Tree, node: &Node, out: &mut String, ln_start: &mut usize) {
    for (i, index) in node.children.iter().enumerate() {
        if out.len() - *ln_start >= 60 {
            out.push('\n'); 
            *ln_start = out.len();
        }

        out.push_str(&tree.nodes[*index].value);

        if i != node.children.len() - 1 {
            out.push('|');
        }
    }

    out.push(')');
}

fn fmt_words(tree: &Tree, words: &Node, pairs: &Node, chars: &Node, out: &mut String) {
    for index in &words.children {
        let value = &tree.nodes[*index].value;
        let arm = format!("\"{}\" => Some({}),\n", value.to_lowercase(), value);
        out.push_str(&arm);
    }

    for index in &pairs.children {
        let child = &tree.nodes[*index];
        let word = &tree.nodes[child.children[0]].value;
        let arm = format!("\"{}\" => Some({}),\n", word, child.value);
        out.push_str(&arm);
    }

    for index in &chars.children {
        let child = &tree.nodes[*index];
        let word = &tree.nodes[child.children[0]].value;
        let arm = format!("\"{}\" => Some({}),\n", word, child.value);
        out.push_str(&arm);
    }
}

pub fn fmt_prefixes(tree: &Tree, node: &Node, out: &mut String) {
    out.push_str("matches!(prefix, ");

    for (i, index) in node.children.iter().enumerate() {
        let prefix = format!("'{}'", &tree.nodes[*index].value);
        out.push_str(&prefix); 

        if i != node.children.len() - 1 {
            out.push('|');
        }
    }

    out.push(')');
}

pub fn fmt_at(tree: &Tree, node: &Node, out: &mut String, len: &mut usize) {
    for index in &node.children {
        let child = &tree.nodes[*index];
        out.push_str(&format!("    {} => {},\n", *len, child.value));
        *len += 1; 

        fmt_at(tree, child, out, len);
    }
}

fn fmt_char_words(tree: &Tree, node: &Node, out: &mut String) {
    let mut ln = 0;

    for (i, index) in node.children.iter().enumerate() {
        let child = &tree.nodes[*index];
        let word = &tree.nodes[child.children[0]].value;
        let arm = format!("'{}'", word);
        out.push_str(&arm);

        if i != node.children.len() - 1 {
            out.push('|');

            if ln > 60 {
                out.push('\n');
            }
        }

        ln += arm.len()
    }

    out.push(')');
}
