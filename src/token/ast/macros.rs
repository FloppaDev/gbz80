
use crate::{
    parse::{ lex::TokenType::{self, *} },
    token::{Token, Value, ast::Ast},
    error::asm::{ AsmErr, MacroMsg::{self, *} },
};

pub struct Macros {
    pub decls: Vec<usize>,
    pub calls: Vec<usize>,
}

impl<'a, 'b> Macros {

    pub const fn new() -> Self {
        Self { decls: vec![], calls: vec![] }
    }

    fn check_macro_decls(
        &self, 
        ast: &Ast<'a>,
        errors: &mut Vec<AsmErr<'a, MacroMsg>>
    ) -> bool {
        let err_count = errors.len(); 

        for macro_decl in &self.decls {
            let token = &ast.tokens[*macro_decl];
            let err_ctx = token.into();

            // First child must be the macro identifier.
            if let Some(decl_ident) = token.children.first() {
                if ast.tokens[*decl_ident].ty != MacroIdent {
                    errors.push(err!(MacroMsg, NoDeclIdent, err_ctx));
                }
            }

            else {
                errors.push(err!(MacroMsg, BadDecl, err_ctx));
                continue;
            }

            // Last child must be the body of the declaration.
            let body = ast.tokens[*macro_decl].children.last().unwrap();

            if ast.tokens[*body].ty != MacroBody {
                errors.push(err!(MacroMsg, NoDeclBody, err_ctx));
            }

            // All other tokens must be macro arguments.
            if let Some(args) = token.children.get(1..token.children.len()-1) {
                for arg in args {
                    if ast.tokens[*arg].ty != MacroArg {
                        errors.push(err!(MacroMsg, BadDeclToken, err_ctx)); 
                    }
                }
            }
        }

        err_count == errors.len()
    }

    /// Expand all macro calls from the `Ast`.
    pub fn expand(
        &self,
        ast: &'b mut Ast<'a>,
    ) -> Result<(), Vec<AsmErr<'a, MacroMsg>>> {
        let mut errors = vec![];

        // Cannot continue if macro declarations are unreliable.
        if !self.check_macro_decls(ast, &mut errors) {
            return Err(errors);
        }

        // Expand macro calls.
        for macro_call in &self.calls {
            let token = &ast.tokens[*macro_call];

            // Get the identifier of the macro call.
            let call_ident = token.children.first();

            if call_ident.is_none() {
                errors.push(err!(MacroMsg, NoCallIdent, token.into()));
                continue;
            }

            let call_ident = &ast.tokens[*call_ident.unwrap()].value.as_str();
            let mut decl_index = None;

            // Search for the corresponding declaration.
            for macro_decl in &self.decls {
                let decl_ident_token = &ast.tokens[ast.tokens[*macro_decl].children[0]];
                let decl_ident = &decl_ident_token.value.as_str();
               
                if decl_ident == call_ident {
                    decl_index = Some(macro_decl);
                    break;
                }
            }

            // Macro declaration not found.
            if decl_index.is_none() {
                errors.push(err!(MacroMsg, DeclNotFound, token.into()));
                continue;
            }

            Self::expand_call(ast, *decl_index.unwrap(), *macro_call, &mut errors); 
        }

        // Disconnect declarations from their parents to remove them from the `Ast`.
        for macro_decl in &self.decls {
            let decl_parent = ast.tokens[*macro_decl].parent;
            ast.tokens[decl_parent].children.retain(|c| *c != *macro_decl);
        }

        if errors.is_empty() {
            Ok(())
        }else {
            Err(errors)
        }
    }

    /// Expand a macro call following the given declaration.
    fn expand_call(
        ast: &'b mut Ast<'a>, 
        macro_decl: usize,
        macro_call: usize,
        errors: &mut Vec<AsmErr<'a, MacroMsg>>,
    ) {
        // Add the call's `MacroBody` as a child.
        let call_body_index = ast.tokens.len();
        ast.tokens[macro_call].children.push(call_body_index);

        let decl = &ast.tokens[macro_decl];
        let decl_args = decl.children.get(1..decl.children.len()-1).unwrap();
        let decl_body_index = decl.children.last().unwrap();

        let call = &ast.tokens[macro_call];
        let Token{ file, line_number, line, word, value, .. } = *call;

        let call_children = &call.children;
        let call_args = call_children.get(1..call_children.len()-1).unwrap();

        if decl_args.len() != call_args.len() {
            errors.push(err!(MacroMsg, ArgCountMismatch, call.into()));
            return;
        }

        // Map declared arguments to their tokens in the `Ast`.
        let mut arg_names = vec![];
        let mut arg_tokens = vec![];

        for decl_arg in decl_args {
            let decl_arg_token = &ast.tokens[*decl_arg];
            arg_names.push(decl_arg_token.value.as_str().unwrap()); 
        }

        for call_arg in call_args {
            arg_tokens.push(&ast.tokens[*call_arg]);
        }

        let call_body = Token{
            ty: MacroBody,
            file,
            line_number,
            line,
            word,
            value, 
            index: call_body_index,
            parent: macro_call,
            children: vec![],
        };

        // Create a separate `Ast` with call `MacroBody` as root.
        let mut call_ast = Ast { source: ast.source, tokens: vec![call_body] };

        call_ast = Self::copy_decl(
            ast,
            call_ast, 
            ast.tokens.len(), 
            *decl_body_index, 
            0, 
            &arg_names, 
            &arg_tokens, 
            call);

        // Append the new `Ast` to the main one.
        for token in call_ast.tokens {
            ast.tokens.push(token);
        }
    }

    /// Recursively traverse a macro declaration to expand a call into a separate Ast.
    fn copy_decl(
        ast: &Ast<'a>,
        mut call_ast: Ast<'a>,
        offset: usize,
        src: usize,
        dest: usize,
        arg_names: &[&str],
        arg_tokens: &[&'b Token<'a>],
        macro_call: &'b Token<'a>,
    ) -> Ast<'a> {
        // Iterate over all tokens inside the declaration's body.
        for child in &ast.tokens[src].children {
            let mut child = &ast.tokens[*child];
            let parent_ty = ast.type_of(child.parent);

            // Macro arguments must be replaced with the corresponding tokens.
            if child.ty == MacroArg {
                let arg = child.value.as_str().unwrap();

                for (i, arg_name) in arg_names.iter().enumerate() {
                    if *arg_name == arg {
                        child = arg_tokens[i];
                        break;
                    }
                }

                if parent_ty == Instruction {
                    // Add Argument.
                    let ty = Argument;
                    let value = Value::Void;

                    let index = Self::push_arg(
                        &mut call_ast, offset, macro_call, ty, value, dest);

                    // Add child.
                    let Token{ ty, value, .. } = *child;

                    let index = Self::push_arg(
                        &mut call_ast, offset, macro_call, ty, value, index);

                    call_ast = Self::copy_decl(
                        ast, 
                        call_ast, 
                        offset, 
                        child.index, 
                        index, 
                        arg_names, 
                        arg_tokens, 
                        macro_call);

                    continue;
                }
            }

            // Add child.
            let Token{ ty, value, .. } = *child;

            let index = Self::push_arg(
                &mut call_ast, offset, macro_call, ty, value, dest);
            
            call_ast = Self::copy_decl(
                ast, 
                call_ast, 
                offset, 
                child.index, 
                index, 
                arg_names, 
                arg_tokens, 
                macro_call);
        }

        call_ast
    }

    fn push_arg(
        call_ast: &mut Ast<'a>,
        offset: usize,
        macro_call: &'b Token<'a>,
        ty: TokenType,
        value: Value<'a>,
        dest: usize,
    ) -> usize {
        // `call_ast` will be merged into `ast` after the declaration has been copied.
        let index = call_ast.tokens.len();
        call_ast.tokens[dest].children.push(offset + index);

        let Token{ file, line_number, line, word, .. } = *macro_call;
        let children = vec![];
        let parent = offset + dest;

        let token = Token{
            ty, file, line_number, line, word, value, index, parent, children
        };

        call_ast.tokens.push(token);

        index
    }

}
