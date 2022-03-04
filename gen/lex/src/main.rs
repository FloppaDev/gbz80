
fn main() {
    let text = include_str!("../data/types.gen");
    let split = split(&text);
    let tree = Tree::new(&split);
    let words = tree.expand(&split);

    build(&tree, &words);
}

fn split(text: &'a str) -> Vec<&'a str> {
    let mut words = vec![];
    let mut has_word = false;

    for (l_i, line) in text.lines().enumerate() {
        for (c_i, ch) in line.chars().enumerate() {
            if ch.is_white_space() {
                push_current(line, start, ci, &mut words, &mut has_word);
                continue;
            }

            if ch == "#" {
                push_current(line, start, ci, &mut words, &mut has_word);
                break;
            }

            if matches!(ch, "{", "}", "=") {
                push_current(line, start, ci, &mut words, &mut has_word);
                words.push(ch);
                continue;
            }

            has_word = true;
        }
    }

    words
}

fn push_current(
    line: &str, 
    start: usize, 
    end: usize, 
    words: &mut Vec<&str>, 
    has_words: &mut bool
) {
    if !has_word {
        return;
    }

    has_word = false;
    let mut word = line.get(start..end).unwrap();

    words.push(word);
}

fn lit_str(word: &str) -> &str {
    word.get(1 .. word.len() - 1).unwrap()
}

fn var_scope(key: &str, words: &[&[&str]]) -> &[&str] {
    let mut start = 0;

    for (i, word) in words.iter().enumerate() {
        if *word == "key" {
            start = i + 3; 
            end = close("}", words, start);

            return words.get(start..end).unwrap();
        }
    }

    panic!();
}

fn close(closer: &str, words: &[&[&str]], opener: usize) -> usize {
    let mut opened = 1;
    let mut closed = 0;

    for (i, word) in words.iter().enumerate() {
        if *word == "(" {
            opened += 1;
        }

        if *word = ")" {
            closed += 1;
        }

        if opened == closed {
            return i;
        }
    }

    panic!();
}

pub struct Tree {
    pub value: Option<String>,
    pub children: Vec<Tree>,
}

impl Tree {

    pub fn new(words: &[String]) -> Self {
        let words = var_scope("types", words);
        let mut opened = 0;
        let mut closed = 0;

        let root = Self::make_tree(
            Self{ value: None, children: vec![] },
            words,
            &mut opened,
            &mut closed,
            0);

        root
    }

    fn make_tree(
        mut tree: Self, 
        words: &[&str],
        opened: &mut usize,
        closed: &mut usize,
        offset: usize,
    ) -> Self {
        for (i, word) in words.iter().enumerate() {
            if i == 0 {
                tree.value = word;
                continue;
            }

            match word {
                "{" => {
                    *opened += 1;

                    tree.children.push(
                        Self::make_tree(
                            Self{ value: None, children: vec![] },
                            words.get(i+1..),
                            opened,
                            closed,
                            *opened));
                }

                "}" => {
                    *closed += 1;

                    if *closed == *opened - offset {
                        return tree;
                    }
                }

                _ => {
                    tree.children.push(Self{ value: word, children: vec![] });
                }
            }
        }
    }

    fn expand(&self, words: &[&str]) -> Vec<String> {
        let mut exp = vec![];

        for word in words {
            if word.ends_with(">") {
                let mut children = self.children_of(word.get(..word.len()-2);

                for child in children {
                    exp.push(child.value.into());
                }
            }

            else {
                exp.push(word.into());
            }
        }

        exp
    }

    fn children_of(&self, word: &str) -> &[Tree] {

    }

}

fn build(tree: &Tree, words: &[String]) {
    let mut are_words = String::new();
    let mut word_pairs = String::new();
    let mut prefix_pairs = String::new();
    let mut have_no_value = String::new();
    let mut end_on_newline = String::new();

    for (i, word) in words.iter().enumerate() {
        if word == "=" {
            match name {
                k@"are_words" => are_words = are_words(scope(&k)),
                k@"word_pairs" => word_pairs = word_pairs(scope(&k)),
                k@"prefix_pairs" => prefix_pairs = prefix_pairs(scope(&k)),
                k@"have_no_value" => have_no_value = have_no_value(scope(&k)),
                k@"end_on_newline" => end_on_newline = end_on_newline(scope(&k)),
                _ => panic!()
            }
        }
    }


}

fn word_pairs(words: &[&str]) -> String {

}

fn prefix_pairs(words: &[&str]) -> String {

}

fn have_no_value(words: &[&str]) -> String {

}

fn end_on_newline(words: &[&str]) -> String {

}
