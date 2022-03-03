
use crate::{
    text::{ CheckedStr, charset },
    lingo::{ TokenType::{self, *}, Lexicon },
    error::{ ErrCtx, ParseErr, ParseErrType },
    data::{ Data, Key },
    split::Split,
};

/// Output of the parser. Contains the type and the key to the data.
pub struct ParsedToken<'a> {
    pub ty: TokenType,
    pub data_key: Key,
    pub line_number: usize,
    pub line: &'a str,
    pub word: &'a str,
}

/// Map words to token types and extract their data.
pub fn parse<'a>(
    lexicon: &Lexicon,
    data: &mut Data<'a>,
    split: &Split<'a>,
) -> Result<Vec<ParsedToken<'a>>, Vec<ParseErr<'a>>> {
    let mut parsed_tokens = vec![];
    let mut errors = vec![];

    for word in split.words() {
        let id_words = identify(lexicon, word.value);

        // Error while identifying token type.
        if let Err(err_type) = id_words {
            let err_ctx = ErrCtx::new(
                split.line_number(word.line_index),
                split.line(word.line_index),
                word.value);

            let err = ParseErr::new(err_type, err_ctx);
            errors.push(err);

            continue;
        }

        // Extract data for all words and collect errors.
        for (ty, word_str) in id_words.unwrap() {
            let values = extract(lexicon, data, (ty, word_str.as_str()));

            if let Err(err_type) = values {
                let err_ctx = ErrCtx::new(
                    split.line_number(word.line_index), 
                    split.line(word.line_index), 
                    word_str.as_str());

                let err = ParseErr::new(err_type, err_ctx);
                errors.push(err);

                continue;
            }

            let (ty, data_key) = values.unwrap();
            let parsed_token = ParsedToken { 
                ty, 
                data_key, 
                line_number: split.line_number(word.line_index),
                line: split.line(word.line_index), 
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
fn extract<'a>(
    lexicon: &Lexicon,
    data: &mut Data<'a>,
    word: (TokenType, &'a str)
) -> Result<(TokenType, Key), ParseErrType> {
    let (ty, str_value) = word; 

    if lexicon.no_value(ty) {
        //TODO don't remember what it does and why.
        //println!("{}", str_value);
        //  prints nothing. Use Key::Void instead?
        let data_key = data.push_str(str_value);
        return Ok((ty, data_key)) 
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

            if let Some(data_key) = data.push_num(hex) {
                return Ok((ty, data_key));
            }

            Err(ParseErrType::HexOverflow)
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

            if let Some(data_key) = data.push_num(bin) {
                return Ok((ty, data_key));
            }

            Err(ParseErrType::BinOverflow)
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

            if let Some(data_key) = data.push_num(dec) {
                return Ok((ty, data_key));
            }

            Err(ParseErrType::DecOverflow)
        }

        // Value for those is str_value
        LitStr|Identifier|Label|NamedMark|AnonMark|MacroArg|MacroIdent => {
            Ok((ty, data.push_str(str_value)))
        }

        _ => Err(ParseErrType::UnhandledType)
    }
}

/// Get token type(s) and value(s) from word.
fn identify<'a>(
    lexicon: &Lexicon,
    word: &'a str
) -> Result<Vec<(TokenType, CheckedStr<'a>)>, ParseErrType> {
    if word.is_empty() {
        return Err(ParseErrType::EmptyStr);
    }

    // Find token type by name.
    // Works with registers and instruction names.
    if let Some(ty) = lexicon.get_by_name(word) {
        return Ok(vec![ (ty, charset::no_check("")) ]);
    }

    let ch = word.get(0..1);

    if ch.is_none() {
        return Err(ParseErrType::Invalid);
    }

    let ch = ch.unwrap();
    let c = ch.chars().next().unwrap();
    let last = word.chars().last().unwrap();

    // Find token type by prefix.
    if let Some(ty) = lexicon.get_by_prefix(ch) {
        match ty {
            LitHex => {
                // &6762:
                if last == ':' {
                    let lit = word.get(1..word.len() - 1);
                    if lit.is_none() {
                        return Err(ParseErrType::InvalidAnonMark);
                    }

                    let lit = charset::check_hex(lit.unwrap());
                    if lit.is_none() {
                        return Err(ParseErrType::InvalidAnonMarkHex);
                    }

                    return Ok(vec![
                        (AnonMark, charset::no_check("")),
                        (LitHex, lit.unwrap()),
                    ]);
                }

                // &2763:label
                if let Some(sep) = word.find(':') {
                    let lit = word.get(1..sep);
                    if lit.is_none() {
                        return Err(ParseErrType::InvalidNamedMark);
                    }

                    let lit = charset::check_hex(lit.unwrap());
                    if lit.is_none() {
                        return Err(ParseErrType::InvalidNamedMarkHex);
                    }

                    let label = word.get(sep + 1 ..);
                    if label.is_none() {
                        return Err(ParseErrType::InvalidNamedMarkLabel);
                    }

                    let label = charset::check_ident(label.unwrap());
                    if label.is_none() {
                        return Err(ParseErrType::InvalidNamedMarkLabelIdent);
                    }

                    return Ok(vec![
                        (NamedMark, label.unwrap()),
                        (LitHex, lit.unwrap()),
                    ]);
                }

                // &2787
                let wend = word.get(1..);
                if wend.is_none() {
                    return Err(ParseErrType::InvalidHex);
                }

                let lit = charset::check_hex(wend.unwrap());
                if lit.is_none() {
                    return Err(ParseErrType::InvalidHex);
                }

                return Ok(vec![ (LitHex, lit.unwrap()) ]);
            }

            // 0101_0101 or 11010
            LitBin => {
                let wend = word.get(1..);
                if wend.is_none() {
                    return Err(ParseErrType::InvalidBin);
                }

                let lit = charset::check_bin(wend.unwrap());
                if lit.is_none() {
                    return Err(ParseErrType::InvalidBin);
                }

                return Ok(vec![ (LitBin, lit.unwrap()) ]);
            }

            // "...
            LitStr => {
                if let Some(value) = word.get(1..) {
                    return Ok(vec![ (LitStr, charset::no_check(value)) ]);
                }
                
                return Err(ParseErrType::InvalidStr);
            }

            // "#def or "include or #macro
            Directive => {
                let wend = word.get(1..);
                if wend.is_none() {
                    return Err(ParseErrType::InvalidDirective);
                }

                return match wend.unwrap() {
                    "def" => Ok(vec![ (Define, charset::no_check("")) ]),
                    "include" => Ok(vec![ (Include, charset::no_check("")) ]),
                    "macro" => Ok(vec![ (Macro, charset::no_check("")) ]),
                    _ => Err(ParseErrType::InvalidDirectiveIdent)
                };
            }

            // .arg
            MacroArg => {
                let ident = word.get(1..);
                if ident.is_none() {
                    return Err(ParseErrType::InvalidMacroArg);
                }

                let ident = charset::check_ident(ident.unwrap());
                if ident.is_none() {
                    return Err(ParseErrType::InvalidMacroArgIdent);
                }

                return Ok(vec![ (MacroArg, ident.unwrap()) ]);
            }

            Label => {
                let ident = word.get(1..);
                if ident.is_none() {
                    return Err(ParseErrType::InvalidLabel);
                }

                let ident = charset::check_ident(ident.unwrap());
                if ident.is_none() {
                    return Err(ParseErrType::InvalidLabelIdent);
                }

                return Ok(vec![ (ty, ident.unwrap()) ]);
            }

            // Search by prefix gave a wrong result
            _ => return Err(ParseErrType::UnexpectedPrefix),
        }
    }

    //Did not match any prefix.

    // Macro identifier ?
    if word.ends_with('.') {
        let ident = word.get(..word.len() - 1);
        if ident.is_none() {
            return Err(ParseErrType::InvalidMacroIdent);
        }
        let ident = ident.unwrap();

        let mut result = vec![];

        // In macro calls, the identifier can come with a repeat count.
        // e.g. '16ident.'
        let mut dec_i = 0;
        for (i, ident_c) in ident.chars().rev().enumerate() {
            if charset::is_char_num(ident_c) {
                dec_i = ident.len() - i;
                let dec = ident.get(0 .. dec_i);

                if dec.is_none() {
                    return Err(ParseErrType::InvalidDec);
                }

                let dec = charset::check_dec(dec.unwrap());
                if dec.is_none() {
                    return Err(ParseErrType::InvalidDec);
                }

                result.push((Repeat, dec.unwrap()));
            }
        }

        // Split after repeat count.
        let ident = ident.get(dec_i..);
        if ident.is_none() {
            return Err(ParseErrType::InvalidMacroIdent);
        }

        let ident = charset::check_ident(ident.unwrap());
        if ident.is_none() {
            return Err(ParseErrType::InvalidMacroIdent);
        }

        result.push((MacroIdent, ident.unwrap()));
        return Ok(result);
    }

    // Identifier ?
    if charset::is_char_ident_first(c) {
        if let Some(ident) = charset::check_ident(word) {
            return Ok(vec![ (Identifier, ident) ]);
        }

        return Err(ParseErrType::InvalidIdent);
    }

    // Decimal literal ?
    if charset::is_char_num(c) {
        if let Some(dec) = charset::check_dec(word) {
            return Ok(vec![ (LitDec, dec) ]);
        }

        return Err(ParseErrType::InvalidDec);
    }

    // Could not parse word.
    Err(ParseErrType::Invalid)
}
