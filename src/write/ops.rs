
use crate::{
    write::instructions,
    parse::lex::TokenType::{self, *},
    token::{ read::TokenRef, Value },
    error::asm::{ AsmErr, OpMsg::{self, *} },
};

use Constant::*;

use std::collections::HashMap;

pub struct OpMap<'a>(HashMap<&'a TokenRef<'a>, OpCode>);

impl<'a> OpMap<'a> {

    pub fn new(ast: &'a TokenRef<'a>) -> Result<Self, Vec<AsmErr<'a, OpMsg>>> {
        let mut map = HashMap::new(); 
        let mut errors = vec![];

        Self::walk(ast, &mut map, &mut errors);

        if !errors.is_empty() {
            Err(errors)
        }else {
            Ok(Self(map))
        }
    }

    pub fn get(&self, token: &TokenRef<'a>) -> &OpCode {
        let Self(map) = self; 
        map.get(token).unwrap()
    }

    fn walk(
        ast: &'a TokenRef<'a>,
        map: &mut HashMap<&TokenRef<'a>, OpCode>, 
        errors: &mut Vec<AsmErr<'a, OpMsg>>,
    ) {
        for token in ast.children() {
            match token.ty() {
                MacroCall => {
                    for child in token.children() {
                        if child.ty() == MacroBody {
                            Self::walk(child, map, errors);
                        }
                    }
                }

                Instruction => {
                    let opcode = instructions::find(token);

                    if opcode.is_none() {
                        errors.push(err!(OpMsg, NotFound, token.into()));
                        continue;
                    }

                    map.insert(token, opcode.unwrap());
                }

                _ => {}
            }
        }
    }

}

#[derive(Debug)]
pub struct OpCode {
    pub cb: bool,
    pub code: u8,
    pub len: u8,
}

impl OpCode {

    pub fn get_opcode(
        instruction: &TokenRef, 
        cb: bool, 
        ops: Vec<(u8, u8, Vec<Arg>)>
    ) -> Option<Self> {
        let instr_children = instruction.children()[1..].iter()
            .map(|c| c.first())
            .collect::<Vec<_>>();

        for op in ops {
            let (len, code, op_args) = op;

            if Self::cmp_args(&instr_children, &op_args) {
                let opcode = Self{ cb, code, len };
                return Some(opcode);     
            }
        }

        None
    }

    fn cmp_args(instr_args: &[&TokenRef], op_args: &[Arg]) -> bool {
        if instr_args.len() > op_args.len() {
            return false;
        }

        if op_args.is_empty() && instr_args.is_empty() {
            return true;
        }

        if let Arg::Token(ty) = op_args[0] {
            if ty == A {
                if instr_args.is_empty() && op_args.len() == 1 {
                    return true;
                }

                if instr_args.len() == 1 && op_args.len() == 2 {
                    return op_args[1].cmp(instr_args[0]); 
                }
            }
        }

        if op_args.len() != instr_args.len() {
            return false;
        }

        for i in 0..instr_args.len() {
            if !op_args[i].cmp(instr_args[i]) {
                return false;
            }
        }

        true
    }

}

#[derive(Debug)]
pub enum Constant {
    BitN(u8),
    Byte,
    Word,
}

impl Constant {

    fn cmp(&self, token: &TokenRef) -> bool {
        if token.ty() == Identifier {
            return true;
        }

        match self {
            Byte => {
                match token.ty() {
                    LitDec|LitHex|LitBin => matches!(token.value(), Value::U8(_)),
                    LitStr => token.value().as_str().unwrap().len() == 1,
                    _ => false
                }
            }

            Word => {
                match token.ty() {
                    LitDec|LitHex|LitBin => matches!(token.value(), Value::U16(_)),
                    LitStr => token.value().as_str().unwrap().len() == 1,
                    _ => false
                }
            }

            BitN(b) => token.value().as_u8().map_or(false, |value|
                (token.ty() == LitDec) && (value == *b))
        }
    }

}

pub const fn ty(ty: TokenType) -> Arg { Arg::Token(ty) }
pub fn at(arg: Arg) -> Arg { Arg::At(Box::new(arg)) }
pub const fn imm(c: Constant) -> Arg { Arg::Const(c) }
pub const fn bit(n: u8) -> Arg { Arg::Const(Constant::BitN(n)) }

#[derive(Debug)]
pub enum Arg {
    /// Address.
    At(Box<Arg>),
    /// Identified by a `TokenType`.
    Token(TokenType),
    /// Constant value.
    Const(Constant),
}

impl Arg {

    fn cmp(&self, token: &TokenRef) -> bool {
        if token.ty() == At {
            if let Self::At(arg) = self {
                return arg.cmp(token.get(0));
            }

            return false;
        }

        match self {
            Self::Token(ty) => token.leaf().ty() == *ty,
            Self::Const(constant) => constant.cmp(token.leaf()),
            _ => false
        }
    }

}
