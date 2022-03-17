
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
