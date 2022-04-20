
use crate::{
    token::read::TokenRef,
    parse::{
        lex::TokenType::*,
    },
    error::asm::{
        AsmErr, 
        ValidationMsg::{self, *},
    },
};

pub fn run<'a>(root: &TokenRef<'a>) -> Result<(), Vec<AsmErr<'a, ValidationMsg>>> {
    assert_eq!(root.ty(), Root);

    let mut errors = vec![];
    walk(root, &mut errors);

    return if errors.is_empty() {
        Ok(())
    }else {
        Err(errors)
    };
}

fn walk<'a>(scope: &TokenRef<'a>, errors: &mut Vec<AsmErr<'a, ValidationMsg>>) {
    for child in scope.children() {
        if !child.ty().validate(scope.ty()) {
            errors.push(err!(ValidationMsg, InvalidParent, child.into())); 
        }

        walk(child, errors);
    }
}
