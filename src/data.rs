
use KeyType::*;

#[derive(Debug, Copy, Clone)]
enum KeyType {
    Void,
    U8(usize),
    U16(usize),
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
    u8s: Vec<u8>,
    u16s: Vec<u16>,
    strs: Vec<&'a str>,
}

impl<'a> Data<'a> {
    
    pub const fn new() -> Self {
        Self {
            u8s: vec![],
            u16s: vec![],
            strs: vec![],
        }
    }

    /// Push the value as 'u8' if possible or 'u16' and return its index.
    /// Returns 'None' if the value does not fit into a u16.
    pub fn push_num(&mut self, value: usize) -> Option<Key> {
        if value <= 255 {
            return Some(self.push_u8(value as u8))
        }

        if value <= 65536 {
            return Some(self.push_u16(value as u16))
        } 

        None
    }

    /// Push a u8 and return its index.
    pub fn push_u8(&mut self, value: u8) -> Key {
        let len = self.u8s.len();
        self.u8s.push(value);

        Key{ value: U8(len) }
    }

    /// Push a u16 and return its index.
    pub fn push_u16(&mut self, value: u16) -> Key {
        let len = self.u16s.len();
        self.u16s.push(value);

        Key{ value: U16(len) }
    }

    /// Push a str and return its index.
    pub fn push_str(&mut self, value: &'a str) -> Key {
        let len = self.strs.len();
        self.strs.push(value);

        Key{ value: Str(len) }
    }

    /// Return a `u8` with key.
    pub fn get_u8(&self, key: &Key) -> u8 {
        if let U8(index) = key.value {
            return self.u8s[index];
        }

        panic!("Wrong data key type");
    }

    /// Tries to return a `u16` with key.
    pub fn get_u16(&self, key: &Key) -> Option<u16> {
        if let U16(index) = key.value{
            if let Some(index) = self.u16s.get(index) {
                return Some(*index)
            }
        }

        None
    }

    /// Return a `str` with key.
    pub fn get_str(&self, key: &Key) -> &str {
        if let Str(index) = key.value {
            return self.strs[index];
        }

        panic!("Wrong data key type");
    }

}
