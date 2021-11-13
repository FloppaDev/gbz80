use utils;
use split::Split;
use self::TokenType::*;

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
    fn root() -> Self {
        Self {
            line: 0,
            ty: TokenType::UNKNOWN,
            parent: 0 as *mut Token,
            children: vec![],
            value: String::new(),
        }
    }

    /// Add a new token as a child of self.
    fn push(&mut self, line: usize, ty: TokenType, value: String) -> &mut Token {
        let parent: *mut _ = self;
        let children = vec![];
        self.children.push(Token {line, ty, parent, children, value});
        let i = self.children.len() - 1;
        &mut self.children[i]
    }

    /// Move token to self as child
    fn transfer(&mut self, token: Token) {
        self.children.push(token);
    }

    /// Build the AST from split data
    pub fn make_ast(split: Split) -> Token {
        let base_tokens = Token::get_base_tokens(&split); 
        unsafe { 
            let int_ast = Token::make_tree(base_tokens);
            #[cfg(debug)] {
                utils::debug_title("AST data");
                int_ast.ast.debug();
                utils::debug_title("Macros expansion");
            }
            let ast = int_ast.expand();
            
            return ast;
        }
    }

    /// Create a root token and adds all the identified tokens as children
    fn get_base_tokens(split: &Split) -> Token {
        // Root of the token tree, for now it will contain all tokens as direct children.
        let mut root = Token::root();

        for (i, line) in split.lines.iter().enumerate() {

            for word in &line.words {
                let word = &split.input[word.start..word.end];
                let c = word.chars().nth(0).unwrap();

                let mut directive = false;

                // Push a one or more tokens for this word
                macro_rules! pushn { ($tt:ident, $str:expr) => {
                    root.push(line.number, $tt, $str);
                }}

                // Push one token and go the next word
                macro_rules! push1 { ($tt:ident, $str:expr) => {
                    { root.push(line.number, $tt, $str); continue; }
                }}

                if word.chars().last().unwrap() == '.' {
                    push1!(MACRO_CALL, word.get(..word.len()-1).unwrap().to_string());
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
                        if lit { push1!(LIT_HEX, value); }
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
                        push1!(LIT_BIN, value);
                    }
                    '+' => push1!(PLUS, mt!()),
                    '-' => push1!(MINUS, mt!()),
                    '(' => push1!(AT0, mt!()),
                    ')' => push1!(AT1, mt!()),
                    '"' => push1!(LIT_STR, word.get(1..word.len()-1).unwrap().to_string()),
                    '.' => push1!(MACRO_ARGUMENT, word.get(1..).unwrap().to_string()),
                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => push1!(LIT_DEC, word.to_string()),
                    _ => { }
                }

                // Prefixed with '#'
                if directive {
                    let d = word.get(1..).unwrap();
                    match d {
                        "def" => push1!(DEFINE, String::new()),
                        "include" => push1!(INCLUDE, String::new()),
                        "macro" => push1!(MACRO, String::new()),
                        _ => eprintln!("L{}: Unknown directive '{}'", i, d),
                    }
                }

                if word.contains(':') {
                    push1!(MARKER, word.to_string());
                }

                // Create tokens from names
                macro_rules! str_to_token { ($($str:literal=$ty:ident)*) => {
                    let ty = match word { $($str => $ty,)* _ => UNKNOWN };
                    if ty != UNKNOWN { push1!(ty, String::new()); }
                }}

                str_to_token! {
                    "adc"=ADC "add"=ADD "and"=AND "bit"=BIT "call"=CALL "ccf"=CCF
                    "cp"=CP "cpl"=CPL "daa"=DAA "dec"=DEC "di"=DI "ei"=EI
                    "halt"=HALT "inc"=INC "jp"=JP "jr"=JR "ld"=LD "ldi"=LDI
                    "ldd"=LDD "nop"=NOP "or"=OR "pop"=POP "push"=PUSH "res"=RES
                    "ret"=RET "rl"=RL "rla"=RLA "rlc"=RLC "rld"=RLD "rr"=RR
                    "rra"=RRA "rrc"=RRC "rrca"=RRCA "rrd"=RRD "rst"=RST "sbc"=SBC
                    "scf"=SCF "set"=SET "sla"=SLA "sll"=SLL "sra"=SRA "srl"=SRL
                    "stop"=STOP "sub"=SUB "swap"=SWAP "xor"=XOR "reti"=RETI "rlca"=RLCA
                    "a"=A "b"=B "c"=C "d"=D "e"=E "h"=H "l"=L "af"=AF "bc"=BC
                    "de"=DE "hl"=HL "sp"=SP "hix"=HIX "hiy"=HIY "lix"=LIX "liy"=LIY
                    "Z"=FLAG_Z "NZ"=FLAG_NZ "C"=FLAG_C "NC"=FLAG_NC
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
                    if macro_ident { push1!(MACRO_IDENTIFIER, word.to_string()); }
                    else if ident { push1!(IDENTIFIER, word.to_string()); }
                }

                // The word did not match any token type
                pushn!(UNKNOWN, word.to_string());
            }
        }
        
        root
    }

    /// Hierarchize tokens
    /// unsafe: pointer deref
    unsafe fn make_tree(self) -> IntermediateAST {
        let mut ast = Token::root();
        let mut macro_defs= vec![];
        let mut macro_calls= vec![];

        let mut line = 0;

        let mut selected: *mut _ = &mut ast;

        for token in self.children {
            // New line, end instructions and directives.
            if line != token.line {
                line = token.line;
                
                loop {
                    match (*selected).ty {
                        INSTRUCTION|ARGUMENT|PLUS|MINUS|MACRO_CALL => {
                            selected = (*selected).parent;
                        }
                        DIRECTIVE => {
                            if (*selected).children[0].ty == MACRO {
                                selected = (*selected).push(line, MACRO_BODY, mt!());
                            }else {
                                selected = (*selected).parent;
                            }
                        }
                        _ => break,
                    }
                }
            }

            if (*selected).ty == PLUS || (*selected).ty == MINUS {
                if (*selected).children.len() >= 2 {
                    // Close binary operation.
                    selected = (*selected).parent;
                }
            }

            if (*selected).ty == ARGUMENT {
                if token.ty != PLUS && token.ty != MINUS {
                    // Close the argument.
                    selected = (*selected).parent;
                }
            }

            if (*selected).ty == INSTRUCTION || (*selected).ty == MACRO_CALL {
                // Create a new argument.
                selected = (*selected).push(line, ARGUMENT, mt!());
            }

            match token.ty {
                DEFINE|INCLUDE => {
                    selected = (*selected).push(line, DIRECTIVE, mt!());
                    (*selected).push(line, token.ty, mt!());
                }

                MACRO => {
                    if (*selected).ty == MACRO_BODY {
                        // <- DIRECTIVE <- MACRO_BODY
                        selected = (*(*selected).parent).parent; 
                    }else {
                         selected = (*selected).push(line, DIRECTIVE, mt!());
                        (*selected).push(line, MACRO, mt!());   
                        macro_defs.push((&(*selected)) as *const _);
                    }
                }

                MACRO_IDENTIFIER => {
                    let split = token.value.split('.').collect::<Vec<_>>();
                    let mut iter = split.iter();
                    (*selected).push(line, MACRO_IDENTIFIER, iter.next().unwrap().to_string());

                    while let Some(s) = iter.next() {
                        (*selected).push(line, MACRO_ARGUMENT, s.to_string());
                    }
                }

                MACRO_CALL => {
                    selected = (*selected).push(line, MACRO_CALL, mt!());
                    macro_calls.push((&(*selected)) as *const _);

                    // Separate the repeat count and the identifier.
                    for (mcc_i, macro_call_c) in token.value.chars().enumerate() {
                        if !utils::is_num(&macro_call_c) {
                            if mcc_i == 0 {
                                // No repeat count, only an identifier.
                                (*selected).push(line, IDENTIFIER, token.value.clone());
                            }else {
                                // Push LIT, but after the IDENTIFIER so it can be
                                // at the same index even when there is no repeat count.
                                let ident = token.value.get(mcc_i..).unwrap().to_string();
                                (*selected).push(line, IDENTIFIER, ident);

                                let repeat = token.value.get(..mcc_i).unwrap().to_string();
                                let lit = (*selected).push(line, LIT, mt!());
                                (*lit).push(line, LIT_DEC, repeat);
                            }

                            break;
                        }
                    }
                }

                ADC|ADD|AND|BIT|CALL|CCF|CP|CPL|DAA|DEC|DI|EI|HALT|INC|JP|JR|LD|LDI|LDD|NOP|
                OR|POP|PUSH|RES|RET|RL|RLA|RLC|RLD|RR|RRA|RRC|RRCA|RRD|RST|SBC|SCF|SET|SLA|SLL|
                SRA|SRL|STOP|SUB|SWAP|XOR|RETI|RLCA => {
                    selected = (*selected).push(line, INSTRUCTION, mt!());
                    let instr_name = (*selected).push(line, INSTRUCTION_NAME, mt!());
                    (*instr_name).push(line, token.ty, mt!());
                }

                A|B|C|D|E|H|L|AF|BC|DE|HL|SP|HIX|HIY|LIX|LIY => {
                    let reg = (*selected).push(line, REGISTER, mt!());
                    (*reg).push(line, token.ty, mt!());
                }

                FLAG_Z|FLAG_NZ|FLAG_C|FLAG_NC => {
                    let flag = (*selected).push(line, FLAG, mt!());
                    (*flag).push(line, token.ty, mt!());
                }

                LIT_DEC|LIT_BIN|LIT_HEX|LIT_STR => {
                    let lit = (*selected).push(line, LIT, mt!());
                    (*lit).push(line, token.ty, token.value);
                }

                PLUS|MINUS => {
                    let prev = (*selected).children.pop().unwrap();
                    selected = (*selected).push(line, token.ty, mt!());
                    (*selected).transfer(prev);
                }

                MARKER => {
                    let marker = (*selected).push(line, MARKER, mt!());

                    for word in token.value.split(':') {
                        if word.len() == 0 { continue; }
                        if word.chars().nth(0).unwrap() == '&' {
                            let lit = (*marker).push(line, LIT, mt!());
                            (*lit).push(line, LIT_HEX, word.get(1..).unwrap().to_string());
                        }else {
                            // TODO fn
                            if utils::is_ident_first(&word.chars().nth(0).unwrap()) {
                                let mut ident = true;
                                for ch in word.get(1..).unwrap().chars() {
                                    if !utils::is_ident_char(&ch) {
                                        ident = false;
                                        break;
                                    }
                                }
                                if ident {
                                    (*marker).push(line, IDENTIFIER, word.to_string());
                                }
                            }else {
                                (*marker).push(line, UNKNOWN, mt!());
                            }
                        }
                    }
                }

                AT0 => selected = (*selected).push(line, AT, mt!()),
                AT1 => selected = (*selected).parent,

                _ => {
                    (*selected).transfer(token);
                }
            }
        }

        IntermediateAST { ast, macro_defs, macro_calls }
    }

    #[cfg(debug)]
    pub fn debug(&self) {
        fn children(token: &Token, indent: usize) {
            for child in &token.children {
                let mut n = child.line.to_string();
                if n.len() < 6 { n.push_str(&" ".repeat(7-n.len())); }
                let sep = "|   ".repeat(indent);
                // https://en.wikipedia.org/wiki/ANSI_escape_code
                if child.ty == UNKNOWN { 
                    // yellow UNKNOWN: value
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
struct IntermediateAST {
    pub ast: Token,
    pub macro_defs: Vec<*const Token>,
    pub macro_calls: Vec<*const Token>,
}

impl IntermediateAST {
    /// Expand macro calls. resulting tokens go in the existing 
    /// MACRO_CALL tokens, which get their previous children removed.
    /// unsafe: pointer deref
    pub unsafe fn expand(self) -> Token {
        for macro_call in &self.macro_calls {
            // Get macro identifier.
            let ident = &(**macro_call).children[0].value;
            let mut def = None;

            // Look for the corresponding macro declaration.
            for macro_def in &self.macro_defs {
                if &(**macro_def).children[1].value == ident {
                   def = Some(*macro_def);
                   break;
                }
            }

            let mut arg_values = vec![];
            for arg in &(**macro_call).children {
                if (*arg).ty == ARGUMENT {
                    arg_values.push(&arg.children[0]);
                }
            }
            
            if let Some(def) = def {
                // Macro declaration found, expansion can continue.
                let mut arg_names = vec![];
                for arg in &(*def).children {
                    if (*arg).ty == MACRO_ARGUMENT {
                        arg_names.push(&(*arg).value);
                    }
                }

                if arg_names.len() != arg_values.len() {
                    eprintln!(  "Macro call arguments count at line {} does not \
                                match the count in macro declaration. \
                                ({} != {})",
                                (**macro_call).line,
                                arg_values.len(),
                                arg_names.len());

                }
            }else { eprintln!("Macro declaration not found."); }
        }

        self.ast
    }
}

/// All the different token types than can be identified.
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    INSTRUCTION,
        INSTRUCTION_NAME,
            ADC,    ADD,    AND,    BIT,    CALL,
            CCF,    CP,     CPL,    DAA,    DEC,
            DI,     EI,     HALT,   INC,    JP,
            JR,     LD,     LDI,    LDD,    NOP,
            OR,     POP,    PUSH,   RES,    RET,
            RL,     RLA,    RLC,    RLD,    RR,
            RRA,    RRC,    RRCA,   RRD,    RST,
            SBC,    SCF,    SET,    SLA,    SLL,
            SRA,    SRL,    STOP,   SUB,    SWAP,
            XOR,    RETI,   RLCA,

        ARGUMENT,
            REGISTER,
                A, B, C, D, E, H, L, AF, BC, DE, HL,
                SP, HIX, HIY, LIX, LIY,

            LIT,
                LIT_BIN, LIT_HEX, LIT_DEC, LIT_STR,

            PLUS, MINUS,
                
            AT,
                AT0, AT1,

            FLAG,
                FLAG_Z, FLAG_NZ, FLAG_C, FLAG_NC,

            // opcodes.rs only, they are then converted to literals
            B0, B1, B2, B3, B4, B5, B6, B7,

    DIRECTIVE,
        DEFINE,
            IDENTIFIER,
        INCLUDE,
        MACRO,
            MACRO_IDENTIFIER, MACRO_ARGUMENT, MACRO_BODY,
        
    MACRO_CALL,
    MARKER,

    UNKNOWN,
}
