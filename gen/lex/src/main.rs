
mod parse;

use crate::{
    parse::{split, Tree},
};

fn main() {
    let text = include_str!("../data/types.gen");
    let split = split(&text);
    let tree = Tree::new(&split);

    tree.debug();

    build(&tree);
}

fn build(tree: &Tree) {
    let are_words = tree.find("are_words");
    let word_pairs = tree.find("word_pairs");
    let prefix_pairs = tree.find("prefix_pairs");
    let have_no_value = tree.find("have_no_value");
    let end_on_newline = tree.find("end_on_newline");
}
