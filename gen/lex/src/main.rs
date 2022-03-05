
mod parse;

use crate::{
    parse::{split, Tree},
};

fn main() {
    let text = include_str!("../data/types.gen");
    let split = split(text);
    let tree = Tree::new(&split);

    tree.debug();

    build(&tree);
}

fn build(tree: &Tree) {
    let _are_words = tree.find("are_words");
    let _word_pairs = tree.find("word_pairs");
    let _prefix_pairs = tree.find("prefix_pairs");
    let _have_no_value = tree.find("have_no_value");
    let _end_on_newline = tree.find("end_on_newline");
}
