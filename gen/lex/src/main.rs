
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

    build(&tree);
}

fn build(tree: &Tree) {
    let types = tree.find("types");
    let root = &tree.nodes[types.children[0]];
    let has_value = tree.find("has_value");
    let ends_on_newline = tree.find("ends_on_newline");
    let char_words = tree.find("char_words");
    let are_words = tree.find("are_words");
    let word_pairs = tree.find("word_pairs");
    let prefixes = tree.find("prefixes");
    let hierarchy = tree.find("validate_from_hierarchy");
    let validation = tree.find("validation");

    let template = include_str!("../data/lex.rs");
    let mut result = String::from(template);
    let mut fmt = String::from(NO_TOUCHY);
    apply(&mut result, "no_touchy", &mut fmt, 0);

    fmt_types(tree, types, 0, &mut fmt);
    apply(&mut result, "types", &mut fmt, 0);

    fmt_parents(tree, root, &mut fmt, &mut 0);
    apply(&mut result, "parent_type", &mut fmt, 3);

    fmt_match_true(tree, has_value, &mut fmt, &mut 0);
    apply(&mut result, "has_value", &mut fmt, 3);

    fmt_match_true(tree, ends_on_newline, &mut fmt, &mut 0);
    apply(&mut result, "ends_on_newline", &mut fmt, 3);

    fmt_words(tree, are_words, word_pairs, char_words, &mut fmt);
    apply(&mut result, "get_by_word", &mut fmt, 3);

    fmt_prefixes(tree, prefixes, &mut fmt);
    apply(&mut result, "prefixes", &mut fmt, 3);

    let mut len = 0;
    fmt_tests(tree, root, &mut fmt, &mut len);
    apply(&mut result, "tests", &mut fmt, 1);

    fmt_char_words(tree, char_words, &mut fmt);
    apply(&mut result, "is_char_word", &mut fmt, 1);

    len = 0;
    fmt_hierarchy_validation(tree, hierarchy, &mut fmt, &mut len);
    apply(&mut result, "hierarchy_validation", &mut fmt, 3);

    len = 0;
    fmt_validation(tree, validation, &mut fmt, &mut len);
    apply(&mut result, "validation", &mut fmt, 3);

    let mut file = File::create("../../src/parse/lex.rs").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}

fn apply(target: &mut String, key: &str, content: &mut String, indent: usize) {
    *target = target.replace(&format!("//[[{key}]]"), &tab(indent, content.trim_end()));
    content.clear();
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

        fmt_types(tree, child, indent + 1, out);
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

fn fmt_prefixes(tree: &Tree, node: &Node, out: &mut String) {
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

fn fmt_tests(tree: &Tree, node: &Node, out: &mut String, len: &mut usize) {
    let mut at_arms = String::new();
    fmt_at_arms(tree, node, &mut at_arms, len);

    out.push_str("/// The count of token types.\n");
    out.push_str("#[cfg(test)]\n");
    out.push_str(&format!("pub const COUNT: usize = {len};\n\n"));
    out.push_str("/// Returns a `TokenType` from an index.\n");
    out.push_str("#[cfg(test)]\n");
    out.push_str("pub const fn at(index: usize) -> Self {\n");
    out.push_str("    match index % Self::COUNT {\n    ");
    out.push_str(&tab(1, &at_arms));
    out.push_str("\n        _ => panic!()\n    }\n}");
}

fn fmt_at_arms(tree: &Tree, node: &Node, out: &mut String, len: &mut usize) {
    for index in &node.children {
        let child = &tree.nodes[*index];
        out.push_str(&format!("    {} => {},\n", *len, child.value));
        *len += 1; 

        fmt_at_arms(tree, child, out, len);
    }
}

fn fmt_char_words(tree: &Tree, node: &Node, out: &mut String) {
    out.push_str("matches!(c, ");
    let mut ln = 0;

    for (i, index) in node.children.iter().enumerate() {
        let child = &tree.nodes[*index];
        let word = &tree.nodes[child.children[0]].value;
        let arm = format!("'{word}'");
        out.push_str(&arm);

        if i != node.children.len() - 1 {
            out.push('|');

            if ln > 60 {
                out.push('\n');
            }
        }

        ln += arm.len();
    }

    out.push(')');
}

fn fmt_hierarchy_validation(tree: &Tree, node: &Node, out: &mut String, ln_start: &mut usize) {
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

    out.push_str(" => self.parent_type() == parent_type,\n");
}

fn fmt_validation(tree: &Tree, node: &Node, out: &mut String, ln_start: &mut usize) {
    let mut i = 0;
    let children = &node.children;
    let len = children.len();

    loop {
        *ln_start = out.len();
        let children_a = &tree.nodes[children[i]].children;

        for (i, index) in children_a.iter().enumerate() {
            if out.len() - *ln_start >= 60 {
                out.push('\n'); 
                *ln_start = out.len();
            }

            let child = &tree.nodes[*index];
            out.push_str(&child.value);

            if i != children_a.len() - 1 {
                out.push('|');
            }
        }

        *ln_start = out.len();
        out.push_str("=> matches!(parent_type, ");
        let children_b = &tree.nodes[children[i+1]].children;

        for (i, index) in children_b.iter().enumerate() {
            if out.len() - *ln_start >= 60 {
                out.push_str("\n    "); 
                *ln_start = out.len();
            }

            let child = &tree.nodes[*index];
            out.push_str(&child.value);

            if i != children_b.len() - 1 {
                out.push('|');
            }
        }

        out.push_str("),\n");
        i += 2;

        if i >= len {
            break;
        }

        out.push('\n');
    }
}
