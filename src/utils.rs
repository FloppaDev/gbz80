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
        use TokenType::{self, *};

        pub struct Instruction {
            ty: TokenType,
            ops: Vec<Op>,
        }

        pub struct Op {
            args: Vec<OpToken>,
            bytes: u16,
            //TODO rename var_size in Token
            input: usize
        }

        pub struct OpToken {
            ty: TokenType,
            children: Vec<OpToken>,
        }

        /// Get instructions from the opcodes file.
        /// In the file, addresses and addition are written in a way that
        /// is convenient for macro_rules, but not convenient for actual use.
        /// Thus, we will put these tokens in containers, pretty much like with
        /// the input asm code.
        fn get_instructions() -> Vec<Instruction> {
            let mut instructions = vec![];    
            let mut at = None;
            let mut plus = None;

            $(//:ADC
                let mut instr = Instruction { ty: $instr, ops: vec![] };
                $(//0x8E 0
                    let mut op = Op { args: vec![], bytes: $opc, input: $in };

                    $(//A AT0 HL AT1
                        // Moving logic out of $()*
                        // Otherwise this monstrosity would be generated 500~ times.
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
