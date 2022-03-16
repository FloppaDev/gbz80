
use crate::{
    parse::{
        lex::TokenType::{self, *},
        prepare::ParsedToken,
    },
    token::{
        Token, Value,
        macros::Macros,
    },
    program::{
        RECURSION_LIMIT,
        error::{ErrCtx, AstErr, AstErrType::*},
    },
};

#[cfg(debug_assertions)]
use crate::program::title;

/// Abstract Token Tree.
/// The whole hierarchy of parsed tokens from the source file.
#[derive(Debug)]
pub struct Ast<'a> {
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Ast<'a> {

    pub fn get_root(&self) -> &Token<'a> {
        &self.tokens[0]
    }

    /// Assemble the tokens into a tree.
    pub fn new(
        tokens: Vec<ParsedToken<'a>>,
        macros: &mut Macros,
    ) -> Result<Self, Vec<AstErr<'a>>> {
        if tokens.is_empty() {
            let e = err!(AstErr, NoTokens, ErrCtx::new(0, "", ""));
            return Err(vec![e]);
        }

        // All errors go in there. The goal is to try to recover
        // from errors so that we can keep processing all tokens and then
        // report all errors at the same time.
        let mut errors = vec![];

        // Initialize the tree with a Root token and select it.
        let root = Self::create_root();
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

            // Is the token on a new line.
            if current_line != token.line_number {
                current_line = token.line_number;

                // Update selection after the end of a line.
                if ast.newline(&mut selection, &mut errors).is_err() {
                    return Err(errors)
                }
            }

            ast.process_token(token, &mut selection, macros, &mut errors);
        }

        // Run `newline` for the last line too.
        let _ = ast.newline(&mut selection, &mut errors);

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
        selection: &mut usize,
        errors: &mut Vec<AstErr<'a>>,
    ) -> Result<(), ()> {
        let token = &self.tokens[*selection];
        let Token{ line_number, line, word, .. } = *token;
        let err_ctx = token.into();
        let mut fail_safe = RECURSION_LIMIT;

        // Check for tokens that end on a new line and close them.
        loop {
            let sel_ty = self.type_of(*selection);

            if fail_safe == 0 {
                let e = err!(AstErr, UnhandledNewline(sel_ty), err_ctx);
                errors.push(e);

                return Err(());
            }

            fail_safe -= 1;

            if sel_ty.ends_on_newline() {
                match sel_ty {
                    // If it's a macro declaration, add a new macro body.
                    Macro => {
                        let t = Self::empty(MacroBody, line_number, line);
                        *selection = self.push(*selection, t);        

                        break;
                    }

                    At => {
                        let e = err!(AstErr, UnmatchedParen, err_ctx);
                        errors.push(e);
                        *selection = self.parent_of(*selection);
                    }
                    
                    NamedMark|AnonMark if self.tokens[*selection].children.is_empty() => {
                        let e = err!(AstErr, MarkWithoutLiteral, err_ctx);
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
        token: ParsedToken<'a>,
        selection: &mut usize,
        macros: &mut Macros,
        errors: &mut Vec<AstErr<'a>>,
    ) {
        let ParsedToken{ line_number, line, word, .. } = token;
        let err_ctx: ErrCtx = (&token).into();

        // Match parent type of the token.
        match token.ty.parent_type() {
            p @ InstrName => {
                *selection = self.cascade(*selection, &[Instruction, p], token);
            }

            p @ (Register|Flag|Lit) => {
                self.cascade(*selection, &[p], token);
            }

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

                DefB|DefW|DefS|Include => {
                    let t = Self::empty(Directive, line_number, line);
                    *selection = self.push(*selection, t);
                    *selection = self.push(*selection, token);
                }

                AnonMark|NamedMark => {
                    let t = Self::empty(Marker, line_number, line);
                    *selection = self.push(*selection, t);
                    *selection = self.push(*selection, token);
                }

                Repeat => {
                    self.push(*selection, token);
                }

                Label => {
                    let t = Self::empty(Marker, line_number, line);
                    let marker = self.push(*selection, t);
                    self.push(marker, token);
                }

                _ => {
                    let e = err!(AstErr, UnknownError, err_ctx);
                    errors.push(e);
                }
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

        title("Token tree");
        children(self, &self.tokens[0], 0);
        println!();
    }
}

