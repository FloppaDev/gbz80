
use crate::{
    token::ast::Ast,
    token::Token,
};

// Precedence from strongest to weakest:
// unary -
// * / %
// + -
// << >>
// & ^ |

pub fn build<'a>(ast: &'a Ast<'a>, token: &'a Token<'a>) -> &'a Token<'a> {
    todo!()
}
