
use crate::{
    token::Value,
    parse::{
        text::{self, CheckedStr},
        lex::TokenType::{self, *},
        split::Split,
    },
    error::{ ErrCtx, asm::{AsmErr, ParseMsg::{self, *}} },
};

/// Output of the parser. Contains the type and the key to the data.
pub struct ParsedToken<'a> {
    pub ty: TokenType,
    pub value: Value<'a>,
    pub file: &'a str,
    pub line_number: usize,
    pub line: &'a str,
    pub word: &'a str,
}

/// Map words to token types and extract their data.
pub fn parse<'a>(
    split: &Split<'a>,
) -> Result<Vec<ParsedToken<'a>>, Vec<AsmErr<'a, ParseMsg>>> {
    let mut parsed_tokens = vec![];
    let mut errors = vec![];
    let words_vec = split.words();
    let mut words = words_vec.iter();

    while let Some((word, line, line_number, file)) = words.next() {
        // Attempt to identify the token. 
        let id_words = identify(word);
        let line_number = *line_number;

        if let Err(err_type) = id_words {
            let err_ctx = ErrCtx::new(Root, file, line_number, line, word);
            let err = err!(ParseMsg, err_type, err_ctx);
            errors.push(err);

            continue;
        }

        let mut id_words = id_words.unwrap();

        if matches!(id_words[0].0, DefB|DefW) {
            if let Some((word, line, line_number, file)) = words.next() {
                let line_number = *line_number;
                let mut is_ident = false;
                let mut is_allowed = false;
    
                if let Some(c) = word.get(0..1) {
                    let c = c.chars().next().unwrap();

                    if text::is_char_ident_first(c) {
                        if let Some(ident) = text::check_ident(word) {
                            is_allowed = TokenType::get_by_word(ident.as_str()).is_none();

                            if is_allowed {
                                id_words.push((Identifier, ident));
                                is_ident = true;
                            }
                        }
                    }                
                }

                let err_ctx = ErrCtx::new(Root, file, line_number, line, word);

                if !is_allowed {
                    errors.push(err!(ParseMsg, ReservedKeyword, err_ctx));
                }

                else if !is_ident {
                    errors.push(err!(ParseMsg, BadIdent, err_ctx));
                }
            }
        }

        // Extract data for all words and collect errors.
        for (ty, word_str) in id_words {
            let values = extract((ty, word_str.as_str()));

            if let Err(err_type) = values {
                let err_ctx = ErrCtx::new(Root, file, line_number, line, word_str.as_str());
                let err = err!(ParseMsg, err_type, err_ctx);
                errors.push(err);

                continue;
            }

            let (ty, value) = values.unwrap();
            let parsed_token = ParsedToken { 
                ty,
                value, 
                line_number,
                line,
                file,
                word: word_str.as_str(),
            };

            parsed_tokens.push(parsed_token);
        }
    }

    if errors.is_empty() {
        Ok(parsed_tokens)
    }else {
        Err(errors)
    }
}

/// Extract the data from a word.
fn extract(word: (TokenType, &str)) -> Result<(TokenType, Value), ParseMsg> {
    let (ty, str_value) = word; 

    // There is no value to extract.
    if !ty.has_value() {
        return Ok((ty, Value::Void));
    }

    match ty {
        LitHex => {
            let mut hex = 0;
            let mut mul = 1;

            // Read all characters from right to left.
            for c in str_value.chars().rev() {
                // All characters are valid hex.
                let h = c.to_digit(16).unwrap() as usize; 
                hex += h * mul;
                mul *= 16;
            }

            Ok((ty, fit(hex, str_value.len(), 16)?))
        }

        LitBin => {
            let mut bin = 0;
            let mut mul = 1;

            // Read all characters from right to left.
            for c in str_value.chars().rev() {
                match c {
                    '0' => mul *= 2,

                    '1' => {
                        bin += mul;
                        mul *= 2;
                    }

                    _ => {}
                }
            }

            Ok((ty, fit(bin, str_value.len(), 2)?))
        }

        LitDec|Repeat => {
            let mut dec = 0;
            let mut mul = 1;

            // Read all characters from right to left.
            for c in str_value.chars().rev() {
                // All characters are numbers.
                let d = c.to_digit(10).unwrap() as usize; 
                dec += d * mul;
                mul *= 10;
            }

            Ok((ty, fit(dec, str_value.len(), 10)?))
        }

        // Value for those is str_value
        LitStr|Identifier|Label|NamedMark|AnonMark|MacroArg|MacroIdent => {
            Ok((ty, Value::Str(str_value)))
        }

        _ => bug!("Unhandled type")
    }
}

/// Get token type(s) and value(s) from word.
fn identify(word: &str) -> Result<Vec<(TokenType, CheckedStr)>, ParseMsg> {
    if word.is_empty() {
        return Err(ParseMsg::EmptyStr);
    }

    // Find token type by name.
    // Works with registers and instruction names.
    if let Some(ty) = TokenType::get_by_word(word) {
        return Ok(vec![ (ty, text::no_check(word)) ]);
    }

    let c = word.get(0..1).ok_or(Invalid)?.chars().next().unwrap();
    let last = word.chars().last().unwrap();

    // Find token type by prefix.
    if TokenType::has_prefix(c) {
        match c {
            '&' => {
                // &6762:
                if last == ':' {
                    let lit = word.get(1..word.len() - 1).ok_or(BadAnonMark)?;
                    let hex = text::check_hex(lit).ok_or(BadAnonMarkHex)?;
                    return Ok(vec![ (AnonMark, text::no_check("")), (LitHex, hex) ]);
                }

                // &2763:label
                if let Some(sep) = word.find(':') {
                    let lit = word.get(1..sep).ok_or(BadNamedMark)?;
                    let hex = text::check_hex(lit).ok_or(BadNamedMarkHex)?;

                    let label = word.get(sep + 1 ..).ok_or(BadNamedMarkLabel)?;
                    let ident = text::check_ident(label).ok_or(BadNamedMarkLabelIdent)?;

                    return Ok(vec![ (NamedMark, ident), (LitHex, hex) ]);
                }

                // &2787
                let lit = word.get(1..).ok_or(BadHex)?;
                let hex = text::check_hex(lit).ok_or(BadHex)?;
                return Ok(vec![ (LitHex, hex) ]);
            }

            // 0101_0101 or 11010
            '%' => {
                let lit = word.get(1..).ok_or(BadBin)?;
                let bin = text::check_bin(lit).ok_or(BadBin)?;
                return Ok(vec![ (LitBin, bin) ]);
            }

            // "...
            '"' => {
                let value = word.get(1..).ok_or(BadStr)?;
                return Ok(vec![ (LitStr, text::no_check(value)) ]);
            }

            // db, dw, ds, include, import, or macro
            '#' => {
                let directive = word.get(1..).ok_or(BadDirective)?;

                return match directive {
                    "db" => Ok(vec![ (DefB, text::no_check(directive)) ]),
                    "dw" => Ok(vec![ (DefW, text::no_check(directive)) ]),
                    "include" => Ok(vec![ (Include, text::no_check(directive)) ]),
                    "import" => Ok(vec![ (Import, text::no_check(directive)) ]),
                    "macro" => Ok(vec![ (Macro, text::no_check(directive)) ]),
                    _ => Err(ParseMsg::BadDirectiveIdent)
                };
            }

            // .arg
            '.' => {
                let arg = word.get(1..).ok_or(BadMacroArg)?;
                let ident = text::check_ident(arg).ok_or(BadMacroArgIdent)?;
                return Ok(vec![ (MacroArg, ident) ]);
            }

            ':' => {
                let label = word.get(1..).ok_or(BadLabel)?;
                let ident = text::check_ident(label).ok_or(BadLabelIdent)?;
                return Ok(vec![ (Label, ident) ]);
            }

            // Search by prefix gave a wrong result
            _ => bug!("Unexpected prefix")
        }
    }

    //Did not match any prefix.

    // Macro identifier ?
    if word.ends_with('.') {
        let macro_ident = word.get(..word.len() - 1).ok_or(BadMacroIdent)?;
        let mut result = vec![];

        // In macro calls, the identifier can come with a repeat count.
        // e.g. '16ident.'
        let mut dec_i = 0;
        for (i, ident_c) in macro_ident.chars().rev().enumerate() {
            if text::is_char_num(ident_c) {
                dec_i = macro_ident.len() - i;
                let lit = macro_ident.get(0 .. dec_i).ok_or(BadDec)?;
                let dec = text::check_dec(lit).ok_or(BadDec)?;

                result.push((Repeat, dec));
            }
        }

        // Split after repeat count.
        let name = macro_ident.get(dec_i..).ok_or(BadMacroIdent)?;
        let ident = text::check_ident(name).ok_or(BadMacroIdent)?;

        result.push((MacroIdent, ident));
        return Ok(result);
    }

    // Identifier ?
    if text::is_char_ident_first(c) {
        let ident = text::check_ident(word).ok_or(BadIdent)?;
        return Ok(vec![ (Identifier, ident) ]);
    }

    // Decimal literal ?
    if text::is_char_num(c) {
        let dec = text::check_dec(word).ok_or(BadDec)?;
        return Ok(vec![ (LitDec, dec) ]);
    }

    // Could not parse word.
    Err(ParseMsg::Invalid)
}

//TODO check where validation happened, it is here now.
/// Fits a number into a `Value` based on the literal's length (&00FF would be a u16).
fn fit(num: usize, len: usize, base: usize) -> Result<Value<'static>, ParseMsg> {
    return match base {
        2 => if len > 16 {
            Err(ParseMsg::BadBin) 
        }else if len > 8 {
            Ok(Value::U16(num as u16))
        }else {
            Ok(Value::U8(num as u8))
        }

        10 => if len > 5 || num > u16::MAX.into() {
            Err(ParseMsg::BadDec) 
        }else if len > 3 || num > u8::MAX.into() {
            Ok(Value::U16(num as u16))
        }else {
            Ok(Value::U8(num as u8))
        }

        16 => if len > 4 {
            Err(ParseMsg::BadHex) 
        }else if len > 2 {
            Ok(Value::U16(num as u16))
        }else {
            Ok(Value::U8(num as u8))
        }

        _ => bug!("Bad base."),
    };
}
