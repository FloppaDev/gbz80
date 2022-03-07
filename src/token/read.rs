
#![allow(dead_code)]

use crate::{
    parse::lex::TokenType,
    token::{
        Token, Value,
        ast::Ast,
    },
    program::{
        RECURSION_LIMIT,
    }
};

use std::hash::{Hash, Hasher};

/// Read-only reference to a token.
/// Includes the AST for navigating the hierarchy.
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

impl<'a> Hash for TokenRef<'a> {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index().hash(state);
    }

}

impl<'a> TokenRef<'a> {

    /// Creates a `TokenRef` from the root token of an `Ast`.
    pub fn new(ast: &'a Ast) -> Self {
        let mut fail_safe = RECURSION_LIMIT;
        let root = ast.get_root();
        let mut current = Self{ 
            ast, token: root, parent: std::ptr::null(), children: vec![]
        };
        current.parent = &current;

        Self::walk(ast, &mut current, &mut fail_safe); 

        current
    }

    fn walk(ast: &'a Ast, current: &mut Self, fail_safe: &mut usize) {
        if *fail_safe == 0 {
            unreachable!("Recursion limit reached while building TokenRef tree.");
        }

        for child in &current.token.children {
            *fail_safe -= 1;
            let token = &ast.tokens[*child];
            let mut token_ref = Self{ 
                ast, token, parent: current, children: vec![]
            };

            current.children.push(token_ref);

            Self::walk(ast, current.children.last_mut().unwrap(), fail_safe);
        }
    }

    pub const fn ast(&self) -> &Ast {
        self.ast
    }

    pub const fn token(&self) -> &Token<'a> {
        self.token
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

    pub const fn ty(&self) -> TokenType {
        self.token.ty
    }

    pub const fn line_number(&self) -> usize {
        self.token.line_number
    }

    pub const fn line(&self) -> &'a str {
        self.token.line
    }

    pub const fn word(&self) -> &'a str {
        self.token.word
    }

    pub const fn value(&self) -> &Value {
        &self.token.value
    }

    pub const fn index(&self) -> usize {
        self.token.index
    }

    /// Returns a reference to the parent `TokenRef` or `self` if it is the root.
    #[allow(clippy::missing_const_for_fn)]
    pub fn parent(&self) -> &Self {
        unsafe { &*self.parent }
    }

    /// Returns reference to all `TokenRef` children.
    pub fn children(&self) -> Vec<&Self> {
        self.children.iter().collect::<Vec<_>>()
    }

}

