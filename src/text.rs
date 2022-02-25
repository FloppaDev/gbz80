
/// Return the u64 hash of a str.
pub fn hash_str(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    h.finish()
}

/// Wrapper over a str.
/// It is meant to be the output of a function that validates
/// the string for its intended usage. It makes forgetting checks less likely.
#[derive(Copy, Clone, Debug)]
pub struct CheckedStr<'a> {
    value: &'a str
}

impl<'a> CheckedStr<'a> {

    /// This cannot be constructed outside of this module.
    const fn new(value: &'a str) -> Self {
        Self { value }
    }

    /// Get the wrapped value.
    pub const fn as_str(&self) -> &'a str {
        self.value 
    } 

}

/// Provides function to validate characters within words.
pub mod charset {

    use super::CheckedStr;

    /// Is it any of the newline characters?
    pub const fn is_new_line(c: char) -> bool {
        matches!(c, '\u{000A}'|'\u{000D}')
    }

    /// Is it any of the space characters?
    pub const fn is_space(c: char) -> bool {
        matches!(c, '\u{0009}'|'\u{000B}'|'\u{000C}'|'\u{0020}'|'\u{0085}'|
                    '\u{00A0}'|'\u{1680}'|'\u{180E}'|'\u{2000}'|'\u{2001}'|
                    '\u{2002}'|'\u{2003}'|'\u{2004}'|'\u{2005}'|'\u{2006}'|
                    '\u{2007}'|'\u{2008}'|'\u{2009}'|'\u{200A}'|'\u{200B}'|
                    '\u{200C}'|'\u{200D}'|'\u{2028}'|'\u{2029}'|'\u{202F}'|
                    '\u{205F}'|'\u{2060}'|'\u{3000}'|'\u{FEFF}')
    }

    pub const fn is_char_expr(c: char) -> bool {
        matches!(c, '+' | '-' | '(' | ')')
    }

    /// Is the character a number?
    pub const fn is_char_num(c: char) -> bool {
        matches!(c, '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0')
    }

    /// Is the character an hexadecimal number?
    pub const fn is_char_hex(c: char) -> bool {
        matches!(c, 'a'|'b'|'c'|'d'|'e'|'f'|'A'|'B'|'C'|'D'|'E'|'F') || is_char_num(c)
    }

    /// Can it be the first character of an identifier?
    pub const fn is_char_ident_first(c: char) -> bool {
        matches!(c, 'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'|'i'|'j'|'k'|'l'|'m'|
                    'n'|'o'|'p'|'q'|'r'|'s'|'t'|'u'|'v'|'w'|'x'|'y'|'z'|
                    '_'|
                    'A'|'B'|'C'|'D'|'E'|'F'|'G'|'H'|'I'|'J'|'K'|'L'|'M'|
                    'N'|'O'|'P'|'Q'|'R'|'S'|'T'|'U'|'V'|'W'|'X'|'Y'|'Z')
    }

    /// Is the word an identifier?
    pub fn check_ident(word: &str) -> Option<CheckedStr> {
        let mut iter = word.chars();

        let first = iter.next();
        if first.is_none() || !is_char_ident_first(first.unwrap()) {
            return None
        }

        for c in iter {
            if !is_char_ident_first(c) && !is_char_num(c) { 
                return None
            }
        }

        Some(CheckedStr::new(word))
    }

    /// Is the word a decimal literal?
    pub fn check_dec(word: &str) -> Option<CheckedStr> {
        for c in word.chars() {
            if !is_char_num(c) { 
                return None
            }
        }

        Some(CheckedStr::new(word))
    }

    /// Is the word an hexadecimal literal?
    pub fn check_hex(word: &str) -> Option<CheckedStr> {
        for c in word.chars() {
            if !is_char_hex(c) { 
                return None
            }
        }

        Some(CheckedStr::new(word))
    }

    /// Is the word an binary literal?
    pub fn check_bin(word: &str) -> Option<CheckedStr> {
        for c in word.chars() {
            if c != '0' && c != '1' && c != '_' { 
                return None
            }

        }

        Some(CheckedStr::new(word))
    }

    /// Use for strings that do not need checking.
    pub const fn no_check(word: &str) -> CheckedStr {
        CheckedStr::new(word)
    }

}

pub fn hex_to_byte(hex: &str) -> u8 {
    let hex = hex.chars().collect::<Vec<_>>();
    (hex[0].to_digit(16).unwrap() * 16 + hex[1].to_digit(16).unwrap()) as u8
}
