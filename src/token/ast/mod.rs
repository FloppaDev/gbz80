
/// Expands macros.
pub mod macros;

use crate::{
    parse::{ lex::TokenType::*, prepare::ParsedToken, source::Source },
    token::{ Token, TokenType, Value, expr, ast::macros::Macros },
    error::{ ErrCtx, ITERATION_LIMIT, asm::{ AsmErr, AstMsg::{self, *} } },
};

#[cfg(debug_assertions)]
use crate::program::fmt::title;

/// Abstract Token Tree.
/// The whole hierarchy of parsed tokens from the source file.
#[derive(Debug)]
pub struct Ast<'a> {
    pub source: &'a Source,
    pub tokens: Vec<Token<'a>>,
}

impl<'a> Ast<'a> {

    /// Assemble the tokens into a tree.
    pub fn new(
        tokens: Vec<ParsedToken<'a>>,
        macros: &mut Macros,
        source: &'a Source,
    ) -> Result<Self, Vec<AsmErr<'a, AstMsg>>> {
        if tokens.is_empty() {
            let e = err!(AstMsg, NoTokens, ErrCtx::new(Root, "", 0, "", ""));
            return Err(vec![e]);
        }

        // All errors go in there. The goal is to try to recover
        // from errors so that we can keep processing all tokens and then
        // report all errors at the same time.
        let mut errors = vec![];

        // Initialize the tree with a Root token and select it.
        let root = Self::create_root();
        let mut ast = Self { source, tokens: vec![root] };
        let mut selection = 0;
        let mut current_line = 0;

        for token in tokens {
            // `NamedMark` and `AnonMark` need one child.
            if matches!(ast.type_of(selection), AnonMark|NamedMark)
            && ast.tokens[selection].children.len() == 1 {
                ast.up(&mut selection);
            }

            // Is the token on a new line.
            if current_line != token.line_number {
                current_line = token.line_number;

                // Update selection after the end of a line.
                if ast.newline(&mut selection, &mut errors).is_err() {
                    return Err(errors);
                }
            }

            ast.process_token(token, &mut selection, macros);
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
        errors: &mut Vec<AsmErr<'a, AstMsg>>,
    ) -> Result<(), ()> {
        let token = &self.tokens[*selection];
        let Token{ file, line_number, line, .. } = *token;
        let err_ctx = token.into();
        let mut fail_safe = ITERATION_LIMIT;

        // Check for tokens that end on a new line and close them.
        loop {
            let sel_ty = self.type_of(*selection);

            if fail_safe == 0 {
                bug!("Could not process newline");
            }

            fail_safe -= 1;

            if sel_ty.ends_on_newline() {
                match sel_ty {
                    // If it's a macro declaration, add a new macro body.
                    Macro => {
                        let t = Self::empty(MacroBody, file, line_number, line);
                        self.cascade(selection, &[], t, Some(0));        
                        break;
                    }

                    At => {
                        let e = err!(AstMsg, UnmatchedParen, err_ctx);
                        errors.push(e);
                        self.up(selection);
                    }
                    
                    NamedMark|AnonMark if self.tokens[*selection].children.is_empty() => {
                        let e = err!(AstMsg, MarkWithoutLiteral, err_ctx);
                        errors.push(e);
                        self.up(selection);
                    }

                    Expr => {
                        if let Err(e) = expr::build(self, *selection) {
                            errors.push(e);
                        }

                        self.up(selection);
                    }

                    _ => self.up(selection)
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
    ) {
        let ParsedToken{ file, line_number, line, .. } = token;

        // Match parent type of the token.
        match token.ty.parent_type() {
            p @ InstrName =>
                self.cascade(selection, &[Instruction, p], token, Some(2)),

            p @ (Register|Flag|Lit) => {
                if self.type_of(*selection) == Argument {
                    self.up(selection);
                }

                if self.type_of(*selection) == Instruction {
                    self.cascade(selection, &[Argument, p], token, None);
                }

                else {
                    self.cascade(selection, &[p], token, None);
                    let s = *selection;

                    if self.px_ty(s, 0) == At && self.px_ty(s, 1) == Argument {
                        self.up(selection);
                    }
                }

                if matches!(self.type_of(*selection), AnonMark|NamedMark) {
                    if p == Lit || cfg!(test) {
                        self.up(selection);
                        self.up(selection);
                    }

                    else {
                        bug!("Invalid token after Marker");
                    }
                }
            }

            Macro => {
                // Is it a macro call? 
                if token.ty == MacroIdent && self.type_of(*selection) != Macro {
                    self.cascade(selection, &[MacroCall], token, Some(1));
                    macros.calls.push(*selection);
                }

                else {
                    self.cascade(selection, &[], token, None);
                }
            }

            _ => match token.ty {
                Identifier => {
                    if self.type_of(*selection) == Argument {
                        self.up(selection);
                    }

                    if self.type_of(*selection) == Instruction {
                        self.cascade(selection, &[Argument], token, None);
                    }

                    else {
                        self.cascade(selection, &[], token, None);
                        let s = *selection;

                        if self.px_ty(s, 0) == At && self.px_ty(s, 1) == Argument {
                            self.up(selection);
                        }
                        
                        else if matches!(self.type_of(*selection), DefB|DefW) {
                            let t = Self::empty(Expr, file, line_number, line);
                            self.cascade(selection, &[], t, Some(0));
                        }
                    }
                }

                // Open parenthesis.
                At0 => {
                    let at = Self::empty(At, file, line_number, line);

                    if self.type_of(*selection) == Instruction {
                        self.cascade(selection, &[Argument], at, Some(0));
                    }

                    else {
                        self.cascade(selection, &[], at, Some(0));
                    }
                }

                // Close parenthesis.
                At1 => self.up(selection),

                // Macro declaration.
                Macro => {
                    // Are we already in the body of the macro declaration?
                    // (Which starts on the second line of the declaration)
                    if self.type_of(*selection) == MacroBody {
                        // Close macro body and macro declaration. 
                        self.up(selection);
                        self.up(selection);
                    }

                    else {
                        let t = Self::empty(Macro, file, line_number, line);
                        self.cascade(selection, &[], t, Some(0));
                        macros.decls.push(*selection);
                    }
                }

                ty@(DefB|DefW|Include|Import|AnonMark|NamedMark) => 
                    self.cascade(selection, &[ty.parent_type()], token, Some(0)),

                Label => self.cascade(selection, &[Marker], token, None),

                _ => self.cascade(selection, &[], token, None),
            }
        }
    }

    pub(super) fn get_root(&self) -> &Token<'a> {
        &self.tokens[0]
    }

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
    fn cascade(
        &mut self, 
        dest: &mut usize, 
        parent_types: &[TokenType], 
        token: ParsedToken<'a>,
        select_up: Option<usize>,
    ) {
        let ParsedToken{ file, line_number, line, .. } = token;

        let mut selection = *dest;
        let mut inserts = vec![];

        for ty in parent_types {
            let parent = Self::empty(*ty, file, line_number, line);
            selection = self.push(selection, parent);
            inserts.push(self.tokens.len()-1);
        }

        self.push(selection, token);
        inserts.push(self.tokens.len()-1);

        if let Some(select_up) = select_up {
            inserts.iter().rev().nth(select_up).map_or_else(|| {
                bug!("Trying to select a parent token too far up.");
            }, |i| {
                *dest = *i;
            });
        }
    }

    /// Sets index to the index of the parent.
    fn up(&mut self, index: &mut usize) {
        *index = self.parent_of(*index);
    }

    /// Parent of index.
    fn parent_of(&self, index: usize) -> usize {
        self.tokens[index].parent
    }

    /// Returns the type of parent number `px` (0 is the parent directly above).
    fn px_ty(&self, index: usize, px: usize) -> TokenType {
        let mut selection = index;

        for _ in 0..=px {
            selection = self.parent_of(selection); 
        }

        self.type_of(selection)
    }

    /// Token type at index.
    fn type_of(&self, index: usize) -> TokenType {
        self.tokens[index].ty
    }

    /// Index of the token sharing the same parent that was added before this one.
    pub(super) fn left_of(&self, index: usize) -> Option<usize> {
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
    pub(super) fn right_of(&self, index: usize) -> Option<usize> {
        let mut siblings = self.tokens[self.tokens[index].parent].children.iter();

        while let Some(sibling) = siblings.next() {
            if *sibling == index {
                return siblings.next().copied();
            }
        }

        None
    }

    /// Move a token into another.
    pub(super) fn move_into(&mut self, index: usize, dest: usize) {
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

        // Assign new parent to index and add index to `children` in `dest`.
        self.tokens[index].parent = dest;
        self.tokens[dest].children.push(index);
    }

    /// Create a token with only line information and a type.
    const fn empty(
        ty: TokenType,
        file: &'a str,
        line_number: usize, 
        line: &'a str,
    ) -> ParsedToken<'a> {
        ParsedToken { ty, value: Value::Void, file, line_number, line, word: "" }
    }

    /// Root of the token tree.
    const fn create_root() -> Token<'a> {
        Token {
            ty: Root,
            file: "",
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
            if indent >= ITERATION_LIMIT {
                println!("Iteration limit reached.");
                return;
            }

            for child_index in &token.children {
                let child = &ast.tokens[*child_index];
                let Token{ ty, line_number, .. } = *child;
                let mut n = line_number.to_string();

                if n.len() < 6 { 
                    n.push_str(&" ".repeat(7-n.len())); 
                }

                let space = if indent == 0 { String::new() }else{ "    ".repeat(indent - 1) };
                let sub = if indent == 0 { "" }else{ "└── " };

                println!("L{n}{space}{sub}{ty:?}");
                children(ast, child, indent+1);
            }
        }

        title("Token tree");
        children(self, &self.tokens[0], 0);
        println!();
    }
    
}
