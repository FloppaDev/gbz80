
use crate::{
    parse::lex::TokenType::{self, *},
    token::{
        Value, ParsedToken, Token,
        ast::Ast,
    }, 
};

#[cfg(debug_assertions)]
use crate::{
    program::fmt::title,
};

impl<'a> Ast<'a> {

    /// Push a new token as child into destination.
    fn push(&mut self, dest: usize, token: ParsedToken<'a>) -> usize {
        // Create the Token
        let index = self.tokens.len();
        let token = Token::new(token, index, dest);

        // Push the token and it index.
        self.tokens.push(token);
        self.tokens[dest].children.push(index);

        index
    }

    /// Push new tokens of parent types, and parent them in a cascade with 'token' as final child. 
    /// Parent tokens inherit line data from 'token'.
    /// Optionaly sets `dest` to the index of one of the pushed tokens.
    pub fn cascade(
        &mut self, 
        dest: &mut usize, 
        parent_types: &[TokenType], 
        token: ParsedToken<'a>,
        select_up: Option<usize>,
    ) {
        let ParsedToken{ line_number, line, .. } = token;

        let mut selection = *dest;
        let mut inserts = vec![];

        for ty in parent_types {
            let parent = Self::empty(*ty, line_number, line);
            selection = self.push(selection, parent);
            inserts.push(self.tokens.len()-1);
        }

        self.push(selection, token);
        inserts.push(self.tokens.len()-1);

        if let Some(select_up) = select_up {
            if let Some(i) = inserts.iter().rev().nth(select_up) {
                *dest = *i;
            }

            else {
                bug!("Trying to select a parent token too far up");
            }
        }
    }

    /// Sets index to the index of the parent.
    pub fn up(&mut self, index: &mut usize) {
        *index = self.parent_of(*index);
    }

    /// Parent of index.
    pub fn parent_of(&self, index: usize) -> usize {
        self.tokens[index].parent
    }

    /// Returns the type of parent number `px` (0 is the parent directly above).
    pub fn px_ty(&self, index: usize, px: usize) -> TokenType {
        let mut selection = index;

        for _ in 0..=px {
            selection = self.parent_of(selection); 
        }

        self.type_of(selection)
    }

    /// Token type at index.
    pub fn type_of(&self, index: usize) -> TokenType {
        self.tokens[index].ty
    }

    /// Index of the token sharing the same parent that was added before this one.
    pub fn left_of(&self, index: usize) -> Option<usize> {
        let siblings = &self.tokens[self.tokens[index].parent].children;
        let mut alone = true;
        let mut prev = 0;

        for sibling in siblings {
            if !alone && *sibling == index {
                return Some(prev);
            }

            prev = *sibling;

            alone = false;
        }

        None
    }

    /// Index of the token sharing the same parent that was added after this one.
    pub fn right_of(&self, index: usize) -> Option<usize> {
        let mut siblings = self.tokens[self.tokens[index].parent].children.iter();

        while let Some(sibling) = siblings.next() {
            if *sibling == index {
                return siblings.next().copied();
            }
        }

        None
    }

    /// Move a token into another.
    pub fn move_into(&mut self, index: usize, dest: usize) {
        // Remove index from its current parent's 'children' vec.
        let current_parent = self.tokens[index].parent;
        let mut child_vec_index = 0; 

        for (i, child) in self.tokens[current_parent].children.iter().enumerate() {
            if *child == index {
                child_vec_index = i;

                break;
            }
        }

        self.tokens[current_parent].children.remove(child_vec_index); 

        // Assign new parent to index.
        self.tokens[index].parent = dest;

        // Add index to 'children' in dest.
        self.tokens[dest].children.push(index);
    }

    /// Create a token with only line information and a type.
    pub const fn empty(
        ty: TokenType,
        line_number: usize, 
        line: &'a str,
    ) -> ParsedToken<'a> {
        let line_number =  line_number;
        let value = Value::Void;
        let word = "";

        ParsedToken { ty, value, line_number, line, word }
    }

    /// Root of the token tree.
    pub const fn create_root() -> Token<'a> {
        Token {
            ty: Root,
            line_number: 0,
            line: "",
            word: "",
            value: Value::Void,
            index: 0,
            parent: 0,
            children: vec![],
        }
    }

    #[cfg(debug_assertions)]
    pub fn debug(&self) {
        fn children(ast: &Ast, token: &Token, indent: usize) {
            if indent >= 100 {
                println!("Recursion limit reached.");

                return;
            }

            for child_index in &token.children {
                let child = &ast.tokens[*child_index];
                let Token{ ty, line_number, .. } = *child;

                let mut n = line_number.to_string();

                if n.len() < 6 { 
                    n.push_str(&" ".repeat(7-n.len())); 
                }

                let space = if indent == 0 {
                    "".to_string()
                }else {
                    "    ".repeat(indent - 1)
                };

                let sub = if indent == 0 {
                    "" 
                }else {
                    "└── "
                };

                println!("L{}{}{}{:?}", n, space, sub, ty);

                children(ast, child, indent+1);
            }
        }

        title("Token tree");
        children(self, &self.tokens[0], 0);
        println!();
    }

}
