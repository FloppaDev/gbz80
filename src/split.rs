fn is_new_line(c: &char) -> bool {
    const CHARS: [char; 2] = ['\u{000A}', '\u{000D}'];
    CHARS.contains(c)
}

fn is_space(c: &char) -> bool {
    const CHARS: [char; 29] = [
        '\u{0009}', '\u{000B}', '\u{000C}', '\u{0020}',
        '\u{0085}', '\u{00A0}', '\u{1680}', '\u{180E}',
        '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}',
        '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}',
        '\u{2008}', '\u{2009}', '\u{200A}', '\u{200B}',
        '\u{200C}', '\u{200D}', '\u{2028}', '\u{2029}',
        '\u{202F}', '\u{205F}', '\u{2060}', '\u{3000}',
        '\u{FEFF}',
    ];
    CHARS.contains(c)
}

fn is_ident_char(c: &char) -> bool {
    const CHARS: [char; 63] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',

        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',

        '_', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0',
    ];
    CHARS.contains(c)
}

fn is_ident_first(c: &char) -> bool {
    const CHARS: [char; 53] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',

        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',

        '_',
    ];
    CHARS.contains(c)
}

/// Split input into lines and words.
fn get_lines(
    input: &str, symbols: &Vec<String>
) -> Vec<(usize, Vec<Range<usize>>)> {
    let mut lines = vec![];

    let mut has_word = false;
    let mut comment = false;
    let mut str_literal = false;
    let mut dir_if = false;
    let mut dir_else = false;
    let mut has_symbol = false;
    let mut start = 0;
    let mut cur_line = 0;
    let mut source_line = 1;

    macro_rules! prepare_line {()=>{ 
        prepare_line(&mut lines, &mut cur_line, source_line);
    }};

    for (i, c) in input.chars().enumerate() {
        macro_rules! add_word {()=>{
            add_word( input, &mut lines, &mut cur_line, source_line,
                &mut has_word, &mut has_symbol, start, i,
                &mut dir_if, &mut dir_else, symbols);
        }};

        if is_new_line(&c) {
            add_word!();
            cur_line += 1;
            comment = false;
            source_line += 1;
        }else {
            if comment { continue }
            
            if c == '"' {
                str_literal = !str_literal;
                //TODO escape \" and newline and \;
                if str_literal { 
                    add_word!();
                    start = i;
                }else { 
                    prepare_line!();
                    lines[cur_line].1.push(start..i+1);
                    continue
                }
            }

            if str_literal { continue }

            if is_space(&c) {
                add_word!();
                continue
            }

            match c {
                ';' => {
                    add_word!();
                    comment = true;
                }
                '+' | '-' | '(' | ')' => {
                    prepare_line!();
                    // Push the previous word
                    if has_word {
                        lines[cur_line].1.push(start..i);
                        has_word = false;
                    }
                    // Push the character
                    lines[cur_line].1.push(i..i+1);
                }
                _ => {
                    if !has_word { start = i; }
                    has_word = true;
                }
            }
        }
    }

    lines
}

/// Create a new line if necessary
fn prepare_line(
    lines: &mut Vec<(usize, Vec<Range<usize>>)>,
    cur_line: &mut usize,
    source_line: usize,
) {
    if lines.len() < *cur_line+1 {
        lines.push((source_line, vec![])); 
        *cur_line = lines.len()-1;
    }
}

/// Add word to lines
fn add_word(
    input: &str,
    mut lines: &mut Vec<(usize, Vec<Range<usize>>)>,
    mut cur_line: &mut usize,
    source_line: usize,
    has_word: &mut bool,
    has_symbol: &mut bool,
    start: usize,
    i: usize,
    dir_if: &mut bool,
    dir_else: &mut bool,
    symbols: &Vec<String>,
) {
    if *has_word {
        let word = input.get(start..i).unwrap();

        if *dir_if { *has_symbol = symbols.contains(&word.to_string()); }

        match word {
            "#if" => *dir_if = true,
            "#else" => *dir_else = true,
            "endif" => {
                *dir_if = false;
                *dir_else = false;
            }
            _ => {
                if !*dir_if || (*dir_if && *has_symbol) || (*dir_else && !*has_symbol) { 
                    prepare_line(&mut lines, &mut cur_line, source_line);
                    lines[*cur_line].1.push(start..i);
                    *has_word = false;
                }
            }
        }
    }
}
