
use std::{
    path::Path,
    fs,
};

#[derive(Debug)]
pub struct Source {
    pub inputs: Vec<Input>,
}

impl Source {
    
    pub fn new(main_path: &str) -> std::io::Result<Self> {
        let main_content = fs::read_to_string(main_path)?;
        let mut source = Self{ inputs: vec![] };
        source.inputs.push(Input::new(main_path.into(), main_content));

        Ok(source)
    }

    // Gets the main source file. 
    pub fn main(&self) -> &Input {
        &self.inputs[0]
    }

}

#[derive(Debug)]
pub struct Input {
    pub path: String,
    pub content: String,
}

impl Input {

    pub const fn new(path: String, content: String) -> Self {
        Self{ path, content }
    }

    pub fn path(&self) -> &Path {
        Path::new(&self.path)
    }

    pub fn lines(&self) -> std::str::Lines {
        self.content.lines() 
    }

}
