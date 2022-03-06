
use KeyType::*;
use crate::{
    program::control::bug,
};

#[derive(Debug, Copy, Clone)]
enum KeyType {
    Void,
    Usize(usize),
    Str(usize),
}

/// Holds an index and a type to find a value within `Data`.
#[derive(Debug, Copy, Clone)]
pub struct Key {
    value: KeyType,
}

impl Key {

    /// Create a key that is not linked to any data.
    pub const fn void() -> Self {
        Self { value: KeyType::Void }
    }

}

/// Holds parsed data for all tokens.
pub struct Data<'a> {
    usizes: Vec<usize>,
    strs: Vec<&'a str>,
}

impl<'a> Data<'a> {
    
    pub const fn new() -> Self {
        Self {
            usizes: vec![],
            strs: vec![],
        }
    }

    /// Pushes a usize and return its `Key`.
    pub fn push_usize(&mut self, value: usize) -> Key {
        let len = self.usizes.len();
        self.usizes.push(value);

        Key{ value: Usize(len) }
    }

    /// Pushes a str and return its index.
    pub fn push_str(&mut self, value: &'a str) -> Key {
        let len = self.strs.len();
        self.strs.push(value);

        Key{ value: Str(len) }
    }

    /// Returns a `usize` with key.
    pub fn get_usize(&self, key: &Key) -> usize {
        if let Usize(index) = key.value {
            return self.usizes[index];
        }

        bug("Wrong data key type.");
    }

    /// Returns a `str` with key.
    pub fn get_str(&self, key: &Key) -> &str {
        if let Str(index) = key.value {
            return self.strs[index];
        }

        bug("Wrong data key type.");
    }

}
