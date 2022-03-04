
fn main() {
    let text = include_str!("../data/types.gen");
    let split = split(&text);
    let tree = Tree::new(&split);
    let words = tree.expand(&split);

    build(&tree, &words);
}

fn split(text: &str) -> Vec<String> {
    let mut words = vec![];
    let mut has_word = false;

    for (l_i, line) in text.lines().enumerate() {
        let mut start = 0;

        for (c_i, ch) in line.chars().enumerate() {
            if ch.is_whitespace() {
                push_current(line, start, c_i, &mut words, &mut has_word);
                continue;
            }

            if ch == '#' {
                push_current(line, start, c_i, &mut words, &mut has_word);
                break;
            }

            if matches!(ch, '{'|'}'|'=') {
                push_current(line, start, c_i, &mut words, &mut has_word);
                words.push(ch.to_string());
                continue;
            }

            if !has_word {
                has_word = true;
                start = c_i;
            }

            if c_i == line.len() - 1 {
                push_current(line, start, c_i + 1, &mut words, &mut has_word);
            }
        }
    }

    words
}

fn push_current(
    line: &str, 
    start: usize, 
    end: usize, 
    words: &mut Vec<String>, 
    has_word: &mut bool,
) {
    if !*has_word {
        return;
    }

    *has_word = false;
    let mut word = line.get(start..end).unwrap();
    println!("{}", word);

    words.push(word.to_string());
}

fn lit_str(word: &str) -> &str {
    word.get(1 .. word.len() - 1).unwrap()
}

fn var_scope<'a>(key: &str, words: &'a [String]) -> &'a [String] {
    let mut start = 0;

    for (i, word) in words.iter().enumerate() {
        if word == key {
            start = i + 3; 
            let end = close(words, start);

            return words.get(start..end).unwrap();
        }
    }

    panic!("{}", key);
}

fn close(words: &[String], opener: usize) -> usize {
    let mut opened = 1;
    let mut closed = 0;

    for (i, word) in words.get(opener..).unwrap().iter().enumerate() {
        if *word == "{" {
            opened += 1;
        }

        else if *word == "}" {
            closed += 1;
        }

        if opened == closed {
            return i;
        }
    }

    panic!("{} != {} at {}", opened, closed, words[opener]);
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
        words: &[String],
        opened: &mut usize,
        closed: &mut usize,
        offset: usize,
    ) -> Self {
        for (i, word) in words.iter().enumerate() {
            if i == 0 {
                tree.value = Some(word.clone());
                continue;
            }

            match word.as_str() {
                "{" => {
                    *opened += 1;

                    tree.children.push(
                        Self::make_tree(
                            Self{ value: None, children: vec![] },
                            words.get(i+1..).unwrap(),
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
                    tree.children.push(
                        Self{ value: Some(word.clone()), children: vec![] });
                }
            }
        }

        tree
    }

    fn expand(&self, words: &[String]) -> Vec<String> {
        let mut exp: Vec<String> = vec![];

        for word in words {
            if word.ends_with(">") {
                let mut node = self.find(word.get(..word.len()-1).unwrap());

                for child in &node.children {
                    if let Some(value) = &child.value {
                        exp.push(value.clone());
                    }
                }
            }

            else {
                exp.push(word.clone());
            }
        }

        exp
    }

    fn find(&self, word: &str) -> &Tree {
        &self.scan_children(self, word).unwrap()
    }

    fn scan_children<'a>(&self, node: &'a Tree, word: &str) -> Option<&'a Tree> {
        for child in &node.children {
            if child.value.is_some() && child.value.as_ref().unwrap() == word {
                return Some(child);
            }

            if let Some(children) = node.scan_children(child, word) {
                return Some(children);
            }
        }

        None
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
            let name = words[i-1].as_str();

            match name {
                k@"are_words" => {
                    are_words = String::new();
                }

                k@"word_pairs" => {
                    word_pairs = String::new();
                }

                k@"prefix_pairs" => {
                    prefix_pairs = String::new();
                }

                k@"have_no_value" => {
                    have_no_value = String::new();
                }

                k@"end_on_newline" => {
                    end_on_newline = String::new();
                }

                "types" => {}

                _ => panic!("{}", name)
            }
        }
    }


}
