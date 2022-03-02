
use crate::{
    data::Key,
    lingo::{ Lexicon, TokenType::{ self, * } },
    parse::ParsedToken,
    error::{ ErrCtx, AstErr, AstErrType },
    macros::Macros,
};

use std::hash::{Hash, Hasher};

/// Token within the tree.
#[derive(Debug)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub line_number: usize,
    pub line: &'a str,
    pub word: &'a str,
    pub data_key: Key,
    pub index: usize,
    pub parent: usize,
    pub children: Vec<usize>,
}

impl<'a> Token<'a> {

    /// Create a new `Token` from `ParsedToken`.
    const fn new(
        ParsedToken{ ty, line_number, line, word, data_key }: ParsedToken<'a>, 
        index: usize, 
        parent: usize,
    ) -> Self {
        let children = vec![];
        Self { ty, line_number, line, word, data_key, index, parent, children }
    }

}

/// Abstract Token Tree.
/// The whole hierarchy of parsed tokens from the source file.
pub struct Ast<'a> {
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Ast<'a> {

    pub fn get_root(&self) -> &Token<'a> {
        &self.tokens[0]
    }

    /// Assemble the tokens into a tree.
    pub fn new(
        lexicon: &Lexicon, 
        tokens: Vec<ParsedToken<'a>>,
        macros: &mut Macros,
    ) -> Result<Self, Vec<AstErr<'a>>> {
        if tokens.is_empty() {
            let e = AstErr::new(AstErrType::NoTokens, ErrCtx::new(0, "", ""));
            return Err(vec![e]);
        }

        // All errors go in there. The goal is to try to recover
        // from errors so that we can keep processing all tokens and then
        // report all errors at the same time.
        let mut errors = vec![];

        // Initialize the tree with a Root token and select it.
        let root = Self::root();
        let mut ast = Self { tokens: vec![root] };
        let mut selection = 0;
        let mut current_line = 0;

        for token in tokens {
            let ParsedToken{ line_number, line, word, .. } = token;

            // `NamedMark` and `AnonMark` need one child.
            if matches!(ast.type_of(selection), AnonMark|NamedMark)
            && ast.tokens[selection].children.len() == 1 {
                selection = ast.parent_of(selection);
            }

            //TODO repeat

            // Is the token on a new line.
            if current_line != token.line_number {
                current_line = token.line_number;

                // Update selection after the end of a line.
                if ast.newline(lexicon, &mut selection, &mut errors).is_err() {
                    return Err(errors)
                }
            }

            ast.process_token(lexicon, token, &mut selection, macros, &mut errors);
        }

        // Run `newline` for the last line too.
        let _ = ast.newline(lexicon, &mut selection, &mut errors);

        if errors.is_empty() {
            Ok(ast)
        }else {
            Err(errors)
        }
    }

    /// Update selection for the new line. 
    /// Return Err(()) if an unrecoverable error occured.
    fn newline(
        &mut self,
        lexicon : &Lexicon,
        selection: &mut usize,
        errors: &mut Vec<AstErr<'a>>,
    ) -> Result<(), ()> {
        let token = &self.tokens[*selection];
        let Token{ line_number, line, word, .. } = *token;
        let err_ctx = token.into();

        // Fail-safe in case variants are added and not handled.
        let mut loop_counter = 0;

        // Check for tokens that end on a new line and close them.
        loop {
            let sel_ty = self.type_of(*selection);

            if loop_counter >= 1000 {
                let e = AstErr::new(AstErrType::UnhandledNewline(sel_ty), err_ctx);
                errors.push(e);

                return Err(());
            }

            loop_counter += 1;

            if lexicon.ends_on_newline(sel_ty) {
                match sel_ty {
                    // If it's a macro declaration, add a new macro body.
                    Macro => {
                        let t = Self::empty(MacroBody, line_number, line);
                        *selection = self.push(*selection, t);        

                        break;
                    }

                    At => {
                        let e = AstErr::new(AstErrType::UnmatchedParen, err_ctx);
                        errors.push(e);
                        *selection = self.parent_of(*selection);
                    }
                    
                    NamedMark|AnonMark if self.tokens[*selection].children.is_empty() => {
                        let e = AstErr::new(AstErrType::MarkWithoutLiteral, err_ctx);
                        errors.push(e);
                        *selection = self.parent_of(*selection);
                    }

                    _ => {
                        *selection = self.parent_of(*selection);
                    }
                }
            }

            else {
                break;
            }
        }

        Ok(())
    }

    /// Insert the token into the tree and update selection.
    fn process_token(
        &mut self,
        lexicon: &Lexicon,
        token: ParsedToken<'a>,
        selection: &mut usize,
        macros: &mut Macros,
        errors: &mut Vec<AstErr<'a>>,
    ) {
        let ParsedToken{ line_number, line, word, .. } = token;
        let err_ctx: ErrCtx = (&token).into();

        // Match parent type of the token.
        match lexicon.parent_type(token.ty) {
            p @ InstrName => {
                *selection = self.cascade(*selection, &[Instruction, p], token);
            }

            p @ (Register|Flag|Lit) => {
                self.cascade(*selection, &[p], token);
            }

            //TODO take repeat count
            p @ Macro => {
                // Is it a macro call? 
                if token.ty == MacroIdent && self.type_of(*selection) != Macro {
                    *selection = self.cascade(*selection, &[MacroCall], token);
                    macros.calls.push(*selection);
                }

                else {
                    self.push(*selection, token);
                }
            }

            _ => match token.ty {
                Identifier => {
                    self.cascade(*selection, &[], token);
                }

                // Open parenthesis.
                At0 => {
                    let t = Self::empty(At, line_number, line);
                    *selection = self.push(*selection, t);
                }

                // Close parenthesis.
                At1 => *selection = self.parent_of(*selection),

                // Macro declaration.
                Macro => {
                    // Are we already in the body of the macro declaration?
                    // (Which starts on the second line of the declaration)
                    if self.type_of(*selection) == MacroBody {
                        // Close macro body and macro declaration. 
                        *selection = self.parent_of(self.parent_of(*selection));
                    }

                    else {
                        let t = Self::empty(Macro, line_number, line);
                        *selection = self.push(*selection, t);
                        macros.decls.push(*selection);
                    }
                }

                Define|Include => {
                    let t = Self::empty(Directive, line_number, line);
                    *selection = self.push(*selection, t);
                    *selection = self.push(*selection, token);
                }

                AnonMark|NamedMark => {
                    *selection = self.push(*selection, token);
                }

                _ => {}
            }
        }
    }

    /// Push a new token as child into destination and return its index.
    fn push(&mut self, dest: usize, token: ParsedToken<'a>) -> usize {
        // Create the Token
        let index = self.tokens.len();
        let mut token = Token::new(token, index, dest);

        // Push the token and it index.
        self.tokens.push(token);
        self.tokens[dest].children.push(index);

        index
    }

    /// Push new tokens of parent types, and parent them in a cascade with 'token' as final child. 
    /// Parent tokens inherit line data from 'token' but not the data key.
    fn cascade(
        &mut self, 
        dest: usize, 
        parent_types: &[TokenType], 
        token: ParsedToken<'a>
    ) -> usize {
        let ParsedToken{ line_number, line, .. } = token;

        let mut selection = dest;
        let mut oldest_parent = dest;

        for ty in parent_types {
            let parent = Self::empty(*ty, line_number, line);
            selection = self.push(selection, parent);

            if oldest_parent== dest {
                oldest_parent = selection;
            }
        }

        self.push(selection, token);

        oldest_parent 
    }

    /// Parent of index.
    fn parent_of(&self, index: usize) -> usize {
        self.tokens[index].parent
    }

    /// Token type at index.
    fn type_of(&self, index: usize) -> TokenType {
        self.tokens[index].ty
    }

    /// Index of the token sharing the same parent that was added before this one.
    fn older_sibling_of(&self, index: usize) -> Option<usize> {
        let siblings = &self.tokens[self.tokens[index].parent].children;
        let mut alone = true;
        let mut prev = 0;

        for sibling in siblings {
            if alone {
                alone = false;
            }

            // The indices match and there is a token before this one.
            else if *sibling == index {
                return Some(prev);
            }

            prev = *sibling;
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
    const fn empty(
        ty: TokenType,
        line_number: usize, 
        line: &'a str,
    ) -> ParsedToken<'a> {
        let line_number =  line_number;
        let data_key = Key::void();
        let word = "";

        ParsedToken { ty, data_key, line_number, line, word }
    }

    /// Root of the token tree.
    //TODO rename to 'create_root'
    pub const fn root() -> Token<'a> {
        Token {
            ty: Root,
            line_number: 0,
            line: "",
            word: "",
            data_key: Key::void(),
            index: 0,
            parent: 0,
            children: vec![],
        }
    }

    #[cfg(feature = "debug")]
    pub fn debug(&self) {
        fn children(ast: &Ast, token: &Token, indent: usize) {
            if indent >= 100 {
                println!("Recursion limit reached.");

                return;
            }

            for child_index in &token.children {
                let child = &ast.tokens[*child_index];
                let Token{ ty, line_number, line, .. } = *child;

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

        use crate::process::title;

        title("Token tree");
        children(self, &self.tokens[0], 0);
        println!();
    }

}

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
        let mut fail_safe = 500;
        let root = ast.get_root();
        let mut current = Self{ 
            ast, token: root, parent: 0 as *const _, children: vec![]
        };
        current.parent = &current;

        Self::walk(ast, &mut current, &mut fail_safe); 

        current
    }

    fn walk(
        ast: &'a Ast, 
        current: &mut Self,
        fail_safe: &mut usize,
    ) {
        if *fail_safe == 0 {
            panic!("Assembler bug: Recursion limit reached while building TokenRef tree.");
        }

        for child in &current.token.children {
            *fail_safe -= 1;
            let token = &ast.tokens[*child];
            let mut token_ref = Self{ 
                ast, token, parent: current, children: vec![]
            };

            current.children.push(token_ref);

            Self::walk(
                ast, current.children.last_mut().unwrap(), fail_safe);
        }
    }

    pub fn ast(&self) -> &Ast {
        self.ast
    }

    pub fn token(&self) -> &Token<'a> {
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

    pub fn ty(&self) -> TokenType {
        self.token.ty
    }

    pub fn line_number(&self) -> usize {
        self.token.line_number
    }

    pub fn line(&self) -> &'a str {
        self.token.line
    }

    pub fn word(&self) -> &'a str {
        self.token.word
    }

    pub fn data_key(&self) -> &Key {
        &self.token.data_key
    }

    pub fn index(&self) -> usize {
        self.token.index
    }

    /// Returns a reference to the parent `TokenRef` or `self` if it is the root.
    pub fn parent(&self) -> &Self {
        // `TokenRef` contains an immutable ref to the Ast so its safe.
        unsafe { &*self.parent }
    }

    /// Returns reference to all `TokenRef` children.
    pub fn children(&self) -> Vec<&Self> {
        self.children.iter().collect::<Vec<_>>()
    }

}
