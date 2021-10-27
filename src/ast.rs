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
                let mut lit_hex = false;

                // Push a one or more tokens for this word
                macro_rules! pushn { ($tt:ident, $str:expr) => {
                    root.push(line.number, $tt, $str);
                }}

                // Push one token and go the next word
                macro_rules! push1 { ($tt:ident, $str:expr) => {
                    { root.push(line.number, $tt, $str); continue; }
                }}
                
                // Tokens that can at least partially be identified by
                // their first character
                match c {
                    '&' => lit_hex = true,
                    '#' => directive = true,
                    '%' => push1!(LIT_BIN, String::new()), //TODO 
                    ':' => push1!(LABEL, String::new()),
                    '+' => push1!(PLUS, String::new()),
                    '-' => push1!(MINUS, mt!()),
                    '(' => push1!(AT0, mt!()),
                    ')' => push1!(AT1, mt!()),
                    '"' => push1!(LIT_STR, String::new()),
                    '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'0' => push1!(LIT_DEC, String::new()),
                    _ => { }
                }

                // TODO same for LIT_DEC
                // Fence names are optional
                if lit_hex {
                    if word.chars().nth(word.len()-1).unwrap() == ':' {
                        push1!(ANONYMOUS_FENCE, String::new()); 
                    }
                    pushn!(LIT_HEX, String::new());
                    for (j, c) in word[1..word.len()-1].chars().enumerate() {
                        if c == ':' {
                            pushn!(FENCE, String::new());
                            break;
                        }
                    }
                    continue;
                }

                // Prefixed with '#'
                if directive {
                    match word.get(1..).unwrap() {
                        "def" => push1!(DIRECTIVE_DEFINE, String::new()),
                        "include" => push1!(DIRECTIVE_INCLUDE, String::new()),
                        "macro" => push1!(DIRECTIVE_MACRO, String::new()),
                        _ => eprintln!("l{}: Unknown directive", i),
                    }
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
        // Assigned to when next token could be an expression
        let mut lhs: *mut _ = &mut ast;
        let mut paren = 0 as *mut Token;

        for token in self.children {
            // New line
            if line != token.line {
                line = token.line;
                
                //TODO needs to be recursive
                match (*selected).ty {
                    DIRECTIVE_DEFINE|DIRECTIVE_INCLUDE|
                    INSTRUCTION => {
                        // End on a new line 
                    }
                    _ => {}
                }
            }

            match token.ty {
                DIRECTIVE_DEFINE => {
                    selected = (*selected).push(token.line, DIRECTIVE, mt!());
                }
                ADC|ADD|AND|BIT|CALL|CCF|CP|CPL|DAA|DEC|DI|EI|HALT|INC|JP|JR|LD|LDI|LDD|NOP|
                OR|POP|PUSH|RES|RET|RL|RLA|RLC|RLD|RR|RRA|RRC|RRCA|RRD|RST|SBC|SCF|SET|SLA|SLL|
                SRA|SRL|STOP|SUB|SWAP|XOR|RETI|RLCA => {
                    selected = (*selected).push(token.line, INSTRUCTION, mt!());
                    let instr_name = (*selected).push(token.line, INSTRUCTION_NAME, mt!());
                    (*instr_name).push(token.line, token.ty, mt!());
                }
                A|B|C|D|E|H|L|AF|BC|DE|HL|SP|HIX|HIY|LIX|LIY => {
                    //TODO push ARGUMENT on AT1
                    if (*selected).ty == INSTRUCTION {
                        let arg = (*selected).push(token.line, ARGUMENT, mt!());
                        let reg = (*arg).push(token.line, REGISTER, mt!());
                        (*reg).push(token.line, token.ty, mt!());
                    }else {
                        lhs = (*selected).push(token.line, REGISTER, mt!());
                        (*lhs).push(token.line, token.ty, mt!());
                    }
                }
                FLAG_Z|FLAG_NZ|FLAG_C|FLAG_NC => {
                    let arg = (*selected).push(token.line, ARGUMENT, mt!());
                    let flag = (*arg).push(token.line, FLAG, mt!());
                    (*flag).push(token.line, token.ty, mt!());
                }
                LIT_DEC => {
                    //TODO transform into BX if arg1 in instruction
                }
                LIT_BIN|LIT_HEX|LIT_STR => {

                }
                PLUS|MINUS => {
                    //TODO
                }
                AT0 => {

                }
                AT1 => {

                }
                _ => {
                    (*selected).transfer(token);
                }
            }
        }

        ast
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

            B0, B1, B2, B3, B4, B5, B6, B7,

    META,
        DIRECTIVE,
            DIRECTIVE_DEFINE,
                IDENTIFIER,
            DIRECTIVE_INCLUDE,
            DIRECTIVE_MACRO,
            
        MACRO_IDENTIFIER, MACRO_DEFINITION, MACRO_PARAMETER,
        FENCE, ANONYMOUS_FENCE, LABEL,

        UNKNOWN,
}
