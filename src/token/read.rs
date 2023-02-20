
#![allow(dead_code)]

use crate::{
    parse::lex::TokenType,
    token::{ Token, Value, ast::Ast },
    error::{ ITERATION_LIMIT, asm::{ AsmErr, AstValidationMsg::{self, *} } },
};

/// Read-only reference to a token.
/// Includes the AST for navigating the hierarchy.
#[derive(Debug)]
pub struct TokenRef<'a> {
    ast: &'a Ast<'a>,
    token: &'a Token<'a>,
    parent: *const Self,
    children: Vec<Self>,
}

impl<'a> PartialEq for TokenRef<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index() == other.index()
    }
}

impl<'a> Eq for TokenRef<'a> {}

impl<'a> TokenRef<'a> {

    /// Creates a `TokenRef` from the root token of an `Ast`.
    pub fn new(ast: &'a Ast) -> Self {
        let root = ast.get_root();
        let mut current = Self{ ast, token: root, parent: std::ptr::null(), children: vec![] };
        current.parent = &current;
        Self::walk(ast, &mut current, 0); 

        current
    }

    fn walk(ast: &'a Ast, current: &mut Self, fail_safe: usize) {
        if fail_safe >= ITERATION_LIMIT {
            bug!("Recursion limit reached while building TokenRef tree.");
        }

        for child in &current.token.children {
            let token = &ast.tokens[*child];
            let token_ref = Self{ ast, token, parent: current, children: vec![] };
            current.children.push(token_ref);
            Self::walk(ast, current.children.last_mut().unwrap(), fail_safe + 1);
        }
    }

    /// Returns a reference to the child `TokenRef` at specified index.
    /// Panics:
    /// Index not found.
    pub fn get(&self, child: usize) -> &Self {
        &self.children[child]
    }

    /// Tries to return a reference to the child `TokenRef` at specified index.
    pub fn try_get(&self, child: usize) -> Option<&Self> {
        self.children.get(child)
    }

    pub const fn ast(&self) -> &Ast { self.ast }
    pub const fn token(&self) -> &Token<'a> { self.token }
    pub const fn ty(&self) -> TokenType { self.token.ty }
    pub const fn line_number(&self) -> usize { self.token.line_number }
    pub const fn line(&self) -> &'a str { self.token.line }
    pub const fn word(&self) -> &'a str { self.token.word }
    pub const fn value(&self) -> &Value { &self.token.value }
    pub const fn index(&self) -> usize { self.token.index }

    /// Returns a reference to the parent `TokenRef` or `self` if it is the root.
    #[allow(clippy::missing_const_for_fn)]
    pub fn parent(&self) -> &Self {
        unsafe { &*self.parent }
    }

    /// Returns references to all `TokenRef` children.
    pub fn children(&self) -> Vec<&Self> {
        self.children.iter().collect::<Vec<_>>()
    }

    /// Recursively finds the first child until there is no child.
    pub fn leaf(&self) -> &Self {
        let mut current = self;

        while let Some(child) = current.try_get(0) {
            current = child;
        }

        current
    }

    /// Returns a reference to the first child.
    /// Panics:
    /// There are no children.
    pub fn first(&self) -> &Self {
        self.get(0)
    }

    /// Finds the first child of specified type.
    /// Panics:
    /// The token is not found.
    pub fn first_of(&self, ty: TokenType) -> &Self {
        for child in &self.children {
            if child.ty() == ty {
                return child;
            }
        }

        bug!("Could not find Token by type.");
    }

    /// Finds the first child of specified type.
    pub fn try_first_of(&self, ty: TokenType) -> Option<&Self> {
        self.children.iter().find(|&child| child.ty() == ty)
    }

    /// Validates the hierarchy of tokens, starting from `Root`.
    pub fn validate(&self) -> Result<(), Vec<AsmErr<'a, AstValidationMsg>>> {
        assert_eq!(self.ty(), TokenType::Root);

        let mut errors = vec![];
        self.validate_walk(&mut errors);

        if errors.is_empty() {
            Ok(())
        }else {
            Err(errors)
        }
    }

    fn validate_walk(&self, errors: &mut Vec<AsmErr<'a, AstValidationMsg>>) {
        for child in self.children() {
            if !child.ty().validate(self.ty()) {
                errors.push(err!(AstValidationMsg, BadParent, child.into())); 
            }

            child.validate_walk(errors);
        }
    }

}
