
fn main() {
    let text = include_str!("../data/types.gen");
    let split = split(&text);
    let tree = Tree::new(&split);

    let words = tree.expand(&split);

    build(&tree);
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

            if matches!(ch, '{'|'}') {
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

    words.push(word.to_string());
}

pub struct Node {
    parent: usize,
    value: String,
    children: Vec<usize>,
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {

    pub fn new(words: &[String]) -> Self {
        let mut base = Node{ parent: 0, value: "".into(), children: vec![] };
        let mut tree = Tree{ nodes: vec![base] };
        let mut stack = vec![];
        let mut current = 0;
        let mut open = true;

        for word in words.iter() {
            match word.as_ref() {
                "{" => {
                    open = true;  
                    stack.push(current);
                }

                "}" => {
                    current = stack.pop().unwrap();
                }

                _ => {
                    let parent = if open {
                        match tree.nodes.len() {
                            0 => current,
                            l@_ => l-1,
                        }
                    }else {
                        current
                    };

                    let node = Node{ 
                        parent, value: word.clone(), children: vec![] 
                    };

                    let index = tree.nodes.len();
                    tree.nodes.push(node);
                    tree.nodes[current].children.push(index);

                    open = false;
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

                for index in &node.children {
                    exp.push(self.nodes[*index].value.clone());
                }
            }

            else {
                exp.push(word.clone());
            }
        }

        exp
    }

    fn find(&self, word: &str) -> &Node {
        &self.scan_children(&self.nodes[0], word).unwrap()
    }

    fn scan_children<'a>(&'a self, node: &'a Node, word: &str) -> Option<&'a Node> {
        for index in &node.children {
            let child = &self.nodes[*index];

            if &child.value == word {
                return Some(child);
            }

            if let Some(child) = self.scan_children(child, word) {
                return Some(child);
            }
        }

        None
    }

}

fn build(tree: &Tree) {
    let are_words = tree.find("are_words");
    let word_pairs = tree.find("word_pairs");
    let prefix_pairs = tree.find("prefix_pairs");
    let have_no_value = tree.find("have_no_value");
    let end_on_newline = tree.find("end_on_newline");
}
