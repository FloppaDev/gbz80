
use crate::{
    parse::{
        text::{CheckedStr, charset},
        lex::{self, TokenType::{self, *}},
        data::{Data, Key},
        split::Split,
    },
    program::error::{ ErrCtx, ParseErr, ParseErrType::{self, *} },
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
    data: &mut Data<'a>,
    split: &Split<'a>,
) -> Result<Vec<ParsedToken<'a>>, Vec<ParseErr<'a>>> {
    let mut parsed_tokens = vec![];
    let mut errors = vec![];

    for word in split.words() {
        let id_words = identify(word.value);

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
            let values = extract(data, (ty, word_str.as_str()));

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
    data: &mut Data<'a>,
    word: (TokenType, &'a str)
) -> Result<(TokenType, Key), ParseErrType> {
    let (ty, str_value) = word; 

    if !lex::has_value(ty) {
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

            return Ok((ty, data.push_usize(hex)));
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

            return Ok((ty, data.push_usize(bin)));
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

            //TODO check for overflows in validation.
            return Ok((ty, data.push_usize(dec)));
        }

        // Value for those is str_value
        LitStr|Identifier|Label|NamedMark|AnonMark|MacroArg|MacroIdent => {
            Ok((ty, data.push_str(str_value)))
        }

        _ => Err(ParseErrType::UnhandledType)
    }
}

//TODO use try '?'
/// Get token type(s) and value(s) from word.
fn identify<'a>(
    word: &'a str
) -> Result<Vec<(TokenType, CheckedStr<'a>)>, ParseErrType> {
    if word.is_empty() {
        return Err(ParseErrType::EmptyStr);
    }

    // Find token type by name.
    // Works with registers and instruction names.
    if let Some(ty) = lex::get_by_word(word) {
        return Ok(vec![ (ty, charset::no_check("")) ]);
    }

    let c = word.get(0..1).ok_or(Invalid)?.chars().next().unwrap();
    let last = word.chars().last().unwrap();

    // Find token type by prefix.
    if lex::has_prefix(c) {
        match c {
            '&' => {
                // &6762:
                if last == ':' {
                    let lit = word.get(1..word.len() - 1).ok_or(InvalidAnonMark)?;
                    let hex = charset::check_hex(lit).ok_or(InvalidAnonMarkHex)?;
                    return Ok(vec![ (AnonMark, charset::no_check("")), (LitHex, hex) ]);
                    //TODO put hex in AnonMark?
                }

                // &2763:label
                if let Some(sep) = word.find(':') {
                    let lit = word.get(1..sep).ok_or(InvalidNamedMark)?;
                    let hex = charset::check_hex(lit).ok_or(InvalidNamedMarkHex)?;

                    let label = word.get(sep + 1 ..).ok_or(InvalidNamedMarkLabel)?;
                    let ident = charset::check_ident(label).ok_or(InvalidNamedMarkLabelIdent)?;

                    return Ok(vec![ (NamedMark, ident), (LitHex, hex) ]);
                    //TODO put hex in AnonMark?
                }

                // &2787
                let lit = word.get(1..).ok_or(InvalidHex)?;
                let hex = charset::check_hex(lit).ok_or(InvalidHex)?;
                return Ok(vec![ (LitHex, hex) ]);
            }

            // 0101_0101 or 11010
            '%' => {
                let lit = word.get(1..).ok_or(InvalidBin)?;
                let bin = charset::check_bin(lit).ok_or(InvalidBin)?;
                return Ok(vec![ (LitBin, bin) ]);
            }

            // "...
            '"' => {
                let value = word.get(1..).ok_or(InvalidStr)?;
                return Ok(vec![ (LitStr, charset::no_check(value)) ]);
            }

            // "#def or "include or #macro
            '#' => {
                let directive = word.get(1..).ok_or(InvalidDirective)?;

                return match directive {
                    "def" => Ok(vec![ (Define, charset::no_check("")) ]),
                    "include" => Ok(vec![ (Include, charset::no_check("")) ]),
                    "macro" => Ok(vec![ (Macro, charset::no_check("")) ]),
                    _ => Err(ParseErrType::InvalidDirectiveIdent)
                };
            }

            // .arg
            '.' => {
                let arg = word.get(1..).ok_or(InvalidMacroArg)?;
                let ident = charset::check_ident(arg).ok_or(InvalidMacroArgIdent)?;
                return Ok(vec![ (MacroArg, ident) ]);
            }

            ':' => {
                let label = word.get(1..).ok_or(InvalidLabel)?;
                let ident = charset::check_ident(label).ok_or(InvalidLabelIdent)?;
                return Ok(vec![ (Label, ident) ]);
            }

            // Search by prefix gave a wrong result
            _ => return Err(ParseErrType::UnexpectedPrefix),
        }
    }

    //Did not match any prefix.

    // Macro identifier ?
    if word.ends_with('.') {
        let macro_ident = word.get(..word.len() - 1).ok_or(InvalidMacroIdent)?;
        let mut result = vec![];

        // In macro calls, the identifier can come with a repeat count.
        // e.g. '16ident.'
        let mut dec_i = 0;
        for (i, ident_c) in macro_ident.chars().rev().enumerate() {
            if charset::is_char_num(ident_c) {
                dec_i = macro_ident.len() - i;
                let lit = macro_ident.get(0 .. dec_i).ok_or(InvalidDec)?;
                let dec = charset::check_dec(lit).ok_or(InvalidDec)?;

                result.push((Repeat, dec));
            }
        }

        // Split after repeat count.
        let name = macro_ident.get(dec_i..).ok_or(InvalidMacroIdent)?;
        let ident = charset::check_ident(name).ok_or(InvalidMacroIdent)?;

        result.push((MacroIdent, ident));
        return Ok(result);
    }

    // Identifier ?
    if charset::is_char_ident_first(c) {
        let ident = charset::check_ident(word).ok_or(InvalidIdent)?;
        return Ok(vec![ (Identifier, ident) ]);
    }

    // Decimal literal ?
    if charset::is_char_num(c) {
        let dec = charset::check_dec(word).ok_or(InvalidDec)?;
        return Ok(vec![ (LitDec, dec) ]);
    }

    // Could not parse word.
    Err(ParseErrType::Invalid)
}