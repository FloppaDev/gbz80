
use std::path::Path;

use crate::{
    parse::split::Split,
};

#[derive(Debug)]
pub struct Source {
    //There will be multiple inputs once #import is implemented.
    pub inputs: Vec<Input>,
}

impl Source {
    
    pub fn new(main_path: &str) -> Self {
        let main_content = fs::read_to_string(main_path)?;
        let mut source = Self{ inputs: vec![] };
        source.inputs.push(Input::new(main_path, content));

        let stack = vec![main_path];
        source.search_file(main_content, stack);
    }

    fn search_file(&mut self, content: &str, mut stack: Vec<String>) {
        for i in content.match_indices("#import") {
            // Must be surrounded by whitespace.
            if let Some(prev) = content.get(i-1) {
                if !prev.is_whitespace() {
                    continue;
                }
            }

            if let Some(next) = content.get(i+1) {
                if !next.is_whitespace() {
                    continue;
                }
            }

            let import_to_end = content.get(i..).unwrap();
            let mut in_quotes = false;
            let mut indices = vec![];

            // Find the range of the file path.
            for (i, ch) in import_to_end.chars().enumerate() {
                if ch == "\"" {
                    if in_quotes {
                        indices.push(i);
                        break;
                    }

                    else {
                        indices.push(i);
                        in_quotes = true;
                    }
                }

                else if !in_quotes && !ch.is_whitespace {
                    //TODO push err
                }
            }

            if indices.len() != 2 {
                //TODO push err
                continue;
            }

            let path = content.get(i+indices[0]..i+indices[0]+indices[1]).unwrap();

            // Prevent circular dependencies.
            for p in &stack {
                if p == path {
                    //TODO push err
                    continue;
                }
            }

            if let Some(content) = fs::read_to_string(path) {
                source.inputs.push(Input::new(path, content));

                let mut new_stack = stack.clone();
                new_stack.push(path);
                source.search_file(content, new_stack);
            }

            else {
                //TODO push err
            }
        }
    }

}

#[derive(Debug)]
pub struct Input<'a> {
    pub path: String,
    pub content: String,
}

impl<'a> Input<'a> {

    pub fn new(path: String, content: String) -> Self {
        Self{ path, content }
    }

}
