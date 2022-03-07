
//[[no_touchy]]

use TokenType::*;

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    //[[types]] 
}

/// Returns the parent of a type.
pub const fn parent_type(ty: TokenType) -> TokenType {
    match ty {
        Root|//[[parent_type]]
    }
}

/// Can this token type hold a value?
pub const fn has_value(ty: TokenType) -> bool {
    matches!(ty,
        //[[has_value]]
}

/// Is it one the tokens that end on a newline?
pub const fn ends_on_newline(ty: TokenType) -> bool {
    matches!(ty,
        //[[ends_on_newline]]
}

/// Find a token type that can be identified from a word.
pub fn get_by_word(name: &str) -> Option<TokenType> {
    match name {
        //[[get_by_word]]
        _ => None
    }
}

/// Is there any type that starts with this prefix character?
pub const fn has_prefix(prefix: char) -> bool {
    //[[prefixes]]
}
