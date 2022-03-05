
pub fn split(text: &str) -> Vec<String> {
    let mut words = vec![];
    let mut has_word = false;

    for line in text.lines() {
        let mut start = 0;

        for (c_i, ch) in line.chars().enumerate() {
            if ch.is_whitespace() {
                push_current(line, start, c_i, &mut words, &mut has_word);
                continue;
            }

            if ch == '/' {
                let ch2 = line.chars().nth(c_i + 1);

                if ch2.is_some() && ch2.unwrap() == '/' {
                    push_current(line, start, c_i, &mut words, &mut has_word);
                    break;
                }
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
        let base = Node{ parent: 0, value: "".into(), children: vec![] };
        let mut tree = Tree{ nodes: vec![base] };
        let mut stack = vec![];
        let mut parent = 0;
        let mut open = true;

        for word in words.iter() {
            match word.as_ref() {
                "{" => {
                    open = true;  
                    stack.push(parent);
                    println!("{}", word);
                }

                "}" => {
                    println!("{}", word);
                    parent = stack.pop().unwrap();
                }

                _ => {
                    if open && tree.nodes.len() > 0 {
                        parent = tree.nodes.len() - 1;
                    }

                    for value in tree.expand(&word) {
                        let node = Node{ 
                            parent, value, children: vec![] 
                        };

                        let index = tree.nodes.len();
                        tree.nodes.push(node);
                        tree.nodes[parent].children.push(index);
                    }

                    open = false;
                }
            }
        }

        tree
    }

    fn expand(&self, word: &str) -> Vec<String> {
        let mut exp = vec![];

        if word.ends_with(">") {
            let node = self.find(word.get(..word.len()-1).unwrap());

            for index in &node.children {
                exp.push(self.nodes[*index].value.clone());
            }
        }

        else {
            exp.push(word.into());
        }

        exp
    }

    pub fn find(&self, word: &str) -> &Node {
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

    pub fn debug(&self) {
        dbg!(self.nodes[0].children.len());
        self.debug_node(&self.nodes[0], 0); 
    }

    fn debug_node(&self, node: &Node, indent: usize) {
        let tab = (0..indent).map(|_| "    ").collect::<String>();

        for index in &node.children {
            let child = &self.nodes[*index];
            println!("{}{}", tab, &child.value); 
            self.debug_node(child, indent + 1)
        }
    }

}
