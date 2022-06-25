
/// Partial implementation of Ast for manipulating the tree.
pub mod utils;

/// Expands macros.
pub mod macros;

use crate::{
    parse::{
        lex::TokenType::*,
        prepare::ParsedToken,
        source::Source,
    },
    token::{
        Token,
        expr,
        ast::macros::Macros,
    },
    error::{
        ErrCtx, 
        ITERATION_LIMIT,
        asm::{
            AsmErr,
            AstMsg::{self, *},
        },
    },
};

/// Abstract Token Tree.
/// The whole hierarchy of parsed tokens from the source file.
#[derive(Debug)]
pub struct Ast<'a> {
    pub source: &'a Source,
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
                    return Err(errors)
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

                    _ => {
                        self.up(selection);
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

                Label => 
                    self.cascade(selection, &[Marker], token, None),

                _ => 
                    self.cascade(selection, &[], token, None),
            }
        }
    }
    
}
