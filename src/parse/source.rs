
use std::{
    path::Path,
    fs::{self, File},
    io::{self, prelude::*},
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

    #[cfg(test)]
    pub fn from_content(content: String) -> Self {
        let mut source = Self{ inputs: vec![] };
        source.inputs.push(Input::new("".into(), content));

        source
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

    /// Reads a file to bytes with a relative path.
    pub fn read_local(&self, local: &str) -> Result<Vec<u8>, io::Error> {
        let path = match self.path().parent() {
            Some(dir) => format!("{}/{}", dir.to_str().unwrap(), local),
            None => local.into() 
        };

        let mut buffer = vec![];
        let mut file = File::open(path)?;
        file.read_to_end(&mut buffer)?;

        Ok(buffer)
    }

    pub fn lines(&self) -> std::str::Lines {
        self.content.lines() 
    }

}
