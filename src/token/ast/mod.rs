
pub mod utils;
pub mod macros;

use crate::{
    parse::{
        lex::TokenType::*,
        prepare::ParsedToken,
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
    ) -> Result<Self, Vec<AsmErr<'a, AstMsg>>> {
        if tokens.is_empty() {
            let e = err!(AstMsg, NoTokens, ErrCtx::new(Root, 0, "", ""));
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
        let Token{ line_number, line, .. } = *token;
        let err_ctx = token.into();
        let mut fail_safe = ITERATION_LIMIT;

        // Check for tokens that end on a new line and close them.
        loop {
            let sel_ty = self.type_of(*selection);

            if fail_safe == 0 {
                let e = err!(AstMsg, UnhandledNewline, err_ctx);
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
                        let e = err!(AstMsg, UnmatchedParen, err_ctx);
                        errors.push(e);
                        *selection = self.parent_of(*selection);
                    }
                    
                    NamedMark|AnonMark if self.tokens[*selection].children.is_empty() => {
                        let e = err!(AstMsg, MarkWithoutLiteral, err_ctx);
                        errors.push(e);
                        *selection = self.parent_of(*selection);
                    }

                    Expr => {
                        if let Err(e) = expr::build(self, *selection) {
                            errors.push(e);
                        }

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
    ) {
        let ParsedToken{ line_number, line, .. } = token;

        // Match parent type of the token.
        match token.ty.parent_type() {
            p @ InstrName => {
                *selection = self.cascade(*selection, &[Instruction, p], token);
            }

            p @ (Register|Flag|Lit) => {
                if self.type_of(*selection) == Argument {
                    *selection = self.parent_of(*selection);
                }

                if self.type_of(*selection) == Instruction {
                    self.cascade(*selection, &[Argument, p], token);
                }

                else {
                    self.cascade(*selection, &[p], token);

                    if self.type_of(*selection) == At {
                        *selection = self.parent_of(*selection);
                    }
                }

                if matches!(self.type_of(*selection), AnonMark|NamedMark) {
                    if p == Lit {
                        *selection = self.parent_of(*selection);
                        *selection = self.parent_of(*selection);
                    }

                    else {
                        bug!("Invalid token after Marker");
                    }
                }
            }

            Macro => {
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
                    if self.type_of(*selection) == Argument {
                        *selection = self.parent_of(*selection);
                    }

                    if self.type_of(*selection) == Instruction {
                        self.cascade(*selection, &[Argument], token);
                    }

                    else {
                        //TODO completely replace push by cascade.
                        self.cascade(*selection, &[], token);

                        if self.type_of(*selection) == At {
                            *selection = self.parent_of(*selection);
                        }
                        
                        // TODO check what DefS does. It should not be an Expr
                        // and should accept only a LitStr.
                        else if matches!(self.type_of(*selection), DefB|DefW) {
                            let t = Self::empty(Expr, line_number, line);
                            *selection = self.push(*selection, t);
                        }
                    }
                }

                // Open parenthesis.
                At0 => {
                    let at = Self::empty(At, line_number, line);

                    if self.type_of(*selection) == Instruction {
                        let arg = Self::empty(Argument, line_number, line);
                        *selection = self.push(*selection, arg);
                        *selection = self.push(*selection, at);
                    }

                    else {
                        *selection = self.push(*selection, at);
                    }
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

                DefB|DefW|Include => {
                    let t = Self::empty(Directive, line_number, line);
                    *selection = self.push(*selection, t);
                    *selection = self.push(*selection, token);
                }

                AnonMark|NamedMark => {
                    let t = Self::empty(Marker, line_number, line);
                    *selection = self.push(*selection, t);
                    *selection = self.push(*selection, token);
                }

                Label => {
                    self.cascade(*selection, &[Marker], token);
                }

                _ => {
                    self.push(*selection, token);
                }
            }
        }
    }
    
}
