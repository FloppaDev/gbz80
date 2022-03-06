
//[[no_touchy]]

use TokenType::*;

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    //[[token_types]] 
}

/// Returns the parent of a type.
pub const fn parent_type(ty: TokenType) -> TokenType {
    match ty {
        //[[parent_type]]
    }
}

/// Generalization of a type within Argument.
/// Instruction -> Argument -> Lit -> ...   = Lit
/// Instruction -> Argument -> Identifier   = Identifier
pub const fn argument_type(ty: TokenType) -> TokenType {
    //[[argument_type]]
}

/// Can this token type hold a value?
pub const fn has_value(ty: TokenType) -> bool {
    match ty {
        //[[has_value]]

        _ => false
    }
}

/// Is it one the tokens that end on a newline?
pub const fn ends_on_newline(ty: TokenType) -> bool {
    //[[ends_on_newline]]
}

/// Find a token type that can be identified from a word.
pub const fn get_by_word(name: &str) -> Option<TokenType> {
    //[[get_by_word]]
}

/// Find all types that match the prefix.
/// e.g. &2893 is a hexadecimal literal.
pub const fn get_by_prefix(first: &str) -> Vec<TokenType> {
    //[[get_by_prefix]]
}
