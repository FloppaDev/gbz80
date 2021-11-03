/// Empty string
macro_rules! mt { () => { String::new() } }

/// Get opcodes from a macro call in opcodes.rs
macro_rules! opcodes {(
    $(
        :$instr:ident $(
            $opc:literal $in:literal $($arg:ident)*
        )*
    )*
) => {
        use ast::TokenType::{self, *};

        #[derive(Debug)]
        pub struct Instruction {
            ty: TokenType,
            ops: Vec<Op>,
        }

        #[derive(Debug)]
        pub struct Op {
            args: Vec<OpToken>,
            bytes: u16,
            //TODO rename var_size in Token
            input: usize
        }

        #[derive(Debug)]
        pub struct OpToken {
            ty: TokenType,
            children: Vec<OpToken>,
        }

        /// Get instructions from the opcodes file.
        /// In the file, addresses and addition are written in a way that
        /// is convenient for macro_rules, but not convenient for actual use.
        /// Thus, we will put these tokens in containers, pretty much like with
        /// the input asm code.
        pub fn get_instructions() -> Vec<Instruction> {
            let mut instructions = vec![];    
            let mut at = None;
            let mut plus = None;

            $(//:ADC
                let mut instr = Instruction { ty: $instr, ops: vec![] };
                $(//0x8E 0
                    let mut op = Op { args: vec![], bytes: $opc, input: $in };

                    $(//A AT0 HL AT1
                        // Moving logic out of $()*
                        // Otherwise this would be generated thousands of times.
                        op_tokens(&mut op, $arg, &mut at, &mut plus); 
                    )*

                    instr.ops.push(op);
                )*
                instructions.push(instr);
            )*

            instructions
        }

        fn op_tokens(
            op: &mut Op,
            ty: TokenType,
            at: &mut Option<OpToken>,
            plus: &mut Option<OpToken>,
        ) {
            match ty {
                AT0 => *at = Some(OpToken { ty: AT, children: vec![] }),
                AT1 => op.args.push(at.take().unwrap()),
                PLUS => *plus = Some(OpToken{ ty: PLUS, children: vec![] }),
                _ => {
                    // Push normally if not part of an adress ar addition
                    let mut push = true;

                    // Are we between AT0 and AT1 (an address) 
                    if let Some(at) = at.as_mut() {
                        // If last token was a PLUS, we take the last pushed
                        // and the current, and put them in a PLUS.
                        if let Some(mut plus) = plus.take() {
                            plus.children.push(at.children.pop().unwrap());
                            plus.children.push(
                                OpToken { ty, children: vec![] }
                            );
                            at.children.push(plus);
                        }else {
                            // No PLUS, this is just an address.
                            at.children.push(
                                OpToken { ty, children: vec![] }
                            );
                        }
                        push = false;
                    }
                    // Last token was a PLUS.
                    // Last pushed token is Lhs, current is Rhs.
                    if let Some(mut plus) = plus.take() {
                        plus.children.push(op.args.pop().unwrap());
                        plus.children.push(
                            OpToken { ty, children: vec![] }
                        );
                        op.args.push(plus);
                        push = false;
                    }

                    // No PLUS, No address, just push the token.
                    if push {
                        op.args.push(OpToken { ty, children: vec![] });
                    }
                }
            }
        }
    }
}

pub fn is_new_line(c: &char) -> bool {
    const CHARS: [char; 2] = ['\u{000A}', '\u{000D}'];
    CHARS.contains(c)
}

pub fn is_space(c: &char) -> bool {
    const CHARS: [char; 29] = [
        '\u{0009}', '\u{000B}', '\u{000C}', '\u{0020}', '\u{0085}',
        '\u{00A0}', '\u{1680}', '\u{180E}', '\u{2000}', '\u{2001}', 
        '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}',
        '\u{2007}', '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
        '\u{200C}', '\u{200D}', '\u{2028}', '\u{2029}', '\u{202F}',
        '\u{205F}', '\u{2060}', '\u{3000}', '\u{FEFF}',
    ];
    CHARS.contains(c)
}

pub fn is_ident_char(c: &char) -> bool {
    is_num(c) || is_ident_first(c)
}

pub fn is_num(c: &char) -> bool {
    const CHARS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    CHARS.contains(c)
}

pub fn is_ident_first(c: &char) -> bool {
    const CHARS: [char; 53] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '_',
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    CHARS.contains(c)
}

#[cfg(debug)]
pub fn debug_title(title: &str) {
    let decoration = "=".repeat(79);
    println!("{}\n\t\t\t\t{}\n{}", decoration, title, decoration);
}
