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
        let root = unsafe { Token::make_tree(base_tokens) };

        root
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
                    '"' => push1!(LIT_STR, String::new()),
                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => push1!(LIT_DEC, word.to_string()),
                    _ => { }
                }

                // Prefixed with '#'
                if directive {
                    let d = word.get(1..).unwrap();
                    match d {
                        "def" => push1!(DIRECTIVE_DEFINE, String::new()),
                        "include" => push1!(DIRECTIVE_INCLUDE, String::new()),
                        "macro" => push1!(DIRECTIVE_MACRO, String::new()),
                        _ => eprintln!("L{}: Unknown directive '{}'", i, d),
                    }
                }

                if word.contains(':') {
                    push1!(MARKER, word.to_string());
                }

                if word.chars().last().unwrap() == '.' {
                    push1!(MACRO_IDENTIFIER, String::new()); 
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
                    let mut is_macro = false;
                    for (j, c) in word.get(1..).unwrap().chars().enumerate() {
                        // The other characters a-zA-Z0-9_
                        if !utils::is_ident_char(&c) {
                            ident = false;
                            // If character is a point, it is a macro declaration,
                            // possibly with multiple parameters.
                            let macro_decl = word.split('.').collect::<Vec<_>>();
                            if macro_decl.len() != 0 {
                                pushn!(MACRO_IDENTIFIER, String::new());
                            }
                            for split in &macro_decl[1..] {
                                pushn!(MACRO_PARAMETER, String::new());
                            }
                            is_macro = true;
                            break;
                        }
                    }

                    if ident { push1!(IDENTIFIER, String::new()); }
                    if is_macro { continue; }
                }

                // The word did not match any token type
                pushn!(UNKNOWN, String::new());
            }
        }
        
        root
    }

    /// Hierarchize tokens
    /// unsafe: pointer deref
    unsafe fn make_tree(self) -> Token {
        let mut ast = Token::root();
        let mut line = 0;

        let mut selected: *mut _ = &mut ast;
        let mut paren = 0 as *mut Token;

        for token in self.children {
            // New line, end instructions and directives.
            if line != token.line {
                line = token.line;
                
                loop {
                    match (*selected).ty {
                        DIRECTIVE|INSTRUCTION|ARGUMENT => selected = (*selected).parent,
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

            if (*selected).ty == INSTRUCTION {
                // Create a new argument.
                selected = (*selected).push(line, ARGUMENT, mt!());
            }

            // TODO macro calls can have registers, literals, addresses, or bits,
            // just like instructions.
            match token.ty {
                DIRECTIVE_DEFINE => {
                    selected = (*selected).push(line, DIRECTIVE, mt!());
                    (*selected).push(line, DIRECTIVE_DEFINE, mt!());
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

        ast
    }

    #[cfg(debug)]
    pub fn debug(&self) {
        println!("AST data:");

        fn children(token: &Token, indent: usize) {
            for child in &token.children {
                let mut n = child.line.to_string();
                if n.len() < 6 { n.push_str(&" ".repeat(7-n.len())); }
                let sep = "|   ".repeat(indent);
                // Color value in red
                let value = format!("\x1b[0;31m{}\x1b[0m", child.value);
                println!("    L{}{}|{:?}: {}", n, sep, child.ty, value);
                children(child, indent+1);
            }
        }

        children(self, 0);
        println!();
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

    META,
        DIRECTIVE,
            DIRECTIVE_DEFINE,
                IDENTIFIER,
            DIRECTIVE_INCLUDE,
            DIRECTIVE_MACRO,
            
        MACRO_IDENTIFIER, MACRO_DEFINITION, MACRO_PARAMETER,
        MARKER,

        UNKNOWN,
}
