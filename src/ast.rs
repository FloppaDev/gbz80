#![allow(clippy::zero_ptr)]

use crate::utils;
use crate::split::Split;
use self::TokenType::*;
use crate::opcodes::InstructionDef;

#[derive(Debug)]
pub struct Token {
    pub line: usize,
    pub ty: TokenType,
    pub parent: *mut Token,
    pub children: Vec<Token>,
    pub value: String,
}

impl Token {
    /// Root of the tree, it only contains children.
    pub fn root() -> Self {
        Self {
            line: 0,
            ty: TokenType::Unknown,
            parent: 0 as *mut Token,
            children: vec![],
            value: String::new(),
        }
    }

    /// Add a new token as a child of self.
    pub fn push(&mut self, line: usize, ty: TokenType, value: String) -> &mut Token {
        let parent: *mut _ = self;
        let children = vec![];
        self.children.push(Token {line, ty, parent, children, value});
        let i = self.children.len() - 1;
        &mut self.children[i]
    }

    /// Move token to self as child
    pub fn transfer(&mut self, token: Token) {
        self.children.push(token);
    }

    /// Create a copy of the token and its children.
    pub fn clone(&self) -> Token {
        let mut children = vec![];

        for child in &self.children {
            children.push(child.clone());
        }

        Self {
            line: self.line,
            ty: self.ty,
            parent: self.parent,
            children,
            value: self.value.clone(),
        }
    }

    /// Copy data from token to self.
    pub fn copy(&mut self, token: &Token) {
        let mut children = vec![];

        for child in &token.children {
            children.push(child.clone());
        }

        self.line = token.line;
        self.ty = token.ty;
        self.parent = token.parent;
        self.children = children;
        self.value = token.value.clone();
    }

    /// Build the AST from split data
    pub fn make_ast(split: Split, instructions: &[InstructionDef]) -> IntermediateAST {
        let base_tokens = Token::get_base_tokens(&split); 
        let int_ast = unsafe { Token::make_tree(base_tokens) };
        #[cfg(feature = "debug")] {
            utils::debug_title("AST data");
            int_ast.root.debug();
            utils::debug_title("Macros expansion");
        }

        int_ast.expand()
    }

    /// Create a root token and adds all the identified tokens as children
    fn get_base_tokens(split: &Split) -> Token {
        // Root of the token tree, for now it will contain all tokens as direct children.
        let mut root = Token::root();

        for (i, line) in split.lines.iter().enumerate() {

            for word in &line.words {
                let word = &split.input[word.start..word.end];
                let c = word.chars().next().unwrap();

                let mut directive = false;

                // Push a one or more tokens for this word
                macro_rules! pushn { ($tt:ident, $str:expr) => {
                    root.push(line.number, $tt, $str);
                }}

                // Push one token and go the next word
                macro_rules! push1 { ($tt:ident, $str:expr) => {
                    { root.push(line.number, $tt, $str); continue; }
                }}

                if word.ends_with('.') {
                    push1!(MacroCall, word.get(..word.len()-1).unwrap().to_string());
                }

                match c {
                    '&' => {
                        let mut value = mt!();
                        let mut lit = true;
                        for word_char in word.get(1..).unwrap().chars() {
                            if true {//TODO is_hex()
                                value.push(word_char);
                            }
                            if word_char == ':' {
                                lit = false;
                                break;
                            }
                        }
                        if lit { push1!(LitHex, value); }
                    }
                    '#' => directive = true,
                    '%' => {
                        let mut value = mt!();
                        for word_char in word.get(1..).unwrap().chars() {
                            // Remove underscores 1011_1010
                            if word_char == '0' || word_char == '1' {
                                value.push(word_char);
                            }
                        }
                        push1!(LitBin, value);
                    }
                    '+' => push1!(Plus, mt!()),
                    '-' => push1!(Minus, mt!()),
                    '(' => push1!(At0, mt!()),
                    ')' => push1!(At1, mt!()),
                    '"' => push1!(LitStr, word.get(1..word.len()-1).unwrap().to_string()),
                    '.' => push1!(MacroArgument, word.get(1..).unwrap().to_string()),
                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => push1!(LitDec, word.to_string()),
                    _ => { }
                }

                // Prefixed with '#'
                if directive {
                    let d = word.get(1..).unwrap();
                    match d {
                        "def" => push1!(Define, String::new()),
                        "include" => push1!(Include, String::new()),
                        "macro" => push1!(Macro, String::new()),
                        _ => eprintln!("l{}: unknown directive '{}'", i, d),
                    }
                }

                if word.contains(':') {
                    push1!(Marker, word.to_string());
                }

                // Create tokens from names
                macro_rules! str_to_token { ($($str:literal=$ty:ident)*) => {
                    let ty = match word { $($str => $ty,)* _ => Unknown };
                    if ty != Unknown { push1!(ty, String::new()); }
                }}

                str_to_token! {
                    "adc"=Adc "add"=Add "and"=And "bit"=Bit "call"=Call "ccf"=Ccf
                    "cp"=Cp "cpl"=Cpl "daa"=Daa "dec"=Dec "di"=Di "ei"=Ei
                    "halt"=Halt "inc"=Inc "jp"=Jp "jr"=Jr "ld"=Ld "ldi"=Ldi
                    "ldd"=Ldd "nop"=Nop "or"=Or "pop"=Pop "push"=Push "res"=Res
                    "ret"=Ret "rl"=Rl "rla"=Rla "rlc"=Rlc "rld"=Rld "rr"=Rr
                    "rra"=Rra "rrc"=Rrc "rrca"=Rrca "rrd"=Rrd "rst"=Rst "sbc"=Sbc
                    "scf"=Scf "set"=Set "sla"=Sla "sll"=Sll "sra"=Sra "srl"=Srl
                    "stop"=Stop "sub"=Sub "swap"=Swap "xor"=Xor "reti"=Reti "rlca"=Rlca
                    "a"=A "b"=B "c"=C "d"=D "e"=E "h"=H "l"=L "af"=Af "bc"=Bc
                    "de"=De "hl"=Hl "sp"=Sp "hix"=Hix "hiy"=Hiy "lix"=Lix "liy"=Liy
                    "Z"=FlagZ "NZ"=FlagNz "C"=FlagC "NC"=FlagNc
                };

                // Identifiers. they cannot start with a number
                if utils::is_ident_first(&c) {
                    let mut ident = true;
                    let mut macro_ident = false;

                    for c in word.get(1..).unwrap().chars() {
                        // The other characters a-zA-Z0-9_
                        if !utils::is_ident_char(&c) {
                            ident = false;
                            if c == '.' {
                                macro_ident = true;
                                //TODO break out of for, idk if break here would work.
                            }else {
                                macro_ident = false; 
                                //TODO
                            }
                        }
                    }
                    if macro_ident { push1!(MacroIdentifier, word.to_string()); }
                    else if ident { push1!(Identifier, word.to_string()); }
                }

                // The word did not match any token type
                pushn!(Unknown, word.to_string());
            }
        }
        
        root
    }

    /// Hierarchize tokens
    /// unsafe: pointer deref
    unsafe fn make_tree(self) -> IntermediateAST {
        let mut ast = Token::root();
        let mut macro_defs= vec![];

        let mut line = 0;

        let mut selected: *mut _ = &mut ast;

        for token in self.children {
            // New line, end instructions and directives.
            if line != token.line {
                line = token.line;
                
                loop {
                    match (*selected).ty {
                        Instruction|Argument|Plus|Minus|MacroCall => {
                            selected = (*selected).parent;
                        }
                        Directive => {
                            if (*selected).children[0].ty == Macro {
                                selected = (*selected).push(line, MacroBody, mt!());
                            }else {
                                selected = (*selected).parent;
                            }
                        }
                        _ => break,
                    }
                }
            }

            if ((*selected).ty==Plus || (*selected).ty==Minus) && (*selected).children.len() >= 2 {
                // Close binary operation.
                selected = (*selected).parent;
            }

            if (*selected).ty == Argument && token.ty != Plus && token.ty != Minus {
                // Close the argument.
                selected = (*selected).parent;
            }

            if (*selected).ty == Instruction || (*selected).ty == MacroCall {
                // Create a new argument.
                selected = (*selected).push(line, Argument, mt!());
            }

            match token.ty {
                Define|Include => {
                    selected = (*selected).push(line, Directive, mt!());
                    (*selected).push(line, token.ty, mt!());
                }

                Macro => {
                    if (*selected).ty == MacroBody {
                        selected = &mut ast as *mut _;
                    }else {
                        let mut macro_def = Token::root();
                        macro_def.line = line;
                        macro_def.ty = Directive;
                        macro_def.push(line, Macro, mt!());

                        macro_defs.push(macro_def);
                        selected = macro_defs.iter_mut().last().unwrap() as *mut _;
                    }
                }

                MacroIdentifier => {
                    let split = token.value.split('.').collect::<Vec<_>>();
                    let mut iter = split.iter();
                    (*selected).push(line, MacroIdentifier, iter.next().unwrap().to_string());

                    for s in iter {
                        (*selected).push(line, MacroArgument, s.to_string());
                    }
                }

                MacroCall => {
                    selected = (*selected).push(line, MacroCall, mt!());

                    // Separate the repeat count and the identifier.
                    for (mcc_i, macro_call_c) in token.value.chars().enumerate() {
                        if !utils::is_num(&macro_call_c) {
                            if mcc_i == 0 {
                                // No repeat count, only an identifier.
                                (*selected).push(line, Identifier, token.value.clone());
                            }else {
                                // Push LIT, but after the IDENTIFIER so it can be
                                // at the same index even when there is no repeat count.
                                let ident = token.value.get(mcc_i..).unwrap().to_string();
                                (*selected).push(line, Identifier, ident);

                                let repeat = token.value.get(..mcc_i).unwrap().to_string();
                                let lit = (*selected).push(line, Lit, mt!());
                                (*lit).push(line, LitDec, repeat);
                            }

                            break;
                        }
                    }
                }

                Adc|Add|And|Bit|Call|Ccf|Cp|Cpl|Daa|Dec|Di|Ei|Halt|Inc|Jp|Jr|Ld|Ldi|Ldd|Nop|
                Or|Pop|Push|Res|Ret|Rl|Rla|Rlc|Rld|Rr|Rra|Rrc|Rrca|Rrd|Rst|Sbc|Scf|Set|Sla|Sll|
                Sra|Srl|Stop|Sub|Swap|Xor|Reti|Rlca => {
                    selected = (*selected).push(line, Instruction, mt!());
                    let instr_name = (*selected).push(line, InstructionName, mt!());
                    (*instr_name).push(line, token.ty, mt!());
                }

                A|B|C|D|E|H|L|Af|Bc|De|Hl|Sp|Hix|Hiy|Lix|Liy => {
                    let reg = (*selected).push(line, Register, mt!());
                    (*reg).push(line, token.ty, mt!());
                }

                FlagZ|FlagNz|FlagC|FlagNc => {
                    let flag = (*selected).push(line, Flag, mt!());
                    (*flag).push(line, token.ty, mt!());
                }

                LitDec|LitBin|LitHex|LitStr => {
                    let lit = (*selected).push(line, Lit, mt!());
                    (*lit).push(line, token.ty, token.value);
                }

                Plus|Minus => {
                    let prev = (*selected).children.pop().unwrap();
                    selected = (*selected).push(line, token.ty, mt!());
                    (*selected).transfer(prev);
                }

                Marker => {
                    let marker = (*selected).push(line, Marker, mt!());

                    for word in token.value.split(':') {
                        if word.is_empty() { continue; }
                        if word.starts_with('&') {
                            let lit = (*marker).push(line, Lit, mt!());
                            (*lit).push(line, LitHex, word.get(1..).unwrap().to_string());
                        }else {
                            // TODO fn
                            if utils::is_ident_first(&word.chars().next().unwrap()) {
                                let mut ident = true;
                                for ch in word.get(1..).unwrap().chars() {
                                    if !utils::is_ident_char(&ch) {
                                        ident = false;
                                        break;
                                    }
                                }
                                if ident {
                                    (*marker).push(line, Identifier, word.to_string());
                                }
                            }else {
                                (*marker).push(line, Unknown, mt!());
                            }
                        }
                    }
                }

                At0 => selected = (*selected).push(line, At, mt!()),
                At1 => selected = (*selected).parent,

                _ => {
                    (*selected).transfer(token);
                }
            }
        }

        IntermediateAST { root: ast, macro_defs }
    }

    #[cfg(feature = "debug")]
    pub fn debug(&self) {
        fn children(token: &Token, indent: usize) {
            for child in &token.children {
                let mut n = child.line.to_string();
                if n.len() < 6 { n.push_str(&" ".repeat(7-n.len())); }
                let sep = "|   ".repeat(indent);
                // https://en.wikipedia.org/wiki/ANSI_escape_code
                if child.ty == Unknown { 
                    // yellow Unknown: value
                    println!("    L{}{}|\x1b[0;33m{:?}: {}\x1b[0m", n, sep, child.ty, child.value);
                }else {
                    // red value
                    let value = format!("\x1b[0;31m{}\x1b[0m", child.value);
                    println!("    L{}{}|{:?}: {}", n, sep, child.ty, value);
                }
                children(child, indent+1);
            }
        }

        children(self, 0);
        println!();
    }
}

/// Token tree before the macro are expanded.
pub struct IntermediateAST {
    pub root: Token,
    pub macro_defs: Vec<Token>,
}

impl IntermediateAST {
    /// Expand macro calls. resulting tokens go in the existing 
    /// MacroCall tokens, which get their previous children removed.
    /// unsafe: pointer deref
    // TODO fix line numbers in expanded calls.
    // TODO do not put macro_defs in the ast.
    pub fn expand(mut self) -> Self {
        fn walk(current: &mut Token, macro_defs: &[Token]) {
            for current_child in &mut current.children {
                if current_child.ty != MacroCall {
                    walk(current_child, macro_defs);
                    continue;
                }

                let macro_call = current_child;
                #[cfg(feature = "debug")] {
                    println!("    Expanding macro call '{}':", &macro_call.children[0].value);
                }

                // Get macro identifier.
                let ident = &macro_call.children[0].value;
                let mut def = None;

                // Look for the corresponding macro declaration.
                for macro_def in macro_defs {
                    if &macro_def.children[1].value == ident {
                       def = Some(macro_def);
                       break;
                    }
                }

                let mut arg_values = vec![];
                let mut repeat: Option<usize> = None;

                for arg in &macro_call.children {
                    if arg.ty == Argument {
                        arg_values.push(&arg.children[0]);
                    }else if arg.ty == Lit {
                        let dec = &arg.children[0];

                        if dec.ty == LitDec {
                            repeat = Some(utils::parse_dec(&dec.value));
                        }else {
                            eprintln!(  "Error in macro call line {}, \
                                        only decimal repeat counts are supported. \
                                        (type: {:?})",
                                        arg.line,
                                        arg.ty);
                        }
                    }
                }

                if let Some(def) = def {
                    // Macro declaration found, expansion can continue.
                    let mut arg_names = vec![];
                    let mut macro_body = None;

                    for c in &def.children {
                        if c.ty == MacroArgument {
                            arg_names.push(&c.value);
                        }else if c.ty == MacroBody{
                            macro_body = Some(c);
                        }
                    }

                    if arg_names.len() != arg_values.len() {
                        eprintln!(  "Macro call arguments count at line {} does not \
                                    match the count in macro declaration. \
                                    ({} != {})",
                                    macro_call.line,
                                    arg_values.len(),
                                    arg_names.len());
                    }

                    if let Some(body) = macro_body {
                        let mut b = body.clone();

                        // Replace arguments with the values.
                        fn replace_args(token: &mut Token, names: &[&String], args: &[&Token]) {
                            for child in &mut token.children {
                                if child.ty == MacroArgument {
                                    let mut replaced = false;

                                    for (i, name) in names.iter().enumerate() {
                                        if &&child.value == name {
                                            child.copy(args[i]);
                                            replaced = true;
                                            break;
                                        }
                                    }

                                    if !replaced {
                                        eprintln!(  "Unrecognized argument {} in macro call line {}",
                                                    &child.value,
                                                    child.line);
                                    }
                                }else {
                                    replace_args(child, names, args);
                                }
                            }
                        }

                        replace_args(&mut b, &arg_names[..], &arg_values[..]);

                        macro_call.children = vec![];

                        let repeat = if let Some(repeat) = repeat { repeat }else { 1 };
                        for _ in 0..repeat {
                            for b_child in &b.children {
                                macro_call.transfer(b_child.clone());        
                            }
                        }

                        #[cfg(feature = "debug")] macro_call.debug();
                    }else {
                        eprintln!("Macro declaration at line {} does not have a body", def.line);
                    }
                }else { eprintln!("Macro declaration not found."); }
            }
        }

        walk(&mut self.root, &self.macro_defs);

        self
    }

    pub fn get_markers(mut self) -> Self {
        for child in &self.root.children {

        }

        self
    }

    pub fn build(mut self) -> Token {
        //TODO
        self.root
    }
}

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    Instruction,
        InstructionName,
            Adc,    Add,    And,    Bit,    Call,
            Ccf,    Cp,     Cpl,    Daa,    Dec,
            Di,     Ei,     Halt,   Inc,    Jp,
            Jr,     Ld,     Ldi,    Ldd,    Nop,
            Or,     Pop,    Push,   Res,    Ret,
            Rl,     Rla,    Rlc,    Rld,    Rr,
            Rra,    Rrc,    Rrca,   Rrd,    Rst,
            Sbc,    Scf,    Set,    Sla,    Sll,
            Sra,    Srl,    Stop,   Sub,    Swap,
            Xor,    Reti,   Rlca,

        Argument,
            Register,
                A, B, C, D, E, H, L, Af, Bc, De, Hl,
                Sp, Hix, Hiy, Lix, Liy,

            Lit,
                LitBin, LitHex, LitDec, LitStr,

            Plus, Minus,
                
            At,
                At0, At1,

            Flag,
                FlagZ, FlagNz, FlagC, FlagNc,

            // opcodes.rs only, they are then converted to literals
            B0, B1, B2, B3, B4, B5, B6, B7,

    Directive,
        Define,
            Identifier,
        Include,
        Macro,
            MacroIdentifier, MacroArgument, MacroBody,
        
    MacroCall,
    Marker,

    Unknown,
}
