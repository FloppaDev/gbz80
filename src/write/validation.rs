use crate::ast::{TokenType::*, Token};
use crate::opcodes::InstructionDef;

//TODO cleanup
//TODO forbid nested macro calls.
pub fn check(root: &Token) -> usize {

    fn check_children(root: &Token, mut ec: &mut usize) {
        for token in &root.children {
            match token.ty {
                Plus|Minus|At|Flag|Register => {
                    let e = format!(
                        "Token '{:?}' not expected at root. (L{})",
                        token.ty,
                        token.line
                    );
                    err(&e, &mut ec);
                }

                Instruction => {
                    if token.children.len() > 3 {
                        let e = format!(
                            "Too many arguments in instruction, expected [0; 2], got {}. (L{})",
                            token.children.len() - 1,
                            token.line
                        );
                        err(&e, &mut ec);
                    }else {
                        for arg in &token.children[1..] {
                            match arg.children[0].ty {
                                Register|Lit|Flag|Identifier => {}

                                At => {
                                    let at = &arg.children[0];
                                    let child_count = at.children.len();
                                    match child_count {
                                        0 => {
                                            let e = format!(
                                                "Adress is empty. (L{})",
                                                at.line
                                            );
                                            err(&e, &mut ec);
                                        }
                                        1 => {
                                            match at.children[0].ty {
                                                Register|Lit|Identifier => {}
                                                Plus => {
                                                    let plus = &at.children[0];
                                                    let opd_len = plus.children.len();
                                                    if opd_len != 2 {
                                                        let e = format!(
                                                            "Expected 2 operands in '+' expression, got {}. (L{})",
                                                            opd_len,
                                                            plus.line
                                                        );
                                                        err(&e, &mut ec);
                                                    }

                                                    for opd in &plus.children {
                                                        match opd.ty {
                                                            Register|Lit|Identifier => {}
                                                            _ => {
                                                                let e = format!(
                                                                    "Token '{:?}' not expected in adress. (L{})",
                                                                    opd.ty,
                                                                    opd.line
                                                                );
                                                                err(&e, &mut ec);
                                                            }
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    let e = format!(
                                                        "Token '{:?}' not expected in adress. (L{})",
                                                        at.children[0].ty,
                                                        at.line
                                                    );
                                                    err(&e, &mut ec);
                                                }
                                            }
                                        }
                                        _ => {
                                            let e = format!(
                                                "Too many arguments in adress, expected 1, got {}. (L{})",
                                                child_count,
                                                at.line
                                            );
                                            err(&e, &mut ec);
                                        }
                                    }
                                }

                                _ => {
                                    let e = format!(
                                        "Unexpected token '{:?}' as intruction argument. (L{})",
                                        arg.children[0].ty,
                                        token.line
                                    );
                                    err(&e, &mut ec);
                                }
                            }
                        }
                    }
                }

                MacroCall => {
                    check_children(token, &mut ec);
                }

                Unknown => {
                    let e = format!(
                        "Unknown token type. (L{})",
                        token.line
                    );
                    err(&e, &mut ec);
                }

                _ => {}
            }
        }
    }

    check_children(root, &mut ec);

    ec
}